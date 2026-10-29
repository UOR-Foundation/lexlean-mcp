//! SG-01 to SG-07: the signer of the verification records.
//!
//! The expected bytes are made here by code of this file, not by the signer: the PAE below is
//! written again from the DSSE protocol, the keys are the standard's, and every envelope that a
//! test needs to be valid but different (a statement of another type, a subject of no name) is
//! signed here with `ed25519-dalek` directly. A test that asked the signer for its own expected
//! value would prove nothing.

use std::path::{Path, PathBuf};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use ed25519_dalek::{Signer as _, SigningKey, Verifier as _, VerifyingKey};
use gateway_binding::signer::{
    self, EnvelopeError, HistoryError, KeyEntry, KeyError, SignError, Signer,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const SEED_A: [u8; 32] = [0x11; 32];
const SEED_B: [u8; 32] = [0x22; 32];
const SUBJECT: &[u8] = br#"{"attestation_id":"00ff","status":"verified"}"#;
const NOW: u64 = 1_700_000_000;
const PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";
const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
const PREDICATE_TYPE: &str = "https://uor.foundation/lexlean-mcp/verification/v1";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn predicate() -> Value {
    json!({"status": "verified", "modules": ["Nat.AddZero"], "semantic_id": "aa"})
}

/// PAE as the DSSE protocol writes it, independently of the signer.
fn pae(payload_type: &str, payload: &[u8]) -> Vec<u8> {
    let mut out = format!(
        "DSSEv1 {} {} {} ",
        payload_type.len(),
        payload_type,
        payload.len()
    )
    .into_bytes();
    out.extend_from_slice(payload);
    out
}

fn statement(subject: &[u8], predicate: &Value) -> Value {
    json!({
        "_type": STATEMENT_TYPE,
        "subject": [{"name": "attestation.json", "digest": {"sha256": sha256_hex(subject)}}],
        "predicateType": PREDICATE_TYPE,
        "predicate": predicate,
    })
}

fn key_of(seed: [u8; 32]) -> SigningKey {
    SigningKey::from_bytes(&seed)
}

fn keyid_of(seed: [u8; 32]) -> String {
    sha256_hex(key_of(seed).verifying_key().as_bytes())
}

/// A DSSE envelope signed here with the key of `seed`, over any payload type and payload.
fn forge(seed: [u8; 32], payload_type: &str, payload: &[u8]) -> Vec<u8> {
    let key = key_of(seed);
    let signature = key.sign(&pae(payload_type, payload));
    serde_json::to_vec(&json!({
        "payloadType": payload_type,
        "payload": STANDARD.encode(payload),
        "signatures": [{"keyid": keyid_of(seed), "sig": STANDARD.encode(signature.to_bytes())}],
    }))
    .expect("an envelope")
}

fn history_in(dir: &Path) -> PathBuf {
    dir.join("keys").join("history")
}

fn signer_in(dir: &Path, seed: [u8; 32]) -> Signer {
    Signer::from_seed(seed, history_in(dir))
}

/// The envelope the signer writes for `SUBJECT`, and the history it left.
fn written(dir: &Path, seed: [u8; 32]) -> (Vec<u8>, Vec<KeyEntry>) {
    let envelope = signer_in(dir, seed)
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    let history = signer::read_history(&history_in(dir)).expect("a history");
    (envelope, history)
}

fn parse(envelope: &[u8]) -> Value {
    serde_json::from_slice(envelope).expect("an envelope is JSON")
}

fn refused(envelope: &[u8], history: &[KeyEntry], subject: &[u8]) -> EnvelopeError {
    signer::validate(envelope, history, subject).expect_err("the envelope is refused")
}

// SG-01

#[test]
fn the_envelope_is_dsse_over_an_in_toto_statement_sg_01() {
    let dir = tempfile::tempdir().expect("a directory");
    let (envelope, _) = written(dir.path(), SEED_A);
    let document = parse(&envelope);
    let mut keys: Vec<&String> = document.as_object().expect("an object").keys().collect();
    keys.sort();
    assert_eq!(keys, ["payload", "payloadType", "signatures"]);
    assert_eq!(document["payloadType"], PAYLOAD_TYPE);

    let payload = STANDARD
        .decode(document["payload"].as_str().expect("a string"))
        .expect("the payload is base64");
    let decoded: Value = serde_json::from_slice(&payload).expect("the payload is JSON");
    assert_eq!(decoded, statement(SUBJECT, &predicate()));

    let signatures = document["signatures"].as_array().expect("a list");
    assert_eq!(signatures.len(), 1, "one signature");
    let one = signatures[0].as_object().expect("an object");
    assert_eq!(one.len(), 2, "a keyid and a sig and nothing else");
    assert_eq!(one["keyid"], keyid_of(SEED_A));
    let sig = STANDARD
        .decode(one["sig"].as_str().expect("a string"))
        .expect("the signature is base64");
    assert_eq!(sig.len(), 64, "an Ed25519 signature has 64 octets");
    let public =
        VerifyingKey::from_bytes(key_of(SEED_A).verifying_key().as_bytes()).expect("a public key");
    let signature = ed25519_dalek::Signature::from_slice(&sig).expect("a signature");
    public
        .verify(&pae(PAYLOAD_TYPE, &payload), &signature)
        .expect("the signature is over PAE(payloadType, payload)");
}

#[test]
fn the_keyid_is_the_digest_of_the_public_key_not_of_the_seed_sg_01() {
    let dir = tempfile::tempdir().expect("a directory");
    let signer = signer_in(dir.path(), SEED_A);
    assert_eq!(signer.keyid(), keyid_of(SEED_A));
    assert_ne!(signer.keyid(), sha256_hex(&SEED_A));
    assert_eq!(
        signer.public_key(),
        *key_of(SEED_A).verifying_key().as_bytes()
    );
}

// SG-02

#[test]
fn pae_is_the_example_of_the_dsse_protocol_sg_02() {
    assert_eq!(
        signer::pae("http://example.com/HelloWorld", b"hello world"),
        b"DSSEv1 29 http://example.com/HelloWorld 11 hello world"
    );
    // The lengths count octets, not characters.
    assert_eq!(
        signer::pae("é", "ü".as_bytes()),
        "DSSEv1 2 é 2 ü".as_bytes()
    );
}

#[test]
fn the_key_of_rfc_8032_test_1_has_its_public_key_and_signature_sg_02() {
    let seed: [u8; 32] = [
        0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec, 0x2c,
        0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03, 0x1c, 0xae,
        0x7f, 0x60,
    ];
    let dir = tempfile::tempdir().expect("a directory");
    let signer = signer_in(dir.path(), seed);
    assert_eq!(
        hex(&signer.public_key()),
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a"
    );
    // The primitive the signer links is the standard's: the signature of the empty message.
    assert_eq!(
        hex(&key_of(seed).sign(b"").to_bytes()),
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b"
    );
}

#[test]
fn equal_inputs_give_equal_envelopes_sg_02() {
    let first = tempfile::tempdir().expect("a directory");
    let second = tempfile::tempdir().expect("a directory");
    let (one, _) = written(first.path(), SEED_A);
    let (two, _) = written(second.path(), SEED_A);
    assert_eq!(one, two);
    let (other, _) = written(second.path(), SEED_B);
    assert_ne!(one, other, "another key gives another envelope");
}

// SG-03

#[test]
fn the_validator_accepts_what_the_signer_wrote_sg_03() {
    let dir = tempfile::tempdir().expect("a directory");
    let (envelope, history) = written(dir.path(), SEED_A);
    let accepted = signer::validate(&envelope, &history, SUBJECT).expect("the envelope is valid");
    assert_eq!(accepted, statement(SUBJECT, &predicate()));
}

#[test]
fn the_validator_uses_the_key_the_history_names_sg_03() {
    let dir = tempfile::tempdir().expect("a directory");
    let (envelope, _) = written(dir.path(), SEED_A);
    // A history that holds the key under the right keyid, taken from another writer's file.
    let other = tempfile::tempdir().expect("a directory");
    let (_, history) = written(other.path(), SEED_A);
    signer::validate(&envelope, &history, SUBJECT).expect("the history of another store");
    // The same keyid with the public key of another signer is a history that lies.
    let mut lying = history.clone();
    let public = key_of(SEED_B).verifying_key();
    lying[0].public_key = STANDARD.encode(public.as_bytes());
    assert_eq!(
        signer::validate(&envelope, &lying, SUBJECT),
        Err(EnvelopeError::BadSignature),
        "the key of the entry is the key that verifies"
    );
}

// SG-04

#[test]
fn every_refusal_of_an_envelope_is_typed_and_reached_sg_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let (envelope, history) = written(dir.path(), SEED_A);
    let document = parse(&envelope);
    let with = |field: &str, value: Value| -> Vec<u8> {
        let mut changed = document.clone();
        changed[field] = value;
        serde_json::to_vec(&changed).expect("an envelope")
    };
    let payload_bytes = STANDARD
        .decode(document["payload"].as_str().expect("a string"))
        .expect("base64");

    // Malformed: not JSON, and JSON of no envelope.
    assert_eq!(
        refused(b"not json", &history, SUBJECT),
        EnvelopeError::Malformed
    );
    assert_eq!(refused(b"{}", &history, SUBJECT), EnvelopeError::Malformed);
    assert_eq!(
        refused(&with("payload", json!(7)), &history, SUBJECT),
        EnvelopeError::Malformed
    );
    // PayloadType.
    assert_eq!(
        refused(&with("payloadType", json!("text/plain")), &history, SUBJECT),
        EnvelopeError::PayloadType
    );
    // Payload: not base64.
    assert_eq!(
        refused(&with("payload", json!("!!!not base64")), &history, SUBJECT),
        EnvelopeError::Payload
    );
    // NoSignature.
    assert_eq!(
        refused(&with("signatures", json!([])), &history, SUBJECT),
        EnvelopeError::NoSignature
    );
    // UnknownKey: the history lacks the keyid.
    assert_eq!(refused(&envelope, &[], SUBJECT), EnvelopeError::UnknownKey);
    // BadSignature: a flipped signature byte and a flipped payload byte.
    let mut flipped = document.clone();
    let mut sig = STANDARD
        .decode(flipped["signatures"][0]["sig"].as_str().expect("a string"))
        .expect("base64");
    sig[0] ^= 1;
    flipped["signatures"][0]["sig"] = json!(STANDARD.encode(sig));
    assert_eq!(
        refused(
            &serde_json::to_vec(&flipped).expect("an envelope"),
            &history,
            SUBJECT
        ),
        EnvelopeError::BadSignature
    );
    let mut altered = payload_bytes.clone();
    altered[10] ^= 1;
    assert_eq!(
        refused(
            &with("payload", json!(STANDARD.encode(altered))),
            &history,
            SUBJECT
        ),
        EnvelopeError::BadSignature
    );
    // Statement: signed by the right key, and still not a statement of this product.
    let good = statement(SUBJECT, &predicate());
    let planted = [
        json!("not an object"),
        {
            let mut other = good.clone();
            other["_type"] = json!("https://in-toto.io/Statement/v0.1");
            other
        },
        {
            let mut other = good.clone();
            other["predicateType"] = json!("https://slsa.dev/provenance/v1");
            other
        },
        {
            let mut other = good.clone();
            other["subject"] = json!([]);
            other
        },
        {
            let mut other = good.clone();
            other["subject"][0]["name"] = json!("other.json");
            other
        },
        {
            let mut other = good.clone();
            other["subject"] = json!([good["subject"][0].clone(), good["subject"][0].clone()]);
            other
        },
        {
            let mut other = good.clone();
            other["subject"][0]["digest"] = json!({"sha512": "00"});
            other
        },
    ];
    for statement in planted {
        let bytes = serde_json::to_vec(&statement).expect("a statement");
        assert_eq!(
            refused(&forge(SEED_A, PAYLOAD_TYPE, &bytes), &history, SUBJECT),
            EnvelopeError::Statement,
            "{statement}"
        );
    }
    assert_eq!(
        refused(
            &forge(SEED_A, PAYLOAD_TYPE, b"not json at all"),
            &history,
            SUBJECT
        ),
        EnvelopeError::Statement
    );
    // SubjectMismatch: the bytes of another attestation.
    assert_eq!(
        refused(&envelope, &history, b"{\"attestation_id\":\"ffff\"}"),
        EnvelopeError::SubjectMismatch
    );
}

