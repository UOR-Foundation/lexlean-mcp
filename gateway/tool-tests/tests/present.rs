//! PT-01 to PT-05 and EX-01 to EX-05: `lexlean_present` (F9) and `lexlean_export` (F12), issue #12.
//!
//! The tools run as a client calls them, through the production core and the real host, on a store
//! that holds the signed record of a concluded verification. The verification itself needs the
//! Lean toolchain (the ignored test of `taskhost.rs` runs it and then presents and exports its
//! record); what is made here is what the host leaves when one ends `verified`: `conclude` signs and
//! records the facts, the evidence the build manifest names, and the envelope, as `records.rs` of
//! the binding does. The oracle is the store, the key history, and for AC-06 the canonical LaTeX
//! that LexLean wrote for the example `nat-add-zero` (`fixture/nat-add-zero/expected/Main.tex`).

use std::path::{Path, PathBuf};
use std::sync::Arc;

use gateway_binding::signer::{self, Signer};
use gateway_binding::{conclude, Facts, Job};
use gateway_host_api::protocol::{attestation_lookup, Host, HostError};
use gateway_tool_tests::{fixture, tiny, tree, Gateway, SOURCE};
use lexlean::Selection;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// The key of the gateway of the tests (`Gateway::new` writes 32 octets of `0x44`).
const SEED: [u8; 32] = [0x44; 32];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn digest(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The files a verification covered and the renderings it made, by path.
struct Evidence {
    id: String,
    sources: Vec<(String, Vec<u8>)>,
    renderings: Vec<(String, Vec<u8>)>,
}

impl Evidence {
    /// The tiny project: one source, one rendering.
    fn tiny() -> Evidence {
        Evidence {
            id: "ab".repeat(32),
            sources: vec![(
                SOURCE.to_owned(),
                std::fs::read(tiny().join(SOURCE)).expect("a source"),
            )],
            renderings: vec![(
                "modules/Tiny/Tiny.tex".to_owned(),
                b"\\documentclass[11pt]{article}\n\\begin{document}\nTiny\n\\end{document}\n"
                    .to_vec(),
            )],
        }
    }

    /// The example `nat-add-zero` as LexLean wrote it: its source and its canonical LaTeX.
    fn nat_add_zero() -> Evidence {
        let example = fixture("nat-add-zero");
        Evidence {
            id: "e1".repeat(32),
            sources: vec![(
                "src/Main.lex.tex".to_owned(),
                std::fs::read(example.join("src/Main.lex.tex")).expect("the source"),
            )],
            renderings: vec![(
                "modules/LexLeanExample/Main.tex".to_owned(),
                std::fs::read(example.join("expected/Main.tex")).expect("the rendering"),
            )],
        }
    }

    /// Two modules: two sources and two renderings.
    fn two_modules() -> Evidence {
        Evidence {
            id: "cd".repeat(32),
            sources: vec![
                (
                    "src/A.lex.tex".to_owned(),
                    b"\\begin{lexlean}{A}\n".to_vec(),
                ),
                (
                    "src/B.lex.tex".to_owned(),
                    b"\\begin{lexlean}{B}\n".to_vec(),
                ),
            ],
            renderings: vec![
                ("modules/A/A.tex".to_owned(), b"A \\LaTeX\n".to_vec()),
                ("modules/B/B.tex".to_owned(), b"B \\LaTeX\n".to_vec()),
            ],
        }
    }

    fn manifest(&self) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "inputs": self.sources.iter()
                .map(|(path, bytes)| json!({"kind": "source", "path": path, "sha256": digest(bytes)}))
                .collect::<Vec<_>>(),
            "outputs": self.renderings.iter()
                .map(|(path, bytes)| json!({"kind": "tex", "path": path, "sha256": digest(bytes)}))
                .collect::<Vec<_>>(),
        }))
        .expect("JSON")
    }

    fn attestation_json(&self) -> Vec<u8> {
        format!(
            "{{\"attestation_id\":\"{}\",\"build_manifest\":{{\"sha256\":\"{}\"}},\"toolchain\":{{\"lean\":\"4.32.1\"}}}}",
            self.id,
            digest(&self.manifest())
        )
        .into_bytes()
    }

    /// The digest of the first source file: the key of the index entry that finds the record.
    fn source_digest(&self, index: usize) -> String {
        digest(&self.sources[index].1)
    }

    /// The record of this verification, signed and written to the store of `gateway`, as the host
    /// does when a verification ends `verified`.
    fn record(&self, gateway: &Gateway) {
        let work = tempfile::tempdir().expect("a directory");
        let project = work.path().join("project");
        for (path, bytes) in &self.sources {
            write(&project.join(path), bytes);
        }
        let verified = work.path().join("verified");
        write(&verified.join("attestation.json"), &self.attestation_json());
        write(&verified.join("build-manifest.json"), &self.manifest());
        for (path, bytes) in &self.renderings {
            write(&verified.join(path), bytes);
        }
        let config = gateway.config();
        let job = Job {
            directory: project,
            selection: Selection::Entrypoints,
            store: config.store.clone(),
            signer: Arc::new(Signer::from_seed(SEED, config.history_path())),
        };
        let facts = Facts {
            attestation_id: self.id.clone(),
            build_id: "bb".repeat(32),
            source_id: "cc".repeat(32),
            semantic_id: "dd".repeat(32),
            modules: vec!["Module".to_owned()],
            root: verified,
        };
        let outcome = conclude(&job, &facts);
        assert_eq!(
            outcome.class,
            0,
            "{}",
            String::from_utf8_lossy(&outcome.result)
        );
    }
}

