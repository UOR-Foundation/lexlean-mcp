//! IM-01 to IM-06: the images of the services, the next slice of issue #17.
//!
//! What only a running image can show is in `deploy/images/run.sh`, which the gateway job runs and
//! holds to its count. These tests read the definitions as text, so that `just vv`, which has no
//! programs to put in an image, still refuses a base named by a tag, a step that runs something, or
//! a toolchain that is not the one the gateway is pinned to.

use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const IMAGES: [(&str, &[&str]); 5] = [
    ("gateway", &["gateway-host", "gateway-ready"]),
    ("engine-host", &["gateway-engine-host"]),
    ("signer", &["gateway-signer"]),
    ("verifier", &["gateway-verifier-host", "gateway-verifier"]),
    ("collector", &[]),
];

fn definition(name: &str) -> String {
    read(&format!("deploy/images/{name}.Containerfile"))
}

/// The instructions of a definition, without its comments.
fn instructions(name: &str) -> Vec<String> {
    definition(name)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

#[test]
fn each_image_is_pinned_runs_nothing_and_is_not_root_im_01() {
    let definitions = std::fs::read_dir(root().join("deploy/images"))
        .expect("deploy/images")
        .map(|entry| entry.expect("an entry").file_name())
        .filter_map(|name| {
            name.to_str()?
                .strip_suffix(".Containerfile")
                .map(str::to_string)
        })
        .count();
    assert_eq!(definitions, IMAGES.len(), "a definition for each image");
    for (name, programs) in IMAGES {
        let lines = instructions(name);
        let bases: Vec<&String> = lines.iter().filter(|l| l.starts_with("FROM ")).collect();
        assert_eq!(bases.len(), 1, "{name}: one base");
        let (_, digest) = bases[0]
            .split_once("@sha256:")
            .unwrap_or_else(|| panic!("{name}: the base is not named by a digest: {}", bases[0]));
        assert!(
            digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()),
            "{name}: the digest of the base: {digest}"
        );
        for line in &lines {
            assert!(!line.starts_with("RUN"), "{name} has a RUN step: {line}");
            assert!(!line.starts_with("ADD"), "{name} has an ADD step: {line}");
        }
        assert_eq!(
            lines
                .iter()
                .rfind(|l| l.starts_with("USER "))
                .map(String::as_str),
            Some("USER 65532:65532"),
            "{name}: the user it runs as"
        );
        for program in programs {
            assert!(
                lines
                    .iter()
                    .any(|l| l.starts_with("COPY") && l.contains(program)),
                "{name} does not copy {program}"
            );
        }
        for (other, others) in IMAGES {
            if other == name {
                continue;
            }
            for program in others.iter().filter(|p| !programs.contains(p)) {
                assert!(
                    !lines
                        .iter()
                        .any(|l| l.split_whitespace().any(|word| word == *program)),
                    "{name} names the program of {other}: {program}"
                );
            }
        }
    }
}

#[test]
fn the_verifier_alone_holds_the_pinned_lean_im_02() {
    let pin = read("gateway/lean-toolchain");
    let version = pin
        .trim()
        .strip_prefix("leanprover/lean4:v")
        .expect("gateway/lean-toolchain names a release");
    let recorded = read("deploy/images/lean-toolchain.sha256");
    let (digest, archive) = recorded
        .trim()
        .split_once("  ")
        .expect("a line of sha256sum");
    assert!(
        digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_hexdigit()),
        "the recorded digest: {digest}"
    );
    assert_eq!(
        archive,
        format!("lean-{version}-linux.tar.zst"),
        "the archive recorded is not of the version gateway/lean-toolchain names"
    );
    let build = read("deploy/images/build.sh");
    assert!(
        build.contains("sha256sum --check --strict"),
        "the build does not check the archive"
    );
    let mangled = format!("/opt/elan/toolchains/leanprover--lean4---v{version}");
    let verifier = instructions("verifier");
    assert!(
        verifier
            .iter()
            .any(|l| l.starts_with("COPY toolchain") && l.ends_with(&mangled)),
        "the verifier image does not hold the toolchain at {mangled}"
    );
    assert!(
        verifier.iter().any(|l| l == "ENV ELAN_HOME=/opt/elan"),
        "LexLean finds the toolchain through ELAN_HOME"
    );
    for (name, _) in IMAGES.iter().filter(|(name, _)| *name != "verifier") {
        assert!(
            !definition(name).contains("toolchain"),
            "{name} holds a toolchain"
        );
    }
}

/// The lines of `deploy/images/run.sh` that start the service `name`, joined.
fn started(name: &str) -> String {
    let source = read("deploy/images/run.sh");
    let head = format!("service {name} {name} ");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("run.sh starts no {name}"));
    let rest = &source[start..];
    let mut end = 0;
    for line in rest.lines() {
        end += line.len() + 1;
        if !line.ends_with('\\') {
            break;
        }
    }
    rest[..end].to_string()
}