#[test]
fn a_valid_signature_beside_a_bad_one_is_accepted_and_a_bad_one_alone_is_not_sg_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let (envelope, history) = written(dir.path(), SEED_A);
    let mut document = parse(&envelope);
    let good = document["signatures"][0].clone();
    document["signatures"] = json!([
        {"keyid": "00".repeat(32), "sig": STANDARD.encode([0u8; 64])},
        good,
    ]);
    let both = serde_json::to_vec(&document).expect("an envelope");
    signer::validate(&both, &history, SUBJECT).expect("one valid signature is enough");
    document["signatures"] = json!([{"keyid": "00".repeat(32), "sig": STANDARD.encode([0u8; 64])}]);
    let alone = serde_json::to_vec(&document).expect("an envelope");
    assert_eq!(
        refused(&alone, &history, SUBJECT),
        EnvelopeError::UnknownKey
    );
}

// SG-05

#[test]
fn the_first_use_of_a_key_adds_its_entry_and_later_uses_add_nothing_sg_05() {
    let dir = tempfile::tempdir().expect("a directory");
    assert_eq!(
        signer::read_history(&history_in(dir.path())),
        Ok(Vec::new()),
        "no file is an empty history"
    );
    let signer = signer_in(dir.path(), SEED_A);
    signer
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    let first = signer::read_history(&history_in(dir.path())).expect("a history");
    assert_eq!(
        first,
        vec![KeyEntry {
            keyid: keyid_of(SEED_A),
            algorithm: "Ed25519".to_string(),
            public_key: STANDARD.encode(key_of(SEED_A).verifying_key().as_bytes()),
            not_before: NOW,
            revoked_at: None,
        }]
    );
    signer
        .sign(b"{\"other\":1}", &predicate(), NOW + 500)
        .expect("a signature");
    assert_eq!(
        signer::read_history(&history_in(dir.path())).expect("a history"),
        first,
        "a second use adds nothing and keeps not_before"
    );
}

