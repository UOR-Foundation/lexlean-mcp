//! AT-01 to AT-04: `lexlean_attestation` (F10, issue #12) calls the production core with the real
//! host, as a client does, on a store that holds the signed record of a concluded verification.
//!
//! The verification itself needs the Lean toolchain (the ignored test of `effects.rs` in the
//! binding runs it). What is made here is what the host leaves when one ends `verified`:
//! `conclude` signs and records the facts of a verification, as `records.rs` of the binding does,
//! and the oracle of what the tool must return is the envelope in the store and the key history.

use std::sync::Arc;

use gateway_binding::signer::{self, Signer};
use gateway_binding::{conclude, Facts, Job};
use gateway_tool_tests::{copy_tree, tiny, tree, Gateway, SOURCE};
use lexlean::Selection;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// The key of the gateway of the tests (`Gateway::new` writes 32 octets of `0x44`).
const SEED: [u8; 32] = [0x44; 32];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn id() -> String {
    "ab".repeat(32)
}

/// The attestation.json of the verified directory: bytes the signer must not alter.
fn attestation_json() -> Vec<u8> {
    format!(
        "{{\"attestation_id\":\"{}\",\"toolchain\":{{\"lean\":\"4.32.1\"}}}}",
        id()
    )
    .into_bytes()
}

/// The digest of the one source file the verification covered.
fn source_digest() -> String {
    hex(&Sha256::digest(
        std::fs::read(tiny().join(SOURCE)).expect("a source"),
    ))
}

/// A gateway whose store holds the signed record of a concluded verification.
fn with_record() -> Gateway {
    let gateway = Gateway::new();
    let work = tempfile::tempdir().expect("a directory");
    let project = work.path().join("project");
    copy_tree(&tiny(), &project);
    let verified = work.path().join("verified");
    std::fs::create_dir_all(&verified).expect("a directory");
    std::fs::write(verified.join("attestation.json"), attestation_json()).expect("a file");
    let config = gateway.config();
    let job = Job {
        directory: project,
        selection: Selection::Entrypoints,
        store: config.store.clone(),
        signer: Arc::new(Signer::from_seed(SEED, config.history_path())),
    };
    let facts = Facts {
        attestation_id: id(),
        build_id: "bb".repeat(32),
        source_id: "cc".repeat(32),
        semantic_id: "dd".repeat(32),
        modules: vec!["Tiny".to_owned()],
        root: verified,
    };
    let outcome = conclude(&job, &facts);
    assert_eq!(
        outcome.class,
        0,
        "{}",
        String::from_utf8_lossy(&outcome.result)
    );
    gateway
}

fn verifications(gateway: &Gateway) -> std::path::PathBuf {
    gateway.store().join("verifications")
}

/// The envelope the store holds for the record, and the file it signs.
fn stored(gateway: &Gateway) -> (Vec<u8>, Vec<u8>) {
    let record = verifications(gateway);
    (
        std::fs::read(record.join(format!("{}.json", id()))).expect("the envelope"),
        std::fs::read(record.join(id()).join("attestation.json")).expect("the subject"),
    )
}

/// The statement the envelope carries, once it validates against the key history of the store
/// and the file it signs.
fn validated(gateway: &Gateway, envelope: &[u8], subject: &[u8]) -> Value {
    let history = signer::read_history(&gateway.config().history_path()).expect("the history");
    signer::validate(envelope, &history, subject)
        .unwrap_or_else(|error| panic!("the envelope does not validate: {error}"))
}

