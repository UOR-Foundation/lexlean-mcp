//! SG-09 to SG-12: the signed record of a verification.
//!
//! The verification itself needs the Lean toolchain and is run by the ignored test of
//! `effects.rs`. What is tested here is what the host does with its result: `conclude` takes the
//! facts of a verification that ended `verified` and a directory that holds its
//! `attestation.json`, and either signs and records it or leaves the store as it was. The
//! envelopes are validated by `signer::validate`, whose own tests (`signing.rs`) make the
//! expected bytes independently.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use gateway_binding::signer::{self, Signer};
use gateway_binding::signer_client::SignerClient;
use gateway_binding::{conclude, Config, Facts, Job, Native, State};
use gateway_host_api::protocol::{
    attestation_lookup, verify_control, verify_start, Host, HostError,
};
use lexlean::Selection;
use serde_json::Value;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
mod common;

const TINY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors/fixture/tiny");
const SEED: [u8; 32] = [0x33; 32];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// How a test makes a configuration below a directory.
type Make = Box<dyn Fn(&Path) -> Config>;

fn id() -> String {
    "ab".repeat(32)
}

fn failure<F>(error: HostError<F>) -> F {
    match error {
        HostError::Failed(failure) => failure,
        HostError::Fault => panic!("the host faulted"),
    }
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from)
        .expect("a fixture directory")
        .flatten()
    {
        let name = entry.file_name();
        if name == ".lexlean" {
            continue;
        }
        if entry.file_type().expect("a type").is_dir() {
            copy(&entry.path(), &to.join(&name));
        } else {
            std::fs::copy(entry.path(), to.join(&name)).expect("a copy");
        }
    }
}

fn key_file(dir: &Path) -> PathBuf {
    let path = dir.join("signing.key");
    std::fs::write(&path, format!("{}\n", hex(&SEED))).expect("a key file");
    path
}

/// A configuration whose store is below `dir` and whose signer has a key file of `dir`.
fn config(dir: &Path) -> Config {
    Config::at(dir.join("store"))
        .with_signer(common::signer(dir, &key_file(dir)))
        .with_verifier(common::real_verifier())
}

/// The canonical LaTeX of the module of the tiny project, as a verification renders it.
const RENDERING: &[u8] =
    b"\\documentclass[11pt]{article}\n\\begin{document}\n\\section*{Semantic declarations}\n\\end{document}\n";

/// Where a verification writes the rendering of the module `Tiny`.
const RENDERING_PATH: &str = "modules/Tiny/Tiny.tex";

fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// A build manifest that names the source of the tiny project and `rendering` as the canonical
/// LaTeX of its module.
fn manifest_naming(rendering: &[u8]) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "inputs": [{"kind": "source", "path": "src/Tiny.lex.tex", "sha256": tiny_digest()}],
        "outputs": [{"kind": "tex", "path": RENDERING_PATH, "sha256": sha(rendering)}],
    }))
    .expect("JSON")
}

fn manifest() -> Vec<u8> {
    manifest_naming(RENDERING)
}

/// The attestation.json of a verified directory whose build manifest is `manifest`, as bytes
/// the signer must not alter.
fn attestation_json_naming(manifest: &[u8]) -> Vec<u8> {
    format!(
        "{{\"attestation_id\":\"{}\",\"build_manifest\":{{\"byte_length\":{},\"sha256\":\"{}\"}},\"toolchain\":{{\"lean\":\"4.32.1\"}}}}",
        id(),
        manifest.len(),
        sha(manifest)
    )
    .into_bytes()
}

fn attestation_json() -> Vec<u8> {
    attestation_json_naming(&manifest())
}

/// A verified directory below `dir` whose attestation.json names `manifest`, and which holds
/// `manifest` and the `files` given, at their paths.
fn verified_with(dir: &Path, manifest: &[u8], files: &[(&str, &[u8])]) -> PathBuf {
    let root = dir.join("verified");
    std::fs::create_dir_all(&root).expect("a directory");
    std::fs::write(
        root.join("attestation.json"),
        attestation_json_naming(manifest),
    )
    .expect("a file");
    std::fs::write(root.join("build-manifest.json"), manifest).expect("a file");
    for (path, bytes) in files {
        let at = root.join(path);
        std::fs::create_dir_all(at.parent().expect("a parent")).expect("a directory");
        std::fs::write(at, bytes).expect("a file");
    }
    root
}

/// A verified directory that holds an attestation.json, the build manifest it names and the
/// rendering that the manifest names.
fn verified_root(dir: &Path) -> PathBuf {
    verified_with(dir, &manifest(), &[(RENDERING_PATH, RENDERING)])
}