#[test]
fn a_second_key_is_a_rotation_and_the_first_envelopes_stay_valid_sg_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let (old, _) = written(dir.path(), SEED_A);
    let newer = signer_in(dir.path(), SEED_B)
        .sign(SUBJECT, &predicate(), NOW + 1000)
        .expect("a signature");
    let history = signer::read_history(&history_in(dir.path())).expect("a history");
    assert_eq!(
        history
            .iter()
            .map(|entry| entry.keyid.clone())
            .collect::<Vec<_>>(),
        vec![keyid_of(SEED_A), keyid_of(SEED_B)],
        "the keys in the order of their first use"
    );
    assert_eq!(history[1].not_before, NOW + 1000);
    signer::validate(&old, &history, SUBJECT).expect("the envelope of the old key");
    signer::validate(&newer, &history, SUBJECT).expect("the envelope of the new key");
}

#[test]
fn a_revoked_key_fails_and_the_other_key_does_not_sg_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let (old, _) = written(dir.path(), SEED_A);
    let newer = signer_in(dir.path(), SEED_B)
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    signer::revoke(&history_in(dir.path()), &keyid_of(SEED_A), NOW + 9).expect("a revocation");
    let history = signer::read_history(&history_in(dir.path())).expect("a history");
    assert_eq!(history[0].revoked_at, Some(NOW + 9));
    assert_eq!(history[1].revoked_at, None);
    assert_eq!(refused(&old, &history, SUBJECT), EnvelopeError::KeyRevoked);
    signer::validate(&newer, &history, SUBJECT).expect("the other key is not revoked");
    assert_eq!(
        signer::revoke(&history_in(dir.path()), &"ab".repeat(32), NOW),
        Err(HistoryError::UnknownKey),
        "only a key of the history is revoked"
    );
}