#[test]
fn the_tool_returns_the_envelope_by_attestation_id_at_01() {
    let mut gateway = with_record();
    let (envelope, subject) = stored(&gateway);
    let reply = gateway.call("lexlean_attestation", json!({"by": "id", "value": id()}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(reply.effects, ["attestation.lookup"]);
    assert_eq!(reply.structured()["found"], true);
    assert_eq!(
        reply.text().as_bytes(),
        envelope.as_slice(),
        "the text is the envelope in the store, byte for byte"
    );
    let statement = validated(&gateway, reply.text().as_bytes(), &subject);
    assert_eq!(statement["predicate"]["attestation_id"], id());
    assert_eq!(statement["predicate"]["status"], "verified");
}

#[test]
fn the_tool_returns_the_same_envelope_by_the_digest_of_a_source_file_at_02() {
    let mut gateway = with_record();
    let (envelope, subject) = stored(&gateway);
    let by_id = gateway.call("lexlean_attestation", json!({"by": "id", "value": id()}));
    let by_digest = gateway.call(
        "lexlean_attestation",
        json!({"by": "digest", "value": source_digest()}),
    );
    assert!(!by_digest.is_error(), "{}", by_digest.body);
    assert_eq!(by_digest.effects, ["attestation.lookup"]);
    assert_eq!(by_digest.structured()["found"], true);
    assert_eq!(by_digest.text().as_bytes(), envelope.as_slice());
    assert_eq!(
        by_digest.text(),
        by_id.text(),
        "both ways give the one envelope"
    );
    validated(&gateway, by_digest.text().as_bytes(), &subject);
}

#[test]
fn an_absent_record_is_not_found_and_a_malformed_value_is_refused_at_03() {
    let mut gateway = with_record();
    // A well-formed value that names nothing: not an error, found is false.
    for (by, value) in [
        ("id", "00".repeat(32)),
        ("digest", "00".repeat(32)),
        ("digest", id()),
    ] {
        let reply = gateway.call("lexlean_attestation", json!({"by": by, "value": value}));
        assert!(!reply.is_error(), "{by} {value}: {}", reply.body);
        assert_eq!(reply.effects, ["attestation.lookup"], "{by} {value}");
        assert_eq!(reply.structured()["found"], false, "{by} {value}");
        assert_eq!(reply.text(), "not found", "{by} {value}");
    }
    // A value that is no digest is refused by the host as value-refused.
    for (by, value) in [
        ("id", "not-hex".to_string()),
        ("id", "ab".repeat(31)),
        ("digest", "AB".repeat(32)),
        ("digest", "../etc/passwd".to_string()),
    ] {
        let reply = gateway.call("lexlean_attestation", json!({"by": by, "value": value}));
        assert!(reply.is_error(), "{by} {value:?}: {}", reply.body);
        assert_eq!(reply.effects, ["attestation.lookup"], "{by} {value:?}");
        assert_eq!(
            reply.structured()["failure"],
            "value-refused",
            "{by} {value:?}: {}",
            reply.body
        );
    }
    // An empty value is refused by the core as bad arguments: the host is never asked.
    let reply = gateway.call("lexlean_attestation", json!({"by": "id", "value": ""}));
    assert!(reply.effects.is_empty(), "{:?}", reply.effects);
    assert!(
        reply.body.get("error").is_some() || reply.is_error(),
        "{}",
        reply.body
    );
}

#[test]
fn the_tool_on_an_empty_store_finds_nothing_at_03() {
    let mut gateway = Gateway::new();
    for (by, value) in [("id", id()), ("digest", source_digest())] {
        let reply = gateway.call("lexlean_attestation", json!({"by": by, "value": value}));
        assert!(!reply.is_error(), "{by}: {}", reply.body);
        assert_eq!(reply.structured()["found"], false, "{by}");
    }
}

#[test]
fn a_lookup_writes_nothing_to_the_store_at_04() {
    let mut gateway = with_record();
    let before = tree(gateway.store());
    for (by, value) in [
        ("id", id()),
        ("digest", source_digest()),
        ("id", "00".repeat(32)),
        ("id", "not-hex".to_string()),
    ] {
        gateway.call("lexlean_attestation", json!({"by": by, "value": value}));
    }
    assert_eq!(
        tree(gateway.store()),
        before,
        "the store is as it was after the lookups"
    );
}