fn facts(root: PathBuf) -> Facts {
    Facts {
        attestation_id: id(),
        build_id: "bb".repeat(32),
        source_id: "cc".repeat(32),
        semantic_id: "dd".repeat(32),
        modules: vec!["Tiny".to_owned()],
        root,
    }
}

/// A job on a copy of the tiny project, with the signer and the history of `config`.
fn job(dir: &Path, config: &Config) -> Job {
    let project = dir.join("project");
    copy(Path::new(TINY), &project);
    Job {
        directory: project,
        selection: Selection::Entrypoints,
        store: config.store.clone(),
        signer: Arc::new(Signer::from_seed(SEED, config.history_path())),
    }
}

fn tiny_digest() -> String {
    hex(&Sha256::digest(
        std::fs::read(Path::new(TINY).join("src/Tiny.lex.tex")).expect("a source"),
    ))
}

/// Every file below `directory`, relative to it; empty when it does not exist.
fn files_below(directory: &Path) -> Vec<String> {
    fn walk(base: &Path, at: &Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, found);
            } else {
                found.push(
                    path.strip_prefix(base)
                        .expect("below")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let mut found = Vec::new();
    walk(directory, directory, &mut found);
    found.sort();
    found
}

fn nothing_recorded(store: &Path) {
    for area in ["verifications", "index", "keys"] {
        assert_eq!(
            files_below(&store.join(area)),
            Vec::<String>::new(),
            "the {area} area holds something"
        );
    }
}

// SG-09

#[test]
fn the_variable_names_the_socket_and_an_empty_one_is_no_path_sg_09() {
    let with = |value: Option<&str>| {
        Config::from_variables(|name| match name {
            "LEXLEAN_MCP_SIGNER" => value.map(OsString::from),
            "LEXLEAN_MCP_STORE" => Some(OsString::from("/data/store")),
            _ => None,
        })
    };
    assert_eq!(
        with(Some("/run/signer/socket")).signer,
        Some(PathBuf::from("/run/signer/socket"))
    );
    assert_eq!(with(Some("")).signer, None, "an empty value is no path");
    assert_eq!(with(None).signer, None);
    let configured = with(Some("/run/signer/socket"));
    assert_eq!(configured.store, PathBuf::from("/data/store"));
    assert_eq!(
        configured.history_path(),
        PathBuf::from("/data/store/keys/history"),
        "the history is in the keys area of the store"
    );
    assert_eq!(
        Config::at("/s").signer,
        None,
        "no signer unless one is named"
    );
}

// SG-16

#[test]
fn a_key_named_to_the_host_signs_nothing_sg_16() {
    let directory = tempfile::tempdir().expect("a directory");
    let key = key_file(directory.path());
    let store = directory.path().join("store");
    let config = Config::from_variables(|name| match name {
        "LEXLEAN_MCP_SIGNING_KEY" => Some(key.clone().into_os_string()),
        "LEXLEAN_MCP_STORE" => Some(store.clone().into_os_string()),
        _ => None,
    });
    assert_eq!(config.signer, None, "the key names no signer");
    let program = common::fake_verifier(directory.path());
    let mut host = Native::new(config.with_verifier(program));
    copy(
        Path::new(TINY),
        &host.config().store.join("drafts").join("tiny"),
    );
    let refused = host
        .verify_start(verify_start::Request {
            project: "tiny".into(),
            selection: 0,
            files: String::new(),
        })
        .map(|done| done.task)
        .map_err(failure);
    assert_eq!(refused, Err(verify_start::Failure::VerifierUnavailable));
    assert!(
        !host.config().history_path().exists(),
        "the host used no key"
    );
}

#[test]
fn verify_start_without_a_usable_key_is_unavailable_and_starts_nothing_sg_09() {
    let unusable: [(&str, Make); 5] = [
        ("no signer", Box::new(|dir| Config::at(dir.join("store")))),
        (
            "a socket nothing answers",
            Box::new(|dir| Config::at(dir.join("store")).with_signer(dir.join("absent.sock"))),
        ),
        (
            "a missing file",
            Box::new(|dir| {
                let socket = common::signer(dir, &dir.join("absent"));
                Config::at(dir.join("store")).with_signer(socket)
            }),
        ),
        (
            "a directory",
            Box::new(|dir| Config::at(dir.join("store")).with_signer(common::signer(dir, dir))),
        ),
        (
            "a file that is not a key",
            Box::new(|dir| {
                let path = dir.join("not-a-key");
                std::fs::write(&path, b"not a key").expect("a file");
                Config::at(dir.join("store")).with_signer(common::signer(dir, &path))
            }),
        ),
    ];
    for (label, make) in unusable {
        let directory = tempfile::tempdir().expect("a directory");
        let program = common::fake_verifier(directory.path());
        let mut host = Native::new(make(directory.path()).with_verifier(program));
        copy(
            Path::new(TINY),
            &host.config().store.join("drafts").join("tiny"),
        );
        let refused = host
            .verify_start(verify_start::Request {
                project: "tiny".into(),
                selection: 0,
                files: String::new(),
            })
            .map(|done| done.task)
            .map_err(failure);
        assert_eq!(
            refused,
            Err(verify_start::Failure::VerifierUnavailable),
            "{label}"
        );
        // The request is judged before the service: a project that is not one is refused as such.
        let absent = host
            .verify_start(verify_start::Request {
                project: "absent".into(),
                selection: 0,
                files: String::new(),
            })
            .map(|done| done.task)
            .map_err(failure);
        assert_eq!(
            absent,
            Err(verify_start::Failure::ProjectRefused),
            "{label}: an absent project"
        );
        // No task was made, no process was started, and nothing was copied for one.
        let known = host
            .verify_control(verify_control::Request {
                task: "task-1".into(),
                action: 0,
            })
            .map(|_| ())
            .map_err(failure);
        assert_eq!(known, Err(verify_control::Failure::UnknownTask), "{label}");
        assert_eq!(
            files_below(&host.config().store.join("builds")),
            Vec::<String>::new(),
            "{label}: no copy of the project was made"
        );
    }

    // With a key, the same request starts the first task.
    let directory = tempfile::tempdir().expect("a directory");
    let program = common::fake_verifier(directory.path());
    let mut host = Native::new(config(directory.path()).with_verifier(program));
    copy(
        Path::new(TINY),
        &host.config().store.join("drafts").join("tiny"),
    );
    let started = host
        .verify_start(verify_start::Request {
            project: "tiny".into(),
            selection: 0,
            files: String::new(),
        })
        .map_err(failure)
        .expect("a task");
    assert_eq!(started.task, "task-1");
}

// SG-10

#[test]
fn a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let job = job(directory.path(), &config);
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(
        outcome.class,
        0,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    let result: Value = serde_json::from_slice(&outcome.result).expect("JSON");
    assert_eq!(result["attestation_id"], id());
    assert_eq!(
        result["keyid"],
        Signer::from_seed(SEED, config.history_path()).keyid(),
        "the result names the key"
    );

    let record = config.store.join("verifications");
    let envelope = std::fs::read(record.join(format!("{}.json", id()))).expect("the envelope");
    let subject = std::fs::read(record.join(id()).join("attestation.json")).expect("the subject");
    assert_eq!(
        subject,
        attestation_json(),
        "the subject is attestation.json byte for byte"
    );

    let history = signer::read_history(&config.history_path()).expect("the history");
    let statement = signer::validate(&envelope, &history, &subject).expect("a valid envelope");
    let predicate = &statement["predicate"];
    assert_eq!(predicate["status"], "verified");
    assert_eq!(predicate["attestation_id"], id());
    assert_eq!(predicate["build_id"], "bb".repeat(32));
    assert_eq!(predicate["source_id"], "cc".repeat(32));
    assert_eq!(predicate["semantic_id"], "dd".repeat(32));
    assert_eq!(predicate["modules"], serde_json::json!(["Tiny"]));
    assert_eq!(
        predicate["files"],
        serde_json::json!([tiny_digest()]),
        "the digest of each source file the verification covered"
    );
    // Nothing else is in the areas: the envelope, its subject, the evidence it binds (SG-14),
    // and one index entry for the source.
    assert_eq!(
        files_below(&record),
        vec![
            format!("{}.json", id()),
            format!("{}/attestation.json", id()),
            format!("{}/build-manifest.json", id()),
            format!("{}/{RENDERING_PATH}", id()),
            format!("{}/sources/src/Tiny.lex.tex", id()),
        ]
    );
    assert_eq!(
        files_below(&config.store.join("index")),
        vec![format!("by-file/{}", tiny_digest())]
    );
}

// SG-11

#[test]
fn the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let job = job(directory.path(), &config);
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(outcome.class, 0);
    let stored = std::fs::read(
        config
            .store
            .join("verifications")
            .join(format!("{}.json", id())),
    )
    .expect("the envelope");
    let subject = std::fs::read(
        config
            .store
            .join("verifications")
            .join(id())
            .join("attestation.json"),
    )
    .expect("the subject");
    let history = signer::read_history(&config.history_path()).expect("the history");

    let mut host = Native::new(config);
    for (by, value) in [(0, id()), (1, tiny_digest())] {
        let found = host
            .attestation_lookup(attestation_lookup::Request {
                by,
                value,
                want: 0,
                module: String::new(),
            })
            .map_err(failure)
            .expect("a lookup");
        assert!(found.found, "by {by}");
        assert_eq!(
            found.envelope, stored,
            "by {by}: exactly the stored envelope"
        );
        signer::validate(&found.envelope, &history, &subject)
            .unwrap_or_else(|error| panic!("by {by}: {error}"));
    }
}