#[test]
fn a_revoked_key_signs_nothing_sg_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let signer = signer_in(dir.path(), SEED_A);
    signer
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    signer::revoke(&history_in(dir.path()), signer.keyid(), NOW + 1).expect("a revocation");
    assert_eq!(
        signer.sign(SUBJECT, &predicate(), NOW + 2),
        Err(SignError::KeyRevoked),
        "an envelope that every reader refuses is not made"
    );
    // Revoking again keeps the first time.
    signer::revoke(&history_in(dir.path()), signer.keyid(), NOW + 50).expect("a revocation");
    let history = signer::read_history(&history_in(dir.path())).expect("a history");
    assert_eq!(history[0].revoked_at, Some(NOW + 1));
}

#[test]
fn a_history_that_is_not_a_list_of_good_entries_is_malformed_sg_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let path = history_in(dir.path());
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
    let entry = json!({
        "keyid": keyid_of(SEED_A),
        "algorithm": "Ed25519",
        "public_key": STANDARD.encode(key_of(SEED_A).verifying_key().as_bytes()),
        "not_before": NOW,
    });
    let mut cases: Vec<(&str, Value)> = vec![
        ("an object", json!({"keys": []})),
        ("a number", json!(3)),
        ("an entry that is a string", json!(["x"])),
    ];
    for field in ["keyid", "algorithm", "public_key", "not_before"] {
        let mut without = entry.clone();
        without.as_object_mut().expect("an object").remove(field);
        cases.push((field, json!([without])));
    }
    let mut other_algorithm = entry.clone();
    other_algorithm["algorithm"] = json!("RSA");
    cases.push(("another algorithm", json!([other_algorithm])));
    let mut short_key = entry.clone();
    short_key["public_key"] = json!(STANDARD.encode([1u8; 16]));
    cases.push(("a public key of 16 octets", json!([short_key])));
    let mut not_base64 = entry.clone();
    not_base64["public_key"] = json!("!!!");
    cases.push(("a public key that is not base64", json!([not_base64])));
    let mut wrong_id = entry.clone();
    wrong_id["keyid"] = json!("00".repeat(32));
    cases.push((
        "a keyid that is not the digest of the key",
        json!([wrong_id]),
    ));
    let mut doubled = vec![entry.clone(), entry.clone()];
    cases.push(("the same key twice", json!(doubled.split_off(0))));
    for (label, value) in cases {
        std::fs::write(&path, serde_json::to_vec(&value).expect("JSON")).expect("a write");
        assert_eq!(
            signer::read_history(&path),
            Err(HistoryError::Malformed),
            "{label}"
        );
    }
    std::fs::write(&path, b"not json").expect("a write");
    assert_eq!(signer::read_history(&path), Err(HistoryError::Malformed));
    std::fs::remove_file(&path).expect("a removal");
    std::fs::create_dir(&path).expect("a directory in the place of the file");
    assert_eq!(signer::read_history(&path), Err(HistoryError::Unreadable));
}