fn write(path: &Path, bytes: &[u8]) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
    std::fs::write(path, bytes).expect("a file");
}

/// A gateway whose store holds the records of `evidence`.
fn holding(evidence: &[&Evidence]) -> Gateway {
    let gateway = Gateway::new();
    for one in evidence {
        one.record(&gateway);
    }
    gateway
}

fn record_dir(gateway: &Gateway, id: &str) -> PathBuf {
    gateway.store().join("verifications").join(id)
}

fn envelope_in_store(gateway: &Gateway, id: &str) -> Vec<u8> {
    std::fs::read(
        gateway
            .store()
            .join("verifications")
            .join(format!("{id}.json")),
    )
    .expect("the envelope")
}

/// The statement the envelope carries, once it validates against the key history of the store and
/// the file it signs.
fn validated(history: &Path, envelope: &[u8], subject: &[u8]) -> Value {
    let history = signer::read_history(history).expect("the history");
    signer::validate(envelope, &history, subject)
        .unwrap_or_else(|error| panic!("the envelope does not validate: {error}"))
}

/// The one embedded resource of a reply: its uri, its type and its text.
fn resource(reply: &gateway_tool_tests::Reply) -> (String, String, String) {
    let content = reply.result()["content"].as_array().expect("content");
    assert_eq!(content.len(), 1, "one block: {}", reply.body);
    assert_eq!(content[0]["type"], "resource", "{}", reply.body);
    let resource = &content[0]["resource"];
    let text = |key: &str| resource[key].as_str().expect(key).to_owned();
    (text("uri"), text("mimeType"), text("text"))
}

// ---- lexlean_present ------------------------------------------------------------------------