// SG-12

#[test]
fn a_history_that_cannot_be_written_leaves_no_record_sg_12() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let mut job = job(directory.path(), &config);
    // The temporary file of the history is a directory: the write fails after the read succeeds.
    let walled = directory.path().join("walled");
    std::fs::create_dir_all(walled.join("history.tmp")).expect("a directory");
    job.signer = Arc::new(Signer::from_seed(SEED, walled.join("history")));
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(
        outcome.class,
        5,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    assert!(
        String::from_utf8_lossy(&outcome.result).contains("could not be signed"),
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    nothing_recorded(&config.store);
}

// SG-17, SG-18

#[test]
fn a_verification_concluded_through_the_socket_is_a_valid_record_sg_17() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let mut job = job(directory.path(), &config);
    let socket = config.signer.clone().expect("a signer");
    job.signer = Arc::new(SignerClient::ready(&socket).expect("the signer is ready"));
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(
        outcome.class,
        0,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    let result: Value = serde_json::from_slice(&outcome.result).expect("JSON");
    assert_eq!(
        result["keyid"],
        Signer::from_seed(SEED, config.history_path()).keyid(),
        "the result names the key that signed"
    );
    let record = config.store.join("verifications");
    let envelope = std::fs::read(record.join(format!("{}.json", id()))).expect("the envelope");
    let subject =
        std::fs::read(record.join(id()).join("attestation.json")).expect("the attestation");
    let history = signer::read_history(&config.history_path()).expect("the history");
    signer::validate(&envelope, &history, &subject).expect("the envelope validates");
}

#[test]
fn a_signer_lost_before_the_verification_concludes_leaves_no_record_sg_18() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let mut job = job(directory.path(), &config);
    let socket = config.signer.clone().expect("a signer");
    job.signer = Arc::new(SignerClient::ready(&socket).expect("the signer is ready"));
    std::fs::remove_file(&socket).expect("the signer is gone");
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(
        outcome.class,
        5,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    assert!(
        String::from_utf8_lossy(&outcome.result).contains("could not be signed"),
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    nothing_recorded(&config.store);
}

#[test]
fn a_verified_directory_without_attestation_json_leaves_no_record_sg_12() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let job = job(directory.path(), &config);
    let empty = directory.path().join("empty");
    std::fs::create_dir_all(&empty).expect("a directory");
    let outcome = conclude(&job, &facts(empty));
    assert_eq!(
        outcome.class,
        5,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    nothing_recorded(&config.store);
}

#[test]
fn a_draft_that_does_not_verify_leaves_no_record_and_no_key_entry_sg_12() {
    let directory = tempfile::tempdir().expect("a directory");
    let mut host = Native::new(config(directory.path()));
    let draft = host.config().store.join("drafts").join("tiny");
    copy(Path::new(TINY), &draft);
    std::fs::write(
        draft.join("src/Tiny.lex.tex"),
        b"\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n",
    )
    .expect("a source");
    let started = host
        .verify_start(verify_start::Request {
            project: "tiny".into(),
            selection: 0,
            files: String::new(),
        })
        .map_err(failure)
        .expect("a task");
    let until = Instant::now() + Duration::from_secs(60);
    loop {
        let report = host
            .verify_control(verify_control::Request {
                task: started.task.clone(),
                action: 0,
            })
            .map_err(failure)
            .expect("a known task");
        if report.state == State::Failed as u32 {
            assert_eq!(report.class, 1, "a language error");
            break;
        }
        assert!(Instant::now() < until, "the task did not fail");
        std::thread::sleep(Duration::from_millis(20));
    }
    nothing_recorded(&host.config().store);
}

// SG-14

#[test]
fn the_record_keeps_the_manifest_the_renderings_and_the_sources_sg_14() {
    let directory = tempfile::tempdir().expect("a directory");
    let config = config(directory.path());
    let job = job(directory.path(), &config);
    let outcome = conclude(&job, &facts(verified_root(directory.path())));
    assert_eq!(
        outcome.class,
        0,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    let record = config.store.join("verifications").join(id());
    let read = |path: &str| {
        std::fs::read(record.join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
    };
    // Each file is kept byte for byte.
    assert_eq!(read("build-manifest.json"), manifest());
    assert_eq!(read(RENDERING_PATH), RENDERING);
    assert_eq!(
        read("sources/src/Tiny.lex.tex"),
        std::fs::read(Path::new(TINY).join("src/Tiny.lex.tex")).expect("a source")
    );
    // The digests lead from the signed subject to each of them.
    let subject: Value = serde_json::from_slice(&read("attestation.json")).expect("JSON");
    assert_eq!(
        subject["build_manifest"]["sha256"],
        sha(&read("build-manifest.json"))
    );
    let kept: Value = serde_json::from_slice(&read("build-manifest.json")).expect("JSON");
    assert_eq!(kept["outputs"][0]["sha256"], sha(&read(RENDERING_PATH)));
    assert_eq!(
        kept["inputs"][0]["sha256"],
        sha(&read("sources/src/Tiny.lex.tex"))
    );
}

// SG-15

/// What a verification left below `dir`, for the project `project`, changed so that its
/// evidence cannot be kept whole; the verified directory.
type Spoil = fn(&Path, &Path) -> PathBuf;

/// A build manifest that names the source of the tiny project and the outputs given.
fn manifest_with_outputs(outputs: Value) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "inputs": [{"kind": "source", "path": "src/Tiny.lex.tex", "sha256": tiny_digest()}],
        "outputs": outputs,
    }))
    .expect("JSON")
}