// SG-06

fn key_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, content).expect("a key file");
    path
}

#[test]
fn a_key_file_of_64_hexadecimal_digits_is_the_seed_sg_06() {
    let dir = tempfile::tempdir().expect("a directory");
    let seed = hex(&SEED_A);
    for (name, content) in [
        ("plain", seed.clone()),
        ("newline", format!("{seed}\n")),
        ("spaces", format!("  {seed} \r\n")),
        ("capitals", seed.to_uppercase()),
    ] {
        let path = key_file(dir.path(), name, content.as_bytes());
        let loaded = Signer::load(&path, history_in(dir.path())).expect(name);
        assert_eq!(loaded.keyid(), keyid_of(SEED_A), "{name}");
    }
    let path = key_file(dir.path(), "key", seed.as_bytes());
    let loaded = Signer::load(&path, history_in(dir.path())).expect("a key");
    let envelope = loaded
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    let history = signer::read_history(&history_in(dir.path())).expect("a history");
    signer::validate(&envelope, &history, SUBJECT).expect("the loaded key signs");
}

#[test]
fn each_fault_of_a_key_file_has_its_own_error_sg_06() {
    let dir = tempfile::tempdir().expect("a directory");
    let history = history_in(dir.path());
    let load = |path: &Path| Signer::load(path, history.clone()).err();
    assert_eq!(load(&dir.path().join("absent")), Some(KeyError::Missing));
    assert_eq!(load(dir.path()), Some(KeyError::Unreadable), "a directory");
    let seed = hex(&SEED_A);
    let cases: [(&str, Vec<u8>); 7] = [
        ("empty", Vec::new()),
        ("short", seed.as_bytes()[..63].to_vec()),
        ("long", format!("{seed}0").into_bytes()),
        ("not hexadecimal", format!("{}zz", &seed[..62]).into_bytes()),
        ("two lines", format!("{seed}\n{seed}").into_bytes()),
        ("not UTF-8", vec![0xff; 64]),
        (
            "a PEM",
            b"-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIA==\n-----END PRIVATE KEY-----\n"
                .to_vec(),
        ),
    ];
    for (name, content) in cases {
        let path = key_file(dir.path(), name, &content);
        assert_eq!(load(&path), Some(KeyError::Malformed), "{name}");
    }
}