#[test]
fn the_services_run_apart_and_only_the_signer_has_the_key_im_03() {
    let source = read("deploy/images/run.sh");
    let function = source
        .split("service() {")
        .nth(1)
        .expect("run.sh has the function that starts a service")
        .split("\n}\n")
        .next()
        .expect("its end");
    for option in [
        "--network none",
        "--read-only",
        "--cap-drop ALL",
        "--security-opt no-new-privileges",
        "--init",
    ] {
        assert!(function.contains(option), "a service runs without {option}");
    }
    for name in ["signer", "engine-host", "verifier", "gateway", "collector"] {
        let line = started(name);
        for widening in [
            "--network",
            "--privileged",
            "--cap-add",
            "--user",
            "--publish",
        ] {
            assert!(
                !line.contains(widening),
                "{name} is started with {widening}"
            );
        }
        assert_eq!(
            line.contains("/run/signing") || line.contains("SIGNING_KEY"),
            name == "signer",
            "{name}: the key is mounted in the signer alone"
        );
        assert_eq!(
            line.contains("/sockets/signer"),
            name == "gateway" || name == "signer",
            "{name}: the socket of the signer"
        );
        assert_eq!(
            line.contains("/sockets/engine"),
            name == "gateway" || name == "engine-host",
            "{name}: the socket of the engine host"
        );
        assert_eq!(
            line.contains("/sockets/verifier"),
            name == "gateway" || name == "verifier",
            "{name}: the socket of the verifier host"
        );
    }
    assert!(
        !definition("signer").contains("KEY") && !read("deploy/images/build.sh").contains("key"),
        "the key is a layer of an image"
    );
}

#[test]
fn the_collector_receives_what_the_gateway_exports_im_04() {
    let configuration = read("deploy/images/collector.yaml");
    for word in [
        "otlpjsonfile",
        "/telemetry/otlp.jsonl",
        "/telemetry/received.jsonl",
    ] {
        assert!(configuration.contains(word), "collector.yaml has no {word}");
    }
    let source = read("deploy/images/run.sh");
    assert!(
        started("gateway").contains("LEXLEAN_MCP_TELEMETRY=/telemetry/otlp.jsonl"),
        "the gateway does not write the file the collector reads"
    );
    for check in [
        "check \"the alert of the signer reaches the collector\"",
        "check \"no module text reaches the collector\"",
    ] {
        assert!(source.contains(check), "run.sh has no {check}");
    }
}

#[test]
fn a_service_that_stops_is_a_failure_and_the_run_is_held_to_its_count_im_05() {
    let source = read("deploy/images/run.sh");
    let exercise = read("deploy/images/exercise.py");
    for check in [
        "check \"a task whose verifier host stops ends as verifier-lost\"",
        "check \"with the verifier host stopped, verify is refused\"",
        "check \"with the engine host stopped, an Engine call is refused\"",
        "check \"with the signer stopped, verify is refused\"",
    ] {
        assert!(source.contains(check), "run.sh has no {check}");
    }
    assert!(
        source.contains("== verifier-lost"),
        "run.sh does not hold a lost task to verifier-lost"
    );
    for failure in ["engine-unavailable", "verifier-unavailable"] {
        assert!(
            exercise.contains(failure),
            "exercise.py does not expect {failure}"
        );
    }
    let lib = read(".github/lexlean-ci/lib.sh");
    let gate = lib
        .split("images_run() {")
        .nth(1)
        .expect("lib.sh has images_run")
        .split("\n}\n")
        .next()
        .expect("its end");
    assert!(
        gate.contains("grep -c '^check \"' deploy/images/run.sh"),
        "images_run does not count the checks written in run.sh"
    );
    assert!(
        gate.contains("checks passed"),
        "images_run does not read the count the run printed"
    );
    let workflow = read(".github/workflows/lexlean.yml");
    for step in ["images_programs", "images_build", "images_run"] {
        assert!(workflow.contains(step), "the workflow does not run {step}");
    }
    assert!(
        lib.contains("'^deploy/images/'"),
        "a change to the images does not reach the job that runs them"
    );
}

#[test]
fn each_gate_of_the_images_rejects_its_planted_defect_im_06() {
    let document = read("IMAGES.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("IMAGES.md has the section")
        .split("\n## ")
        .next()
        .expect("the section");
    let rows = section
        .lines()
        .filter(|line| {
            line.starts_with("| ") && !line.starts_with("| Defect") && !line.starts_with("| ---")
        })
        .count();
    assert_eq!(rows, 12, "the planted defects of IM-06");
    for id in ["IM-01", "IM-02", "IM-03", "IM-04", "IM-05"] {
        assert!(document.contains(id), "IMAGES.md does not name {id}");
    }
}