#[test]
fn evidence_that_cannot_be_kept_whole_leaves_no_record_sg_15() {
    let cases: [(&str, Spoil); 10] = [
        ("no build manifest", |dir, _| {
            let root = verified_root(dir);
            std::fs::remove_file(root.join("build-manifest.json")).expect("removed");
            root
        }),
        (
            "a manifest that is not the one attestation.json names",
            |dir, _| {
                let root = verified_root(dir);
                let mut bytes = manifest();
                bytes.push(b' ');
                std::fs::write(root.join("build-manifest.json"), bytes).expect("a file");
                root
            },
        ),
        ("an attestation.json that names no manifest", |dir, _| {
            let root = verified_root(dir);
            std::fs::write(
                root.join("attestation.json"),
                format!("{{\"attestation_id\":\"{}\"}}", id()),
            )
            .expect("a file");
            root
        }),
        ("a missing rendering", |dir, _| {
            verified_with(dir, &manifest(), &[])
        }),
        (
            "a rendering that is not the one the manifest names",
            |dir, _| verified_with(dir, &manifest(), &[(RENDERING_PATH, b"altered")]),
        ),
        (
            "a source that is not the one the manifest names",
            |dir, project| {
                std::fs::write(project.join("src/Tiny.lex.tex"), b"altered").expect("a file");
                verified_root(dir)
            },
        ),
        ("a missing source", |dir, project| {
            std::fs::remove_file(project.join("src/Tiny.lex.tex")).expect("removed");
            verified_root(dir)
        }),
        ("a manifest that names no source", |dir, _| {
            let bare = serde_json::to_vec(&serde_json::json!({
                "inputs": [],
                "outputs": [{"kind": "tex", "path": RENDERING_PATH, "sha256": sha(RENDERING)}],
            }))
            .expect("JSON");
            verified_with(dir, &bare, &[(RENDERING_PATH, RENDERING)])
        }),
        ("a manifest that names no rendering", |dir, _| {
            verified_with(dir, &manifest_with_outputs(serde_json::json!([])), &[])
        }),
        (
            "a rendering whose path leaves the verified directory",
            |dir, _| {
                let escaping = manifest_with_outputs(serde_json::json!([
                    {"kind": "tex", "path": "../outside.tex", "sha256": sha(RENDERING)}
                ]));
                verified_with(dir, &escaping, &[("../outside.tex", RENDERING)])
            },
        ),
    ];
    for (label, spoil) in cases {
        let directory = tempfile::tempdir().expect("a directory");
        let config = config(directory.path());
        let job = job(directory.path(), &config);
        let root = spoil(directory.path(), &job.directory);
        let outcome = conclude(&job, &facts(root));
        let message = String::from_utf8_lossy(&outcome.result);
        assert_eq!(outcome.class, 5, "{label}: {message}");
        assert!(message.contains("evidence"), "{label}: {message}");
        nothing_recorded(&config.store);
    }
}