#[test]
fn the_tool_presents_the_attested_rendering_as_an_embedded_resource_pt_01() {
    let evidence = Evidence::tiny();
    let mut gateway = holding(&[&evidence]);
    let reply = gateway.call("lexlean_present", json!({"by": "id", "value": evidence.id}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(reply.effects, ["attestation.lookup"]);
    let (uri, mime, text) = resource(&reply);
    let (path, rendering) = &evidence.renderings[0];
    assert_eq!(uri, format!("lexlean://attestation/{}/{path}", evidence.id));
    assert_eq!(mime, "application/x-tex");
    let stored = std::fs::read(record_dir(&gateway, &evidence.id).join(path)).expect("a rendering");
    assert_eq!(text.as_bytes(), stored.as_slice(), "the bytes in the store");
    assert_eq!(text.as_bytes(), rendering.as_slice(), "the bytes attested");
    let structured = reply.structured();
    assert_eq!(structured["found"], true);
    assert_eq!(structured["attestationId"], evidence.id.as_str());
    assert_eq!(structured["module"], path.as_str());
    // The envelope in the structured content is the envelope in the store, and it validates.
    let envelope = envelope_in_store(&gateway, &evidence.id);
    assert_eq!(
        structured["envelope"],
        serde_json::from_slice::<Value>(&envelope).expect("JSON")
    );
    let subject = std::fs::read(record_dir(&gateway, &evidence.id).join("attestation.json"))
        .expect("the subject");
    let statement = validated(&gateway.config().history_path(), &envelope, &subject);
    assert_eq!(
        statement["predicate"]["attestation_id"],
        evidence.id.as_str()
    );
}

#[test]
fn j1_for_nat_add_zero_ends_in_the_expected_canonical_latex_pt_02() {
    let evidence = Evidence::nat_add_zero();
    let mut gateway = holding(&[&evidence]);
    let reply = gateway.call(
        "lexlean_present",
        json!({"by": "digest", "value": evidence.source_digest(0)}),
    );
    assert!(!reply.is_error(), "{}", reply.body);
    let (_, _, text) = resource(&reply);
    let expected = std::fs::read(fixture("nat-add-zero").join("expected/Main.tex")).expect("tex");
    assert_eq!(
        text.as_bytes(),
        expected.as_slice(),
        "the expected canonical LaTeX"
    );
    // The digest of the embedded bytes is the one the attested build manifest names.
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(record_dir(&gateway, &evidence.id).join("build-manifest.json"))
            .expect("the manifest"),
    )
    .expect("JSON");
    let named = manifest["outputs"]
        .as_array()
        .expect("outputs")
        .iter()
        .find(|row| row["kind"] == "tex")
        .expect("a rendering");
    assert_eq!(named["sha256"], digest(text.as_bytes()).as_str());
}

#[test]
fn by_digest_and_by_module_the_tool_presents_the_chosen_rendering_pt_03() {
    let one = Evidence::tiny();
    let two = Evidence::two_modules();
    let mut gateway = holding(&[&one, &two]);
    // By the digest of a source file: the same resource as by ID.
    let by_id = gateway.call("lexlean_present", json!({"by": "id", "value": one.id}));
    let by_file = gateway.call(
        "lexlean_present",
        json!({"by": "digest", "value": one.source_digest(0)}),
    );
    assert_eq!(by_file.result(), by_id.result());
    // A module of a record that has one: named or not.
    let named = gateway.call(
        "lexlean_present",
        json!({"by": "id", "value": one.id, "module": "modules/Tiny/Tiny.tex"}),
    );
    assert_eq!(named.result(), by_id.result());
    // The module selects a rendering of a record that has two.
    for (path, bytes) in &two.renderings {
        let reply = gateway.call(
            "lexlean_present",
            json!({"by": "id", "value": two.id, "module": path}),
        );
        assert!(!reply.is_error(), "{path}: {}", reply.body);
        assert_eq!(resource(&reply).2.as_bytes(), bytes.as_slice(), "{path}");
        assert_eq!(reply.structured()["module"], path.as_str());
    }
    let by_second_file = gateway.call(
        "lexlean_present",
        json!({"by": "digest", "value": two.source_digest(1), "module": "modules/B/B.tex"}),
    );
    assert_eq!(resource(&by_second_file).2, "B \\LaTeX\n");
    // No module on two renderings, a module the manifest does not name, and a module on a record
    // that has another one, are the host's refusal.
    for (value, module) in [
        (&two.id, None),
        (&two.id, Some("modules/C/C.tex")),
        (&one.id, Some("modules/A/A.tex")),
        (&one.id, Some("../keys/history")),
    ] {
        let mut arguments = json!({"by": "id", "value": value});
        if let Some(module) = module {
            arguments["module"] = json!(module);
        }
        let reply = gateway.call("lexlean_present", arguments);
        assert!(reply.is_error(), "{value} {module:?}: {}", reply.body);
        assert_eq!(reply.effects, ["attestation.lookup"]);
        assert_eq!(
            reply.structured()["failure"],
            "module-unknown",
            "{value} {module:?}"
        );
    }
}

#[test]
fn a_miss_is_not_found_a_bad_value_is_refused_and_nothing_is_written_pt_04() {
    let evidence = Evidence::tiny();
    let mut gateway = holding(&[&evidence]);
    let before = tree(gateway.store());
    for (by, value) in [
        ("id", "00".repeat(32)),
        ("digest", "00".repeat(32)),
        ("digest", evidence.id.clone()),
    ] {
        let reply = gateway.call("lexlean_present", json!({"by": by, "value": value}));
        assert!(!reply.is_error(), "{by} {value}: {}", reply.body);
        assert_eq!(reply.effects, ["attestation.lookup"], "{by} {value}");
        assert_eq!(reply.structured()["found"], false, "{by} {value}");
        assert_eq!(reply.text(), "not found", "{by} {value}");
    }
    for (by, value) in [
        ("id", "not-hex".to_string()),
        ("id", "ab".repeat(31)),
        ("digest", "AB".repeat(32)),
        ("digest", "../etc/passwd".to_string()),
    ] {
        let reply = gateway.call("lexlean_present", json!({"by": by, "value": value}));
        assert!(reply.is_error(), "{by} {value:?}: {}", reply.body);
        assert_eq!(
            reply.structured()["failure"],
            "value-refused",
            "{by} {value:?}"
        );
    }
    // An empty value, and an empty module, are refused by the core: the host is not asked.
    for arguments in [
        json!({"by": "id", "value": ""}),
        json!({"by": "id", "value": evidence.id, "module": ""}),
        json!({"by": "other", "value": evidence.id}),
        json!({"value": evidence.id}),
    ] {
        let reply = gateway.call("lexlean_present", arguments.clone());
        assert!(reply.effects.is_empty(), "{arguments}: {:?}", reply.effects);
        assert!(
            reply.body.get("error").is_some() || reply.is_error(),
            "{arguments}: {}",
            reply.body
        );
    }
    for arguments in [
        json!({"by": "id", "value": evidence.id}),
        json!({"by": "digest", "value": evidence.source_digest(0)}),
    ] {
        assert!(!gateway.call("lexlean_present", arguments).is_error());
    }
    assert_eq!(tree(gateway.store()), before, "a present writes nothing");
    let mut empty = Gateway::new();
    let reply = empty.call("lexlean_present", json!({"by": "id", "value": evidence.id}));
    assert_eq!(reply.structured()["found"], false);
}

/// Whether asking the host for `view` of the record `id` is a fault of the store.
fn faults(gateway: &mut Gateway, id: &str, view: u32) -> bool {
    let mut host = gateway.take_host();
    let outcome = host.attestation_lookup(attestation_lookup::Request {
        by: 0,
        value: id.to_owned(),
        want: view,
        module: String::new(),
    });
    gateway.put_host(host);
    matches!(outcome, Err(HostError::Fault))
}

#[test]
fn only_what_is_attested_is_presented_pt_05() {
    let evidence = Evidence::tiny();
    let (path, _) = &evidence.renderings[0];
    let spoil = |gateway: &Gateway, name: &str, bytes: &[u8]| {
        write(&record_dir(gateway, &evidence.id).join(name), bytes);
    };
    // The rendering differs from the digest the manifest names.
    let mut gateway = holding(&[&evidence]);
    assert!(
        !faults(&mut gateway, &evidence.id, 1),
        "the record is whole"
    );
    spoil(&gateway, path, b"altered\n");
    assert!(
        faults(&mut gateway, &evidence.id, 1),
        "an altered rendering"
    );
    // The manifest differs from the digest attestation.json names.
    let mut gateway = holding(&[&evidence]);
    spoil(
        &gateway,
        "build-manifest.json",
        b"{\"inputs\":[],\"outputs\":[]}",
    );
    assert!(faults(&mut gateway, &evidence.id, 1), "an altered manifest");
    // The rendering is missing, as in a record that kept no evidence.
    let mut gateway = holding(&[&evidence]);
    std::fs::remove_file(record_dir(&gateway, &evidence.id).join(path)).expect("removed");
    assert!(faults(&mut gateway, &evidence.id, 1), "a missing rendering");
    // The envelope alone is still looked up: only the view that gives bytes checks them.
    assert!(!faults(&mut gateway, &evidence.id, 0));
}

// ---- lexlean_export -------------------------------------------------------------------------

/// The bundle of a record below `exports/<id>`: its files by path.
fn bundle_of(gateway: &Gateway, id: &str) -> std::collections::BTreeMap<String, Vec<u8>> {
    tree(&gateway.store().join("exports").join(id))
}

fn files_of(evidence: &Evidence) -> Vec<String> {
    let mut names: Vec<String> = [
        "attestation.json",
        "envelope.json",
        "keys.json",
        "build-manifest.json",
    ]
    .map(str::to_owned)
    .into();
    names.extend(evidence.renderings.iter().map(|(path, _)| path.clone()));
    names.extend(
        evidence
            .sources
            .iter()
            .map(|(path, _)| format!("sources/{path}")),
    );
    names.sort();
    names
}

#[test]
fn the_tool_writes_the_bundle_of_the_record_ex_01() {
    let evidence = Evidence::two_modules();
    let mut gateway = holding(&[&evidence]);
    assert!(bundle_of(&gateway, &evidence.id).is_empty());
    let reply = gateway.call("lexlean_export", json!({"by": "id", "value": evidence.id}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(reply.effects, ["attestation.lookup"]);
    assert_eq!(reply.structured()["found"], true);
    assert_eq!(reply.structured()["attestationId"], evidence.id.as_str());
    let key = format!("exports/{}", evidence.id);
    assert_eq!(reply.structured()["bundle"], key.as_str());
    assert_eq!(reply.text(), key);
    let bundle = bundle_of(&gateway, &evidence.id);
    assert_eq!(
        bundle.keys().cloned().collect::<Vec<_>>(),
        files_of(&evidence)
    );
    let record = record_dir(&gateway, &evidence.id);
    assert_eq!(
        bundle["attestation.json"],
        std::fs::read(record.join("attestation.json")).expect("the subject")
    );
    assert_eq!(
        bundle["envelope.json"],
        envelope_in_store(&gateway, &evidence.id)
    );
    assert_eq!(
        bundle["keys.json"],
        std::fs::read(gateway.config().history_path()).expect("the history")
    );
    assert_eq!(
        bundle["build-manifest.json"],
        std::fs::read(record.join("build-manifest.json")).expect("the manifest")
    );
    for (path, bytes) in &evidence.renderings {
        assert_eq!(&bundle[path.as_str()], bytes, "{path}");
    }
    for (path, bytes) in &evidence.sources {
        assert_eq!(&bundle[&format!("sources/{path}")], bytes, "{path}");
    }
    // By the digest of a source file it is the same bundle.
    let by_file = gateway.call(
        "lexlean_export",
        json!({"by": "digest", "value": evidence.source_digest(0)}),
    );
    assert_eq!(by_file.structured()["bundle"], key.as_str());
}

/// A reader's check of a bundle, from its files alone: the envelope validates against the keys and
/// the subject, the subject names the manifest, and the manifest names every rendering and source
/// file by its digest. It returns the first thing that does not hold.
fn check(bundle: &Path) -> Result<(), String> {
    let read =
        |name: &str| std::fs::read(bundle.join(name)).map_err(|_| format!("{name} is missing"));
    let (subject, envelope, manifest) = (
        read("attestation.json")?,
        read("envelope.json")?,
        read("build-manifest.json")?,
    );
    let history = signer::read_history(&bundle.join("keys.json")).map_err(|_| "keys.json")?;
    signer::validate(&envelope, &history, &subject)
        .map_err(|error| format!("envelope: {error}"))?;
    let attestation: Value = serde_json::from_slice(&subject).map_err(|_| "attestation.json")?;
    if attestation["build_manifest"]["sha256"] != digest(&manifest).as_str() {
        return Err("the manifest is not the one attestation.json names".to_owned());
    }
    let manifest: Value = serde_json::from_slice(&manifest).map_err(|_| "the manifest")?;
    let mut named = std::collections::BTreeSet::new();
    for (field, kind, prefix) in [("outputs", "tex", ""), ("inputs", "source", "sources/")] {
        for row in manifest[field].as_array().ok_or("a manifest")? {
            if row["kind"] != kind {
                continue;
            }
            let name = format!("{prefix}{}", row["path"].as_str().ok_or("a path")?);
            if row["sha256"] != digest(&read(&name)?).as_str() {
                return Err(format!("{name} is not the file the manifest names"));
            }
            named.insert(name);
        }
    }
    if named.is_empty() {
        return Err("the manifest names no file".to_owned());
    }
    Ok(())
}

#[test]
fn a_reader_checks_the_bundle_offline_ex_02() {
    let evidence = Evidence::two_modules();
    let mut gateway = holding(&[&evidence]);
    gateway.call("lexlean_export", json!({"by": "id", "value": evidence.id}));
    // The bundle is copied out of the store, which then goes: the reader has the files alone.
    let reader = tempfile::tempdir().expect("a directory");
    let copy = reader.path().join("bundle");
    for (name, bytes) in bundle_of(&gateway, &evidence.id) {
        write(&copy.join(name), &bytes);
    }
    drop(gateway);
    assert_eq!(check(&copy), Ok(()));
    // The check fails when a file is altered: the rendering, the source, the envelope, the keys.
    for name in [
        "modules/A/A.tex",
        "sources/src/B.lex.tex",
        "envelope.json",
        "keys.json",
        "attestation.json",
        "build-manifest.json",
    ] {
        let spoiled = reader
            .path()
            .join(format!("spoiled-{}", name.replace('/', "-")));
        for (file, bytes) in tree(&copy) {
            write(&spoiled.join(&file), &bytes);
        }
        let mut bytes = std::fs::read(spoiled.join(name)).expect("a file");
        let last = bytes.len() - 2;
        bytes[last] ^= 1;
        std::fs::write(spoiled.join(name), bytes).expect("a file");
        assert!(check(&spoiled).is_err(), "{name} altered");
    }
}

#[test]
fn the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03() {
    let evidence = Evidence::two_modules();
    let mut gateway = holding(&[&evidence]);
    gateway.call("lexlean_export", json!({"by": "id", "value": evidence.id}));
    let bundle = bundle_of(&gateway, &evidence.id);
    let manifest: Value = serde_json::from_slice(&bundle["build-manifest.json"]).expect("JSON");
    // The manifest of the bundle is the one the signed attestation.json names.
    let subject: Value = serde_json::from_slice(&bundle["attestation.json"]).expect("JSON");
    assert_eq!(
        subject["build_manifest"]["sha256"],
        digest(&bundle["build-manifest.json"]).as_str()
    );
    let renderings: Vec<&Value> = manifest["outputs"]
        .as_array()
        .expect("outputs")
        .iter()
        .filter(|row| row["kind"] == "tex")
        .collect();
    assert_eq!(renderings.len(), 2);
    for row in &renderings {
        let path = row["path"].as_str().expect("a path");
        assert_eq!(row["sha256"], digest(&bundle[path]).as_str(), "{path}");
    }
    // The bundle holds exactly the files the manifest names, besides its own four.
    let mut names: Vec<String> = renderings
        .iter()
        .map(|row| row["path"].as_str().expect("a path").to_owned())
        .collect();
    names.extend(
        manifest["inputs"]
            .as_array()
            .expect("inputs")
            .iter()
            .filter(|row| row["kind"] == "source")
            .map(|row| format!("sources/{}", row["path"].as_str().expect("a path"))),
    );
    names.extend(
        [
            "attestation.json",
            "envelope.json",
            "keys.json",
            "build-manifest.json",
        ]
        .map(str::to_owned),
    );
    names.sort();
    assert_eq!(bundle.keys().cloned().collect::<Vec<_>>(), names);
    // What the bundle holds is what the tool presents.
    for (path, _) in &evidence.renderings {
        let reply = gateway.call(
            "lexlean_present",
            json!({"by": "id", "value": evidence.id, "module": path}),
        );
        assert_eq!(
            resource(&reply).2.as_bytes(),
            bundle[path.as_str()].as_slice()
        );
    }
}

#[test]
fn a_miss_or_a_refusal_writes_nothing_and_a_record_that_is_not_whole_leaves_no_bundle_ex_04() {
    let evidence = Evidence::tiny();
    let mut gateway = holding(&[&evidence]);
    let before = tree(gateway.store());
    for (by, value) in [("id", "00".repeat(32)), ("digest", "00".repeat(32))] {
        let reply = gateway.call("lexlean_export", json!({"by": by, "value": value}));
        assert!(!reply.is_error(), "{by}: {}", reply.body);
        assert_eq!(reply.structured()["found"], false);
        assert_eq!(reply.text(), "not found");
    }
    for value in ["not-hex".to_string(), "AB".repeat(32)] {
        let reply = gateway.call("lexlean_export", json!({"by": "id", "value": value}));
        assert!(reply.is_error(), "{value}: {}", reply.body);
        assert_eq!(reply.structured()["failure"], "value-refused");
    }
    let reply = gateway.call("lexlean_export", json!({"by": "id", "value": ""}));
    assert!(reply.effects.is_empty());
    assert_eq!(tree(gateway.store()), before, "nothing was written");
    // A record whose evidence is altered, or missing, is a fault of the store, and no file of its
    // bundle is written, whichever file is the one that is wrong.
    let (path, _) = &evidence.renderings[0];
    let source = format!("sources/{}", evidence.sources[0].0);
    for (name, bytes) in [
        (path.as_str(), Some(b"altered\n".as_slice())),
        (path.as_str(), None),
        (source.as_str(), Some(b"altered\n".as_slice())),
        (source.as_str(), None),
        ("build-manifest.json", Some(b"{}".as_slice())),
        ("attestation.json", Some(b"{}".as_slice())),
    ] {
        let mut gateway = holding(&[&evidence]);
        let file = record_dir(&gateway, &evidence.id).join(name);
        match bytes {
            Some(bytes) => std::fs::write(&file, bytes).expect("a file"),
            None => std::fs::remove_file(&file).expect("removed"),
        }
        assert!(faults(&mut gateway, &evidence.id, 2), "{name} {bytes:?}");
        assert!(
            bundle_of(&gateway, &evidence.id).is_empty(),
            "{name}: the bundle was written"
        );
    }
    // No key history is no bundle either.
    let mut gateway = holding(&[&evidence]);
    std::fs::remove_file(gateway.config().history_path()).expect("removed");
    assert!(faults(&mut gateway, &evidence.id, 2));
    assert!(bundle_of(&gateway, &evidence.id).is_empty());
}

#[test]
fn an_export_is_repeatable_and_a_bundle_is_apart_from_another_ex_05() {
    let first = Evidence::tiny();
    let second = Evidence::two_modules();
    let mut gateway = holding(&[&first, &second]);
    let records = tree(&gateway.store().join("verifications"));
    gateway.call("lexlean_export", json!({"by": "id", "value": first.id}));
    let once = bundle_of(&gateway, &first.id);
    let again = gateway.call("lexlean_export", json!({"by": "id", "value": first.id}));
    assert!(!again.is_error(), "{}", again.body);
    assert_eq!(bundle_of(&gateway, &first.id), once, "the same bytes");
    assert!(
        bundle_of(&gateway, &second.id).is_empty(),
        "the other is not touched"
    );
    gateway.call("lexlean_export", json!({"by": "id", "value": second.id}));
    assert_eq!(bundle_of(&gateway, &first.id), once);
    assert_eq!(
        bundle_of(&gateway, &second.id)
            .keys()
            .cloned()
            .collect::<Vec<_>>(),
        files_of(&second)
    );
    assert_eq!(
        tree(&gateway.store().join("verifications")),
        records,
        "the records are not changed"
    );
}