#[test]
fn the_seed_appears_in_nothing_that_is_written_or_printed_sg_06() {
    let dir = tempfile::tempdir().expect("a directory");
    let seed = hex(&SEED_A);
    let path = key_file(dir.path(), "key", seed.as_bytes());
    let signer = Signer::load(&path, history_in(dir.path())).expect("a key");
    let envelope = signer
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    let history = std::fs::read(history_in(dir.path())).expect("a history");
    let printed = format!("{signer:?}");
    let seed_as_base64 = STANDARD.encode(SEED_A);
    for (where_, text) in [
        ("the debug output of the signer", printed.into_bytes()),
        ("the envelope", envelope),
        ("the history", history),
    ] {
        let text = String::from_utf8_lossy(&text).to_lowercase();
        assert!(!text.contains(&seed), "the seed is in {where_}");
        assert!(
            !text.contains(&seed_as_base64.to_lowercase()),
            "the seed, in base64, is in {where_}"
        );
        assert!(
            !text.contains(&seed[..16]),
            "a part of the seed is in {where_}"
        );
    }
    // The error of a malformed file does not repeat what the file held.
    let secretive = key_file(dir.path(), "bad", format!("{seed}ff").as_bytes());
    let error = Signer::load(&secretive, history_in(dir.path())).expect_err("a malformed key");
    let printed = format!("{error:?} {error}");
    assert!(!printed.contains(&seed[..16]), "{printed}");
}

// SG-07

#[test]
fn a_history_that_cannot_be_written_gives_no_envelope_sg_07() {
    let dir = tempfile::tempdir().expect("a directory");
    // The history is where a directory is.
    let path = dir.path().join("history");
    std::fs::create_dir(&path).expect("a directory");
    let signer = Signer::from_seed(SEED_A, path.clone());
    assert_eq!(
        signer.sign(SUBJECT, &predicate(), NOW),
        Err(SignError::History(HistoryError::Unreadable))
    );
    // The parent of the history is a file: it can be neither read nor written.
    let blocked = dir.path().join("blocked");
    std::fs::write(&blocked, b"a file").expect("a file");
    let signer = Signer::from_seed(SEED_A, blocked.join("history"));
    assert_eq!(
        signer.sign(SUBJECT, &predicate(), NOW),
        Err(SignError::History(HistoryError::Unreadable))
    );
    // The history reads as empty, and the temporary file it is written through is a directory:
    // the write fails after the read has succeeded.
    let walled = dir.path().join("walled");
    std::fs::create_dir(&walled).expect("a directory");
    std::fs::create_dir(walled.join("history.tmp")).expect("a directory");
    let signer = Signer::from_seed(SEED_A, walled.join("history"));
    assert_eq!(
        signer.sign(SUBJECT, &predicate(), NOW),
        Err(SignError::History(HistoryError::Unwritable))
    );
    assert!(!walled.join("history").exists(), "no history was written");
    // A history that is damaged is not repaired by a signature, and the signature is refused.
    let damaged = dir.path().join("damaged");
    std::fs::write(&damaged, b"[").expect("a file");
    let signer = Signer::from_seed(SEED_A, damaged.clone());
    assert_eq!(
        signer.sign(SUBJECT, &predicate(), NOW),
        Err(SignError::History(HistoryError::Malformed))
    );
    assert_eq!(std::fs::read(&damaged).expect("a file"), b"[");
}

#[test]
fn the_history_is_replaced_whole_and_no_temporary_file_is_left_sg_07() {
    let dir = tempfile::tempdir().expect("a directory");
    let path = history_in(dir.path());
    signer_in(dir.path(), SEED_A)
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    signer_in(dir.path(), SEED_B)
        .sign(SUBJECT, &predicate(), NOW)
        .expect("a signature");
    let names: Vec<String> = std::fs::read_dir(path.parent().expect("a parent"))
        .expect("a directory")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, ["history"], "no temporary file is left");
    let text = std::fs::read_to_string(&path).expect("a history");
    let list: Value = serde_json::from_str(&text).expect("a complete list");
    assert_eq!(list.as_array().expect("a list").len(), 2);
}
