//! SG-01 to SG-08: the signer of the verification records (issue #11, the host half that stands
//! without the verifier process).
//!
//! The behavior is tested where the host runs: `gateway/bindings/gateway-binding/tests/signing.rs`
//! signs, validates and damages envelopes with expected bytes made by code of its own. These tests
//! read the sources as text and hold them against each other: the rows against the tests that
//! answer them, every constructor of every public failure against a negative test (R5), the key
//! against anything that prints, and the planted defects against `SIGNING.md`. Nothing here runs
//! the signer.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const SIGNER: &str = "gateway/bindings/gateway-signer/src/signer.rs";
const TESTS: &str = "gateway/bindings/gateway-binding/tests/signing.rs";
const RECORDS: &str = "gateway/bindings/gateway-binding/tests/records.rs";
const VERIFIER: &str = "gateway/bindings/gateway-binding/src/verifier.rs";
const SERVICE: &str = "gateway/bindings/gateway-signer/src/service.rs";
const SERVICE_TESTS: &str = "gateway/bindings/gateway-signer/tests/service.rs";

/// The names of the `#[test]` functions of the binding's signing and record tests.
fn gateway_tests() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut in_test = false;
    let sources = [read(TESTS), read(RECORDS), read(SERVICE_TESTS)];
    for line in sources.iter().flat_map(|source| source.lines()) {
        let line = line.trim();
        if line == "#[test]" {
            in_test = true;
        } else if in_test && line.starts_with("fn ") {
            names.insert(line[3..].split('(').next().expect("a name").to_string());
            in_test = false;
        }
    }
    names
}

/// The tests of the binding that answer the row `id`, which must be exactly `expected`.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = id.to_lowercase().replace('-', "_");
    let found: BTreeSet<String> = gateway_tests()
        .into_iter()
        .filter(|name| name.ends_with(&format!("_{suffix}")))
        .collect();
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests of the binding that answer {id}");
    let document = read("SIGNING.md");
    assert!(document.contains(id), "SIGNING.md does not name {id}");
}

/// The constructors of `pub enum <name>` in the signer, as written.
fn constructors(source: &str, name: &str) -> Vec<String> {
    let head = format!("pub enum {name} {{");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("{SIGNER} has no {head}"))
        + head.len();
    let end = start + source[start..].find("\n}\n").expect("the end of the enum");
    source[start..end]
        .lines()
        .map(str::trim)
        .filter(|line| line.chars().next().is_some_and(|c| c.is_ascii_uppercase()))
        .map(|line| {
            line.split(['(', ',', ' '])
                .next()
                .expect("a name")
                .to_string()
        })
        .collect()
}

#[test]
fn the_envelope_is_answered_by_its_tests_sg_01() {
    answered_by(
        "SG-01",
        &[
            "the_envelope_is_dsse_over_an_in_toto_statement_sg_01",
            "the_keyid_is_the_digest_of_the_public_key_not_of_the_seed_sg_01",
        ],
    );
}

#[test]
fn the_primitives_are_answered_by_the_standards_vectors_sg_02() {
    answered_by(
        "SG-02",
        &[
            "pae_is_the_example_of_the_dsse_protocol_sg_02",
            "the_key_of_rfc_8032_test_1_has_its_public_key_and_signature_sg_02",
            "equal_inputs_give_equal_envelopes_sg_02",
        ],
    );
    let tests = read(TESTS);
    // The vectors are the standards' bytes, spelled out in the test and not computed from the signer.
    for published in [
        "DSSEv1 29 http://example.com/HelloWorld 11 hello world",
        "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
        "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
    ] {
        assert!(tests.contains(published), "the test lacks {published}");
    }
}

#[test]
fn the_validator_is_answered_by_its_tests_sg_03() {
    answered_by(
        "SG-03",
        &[
            "the_validator_accepts_what_the_signer_wrote_sg_03",
            "the_validator_uses_the_key_the_history_names_sg_03",
        ],
    );
}

#[test]
fn every_refusal_of_an_envelope_and_of_a_key_has_a_negative_test_sg_04() {
    answered_by(
        "SG-04",
        &[
            "every_refusal_of_an_envelope_is_typed_and_reached_sg_04",
            "a_valid_signature_beside_a_bad_one_is_accepted_and_a_bad_one_alone_is_not_sg_04",
        ],
    );
    // R5 for the whole module: every constructor of every public failure is named by a test.
    let source = read(SIGNER);
    let tests = read(TESTS) + &read(SERVICE_TESTS);
    for kind in ["EnvelopeError", "KeyError", "HistoryError", "SignError"] {
        let found = constructors(&source, kind);
        assert!(!found.is_empty(), "{kind} has no constructors");
        for constructor in found {
            assert!(
                tests.contains(&format!("{kind}::{constructor}")),
                "no test reaches {kind}::{constructor}"
            );
        }
    }
    assert_eq!(
        constructors(&source, "EnvelopeError").len(),
        9,
        "the nine refusals of the row"
    );
}

#[test]
fn the_history_is_answered_by_its_tests_sg_05() {
    answered_by(
        "SG-05",
        &[
            "the_first_use_of_a_key_adds_its_entry_and_later_uses_add_nothing_sg_05",
            "a_second_key_is_a_rotation_and_the_first_envelopes_stay_valid_sg_05",
            "a_revoked_key_fails_and_the_other_key_does_not_sg_05",
            "a_revoked_key_signs_nothing_sg_05",
            "a_history_that_is_not_a_list_of_good_entries_is_malformed_sg_05",
        ],
    );
}

#[test]
fn the_key_is_a_file_secret_that_nothing_prints_sg_06() {
    answered_by(
        "SG-06",
        &[
            "a_key_file_of_64_hexadecimal_digits_is_the_seed_sg_06",
            "each_fault_of_a_key_file_has_its_own_error_sg_06",
            "the_seed_appears_in_nothing_that_is_written_or_printed_sg_06",
        ],
    );
    let source = read(SIGNER);
    // The tests print nothing of the key; the module must not either, and the signer's Debug is
    // written by hand because a derived one would print the key.
    for printing in ["println!", "eprintln!", "dbg!", "print!"] {
        assert!(!source.contains(printing), "the signer uses {printing}");
    }
    let signer_head = source
        .find("pub struct Signer {")
        .expect("the struct of the signer");
    assert!(
        !source[signer_head.saturating_sub(120)..signer_head].contains("derive(Debug"),
        "Signer must not derive Debug: it holds the key"
    );
    assert!(
        source.contains("impl fmt::Debug for Signer"),
        "Signer has a Debug that is written by hand"
    );
}

#[test]
fn a_signature_that_cannot_be_recorded_is_not_returned_sg_07() {
    answered_by(
        "SG-07",
        &[
            "a_history_that_cannot_be_written_gives_no_envelope_sg_07",
            "the_history_is_replaced_whole_and_no_temporary_file_is_left_sg_07",
        ],
    );
    // The history is written before the envelope is made: the order is in the source.
    let source = read(SIGNER);
    let write = source
        .find("write_history(&self.history, &entries)")
        .expect("sign writes the history");
    let envelope = source
        .find("let signature = self.key.sign(")
        .expect("sign signs");
    assert!(
        write < envelope,
        "the history is written before the key signs"
    );
}

#[test]
fn each_gate_of_the_signer_rejects_its_planted_defect_sg_08() {
    let document = read("SIGNING.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("SIGNING.md has a section Planted defects");
    for defect in [
        "the lengths of PAE swapped",
        "the keyid taken from the seed",
        "the digest of the subject not checked",
        "a revoked key accepted",
        "the history not appended on the first use",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("SIGNING.md records no planted defect: {defect}"));
        // A row names the command, the diagnostic and the restore.
        assert!(
            row.contains("cargo test") && row.contains("sg_") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}

#[test]
fn a_verification_is_not_started_without_a_usable_key_sg_09() {
    answered_by(
        "SG-09",
        &[
            "the_variable_names_the_socket_and_an_empty_one_is_no_path_sg_09",
            "verify_start_without_a_usable_key_is_unavailable_and_starts_nothing_sg_09",
        ],
    );
    let config = read("gateway/bindings/gateway-binding/src/config.rs");
    assert!(
        config.contains("SIGNER_VARIABLE") && read(SERVICE).contains("\"LEXLEAN_MCP_SIGNER\""),
        "the socket of the signer is named by LEXLEAN_MCP_SIGNER"
    );
    // The project is judged first and the signer after, so that a project that is not one is
    // refused whatever the key, and the copy a draft makes is dropped when the key is unusable.
    let host = read("gateway/bindings/gateway-binding/src/lib.rs");
    let start = host.find("fn verify_start(").expect("verify_start");
    let body = &host[start..];
    let key = body
        .find("self.config.signer")
        .expect("verify_start asks the signer");
    let copy = body
        .find("project::resolve(")
        .expect("verify_start copies the project");
    assert!(copy < key, "the project is judged before the key");
}

#[test]
fn the_record_of_a_verification_is_the_signed_envelope_sg_10() {
    answered_by(
        "SG-10",
        &["a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10"],
    );
    let verifier = read(VERIFIER);
    // Every record is written by `conclude`, and the result of the engine reaches `conclude`.
    assert_eq!(
        verifier.matches("attestation::record(").count(),
        1,
        "a record is written in one place"
    );
    // The verdict of a process that verified reaches `conclude`, in `settle`.
    let settle = &verifier[verifier.find("fn settle(").expect("settle")..];
    assert!(
        settle.contains("conclude(&task.job"),
        "a verified verdict is concluded"
    );
}

#[test]
fn the_lookup_returns_the_envelope_sg_11() {
    answered_by(
        "SG-11",
        &["the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11"],
    );
}

#[test]
fn a_verification_that_cannot_be_signed_leaves_no_record_sg_12() {
    answered_by(
        "SG-12",
        &[
            "a_history_that_cannot_be_written_leaves_no_record_sg_12",
            "a_verified_directory_without_attestation_json_leaves_no_record_sg_12",
            "a_draft_that_does_not_verify_leaves_no_record_and_no_key_entry_sg_12",
        ],
    );
    // The order in `conclude`: the subject is read, then signed, then recorded.
    let verifier = read(VERIFIER);
    let conclude = &verifier[verifier.find("pub fn conclude(").expect("conclude")..];
    let read_at = conclude
        .find("attestation.json")
        .expect("conclude reads the subject");
    let sign = conclude
        .find("job.signer.sign_record(")
        .expect("conclude signs");
    let record = conclude
        .find("attestation::record(")
        .expect("conclude records");
    assert!(
        read_at < sign && sign < record,
        "read, then sign, then record"
    );
}

#[test]
fn the_record_keeps_its_evidence_sg_14() {
    answered_by(
        "SG-14",
        &["the_record_keeps_the_manifest_the_renderings_and_the_sources_sg_14"],
    );
    // The order in `record`: the subject, then the evidence, then the envelope.
    let source = read("gateway/bindings/gateway-binding/src/attestation.rs");
    let record = &source[source.find("pub fn record(").expect("record")..];
    let subject = record.find("attestation.json").expect("the subject");
    let evidence = record
        .find("for (key, bytes) in evidence")
        .expect("the evidence");
    let envelope = record.find("{attestation}.json").expect("the envelope");
    assert!(
        subject < evidence && evidence < envelope,
        "the subject, then the evidence, then the envelope"
    );
}

#[test]
fn evidence_that_cannot_be_kept_leaves_no_record_sg_15() {
    answered_by(
        "SG-15",
        &["evidence_that_cannot_be_kept_whole_leaves_no_record_sg_15"],
    );
    // The evidence is gathered before the subject is signed.
    let verifier = read(VERIFIER);
    let conclude = &verifier[verifier.find("pub fn conclude(").expect("conclude")..];
    let gather = conclude
        .find("evidence::gather(")
        .expect("conclude gathers the evidence");
    let sign = conclude
        .find("job.signer.sign_record(")
        .expect("conclude signs");
    assert!(gather < sign, "gather, then sign");
}

#[test]
fn each_gate_of_the_signed_record_rejects_its_planted_defect_sg_13() {
    let document = read("SIGNING.md");
    let section = document
        .split("## Planted defects of the signed record")
        .nth(1)
        .expect("SIGNING.md has a section Planted defects of the signed record");
    for defect in [
        "verify.start accepting a verification without a key",
        "the failure to sign ignored",
        "the envelope stored without its attestation.json",
        "the predicate without the digests of the source files",
        "the lookup answering the unsigned record again",
        "the manifest kept without its digest checked",
        "a rendering kept without its digest checked",
        "the evidence not written",
        "evidence that cannot be gathered ignored",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("SIGNING.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("sg_") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}

#[test]
fn the_signer_is_a_program_of_its_own_and_the_host_holds_no_key_sg_16() {
    answered_by(
        "SG-16",
        &[
            "the_program_answers_on_a_socket_of_mode_0600_sg_16",
            "the_program_replaces_the_socket_of_an_earlier_run_sg_16",
            "a_key_named_to_the_host_signs_nothing_sg_16",
        ],
    );
    // Nothing in the host's source can hold a key: the words a key would need are not there.
    let host = root().join("gateway/bindings/gateway-binding/src");
    let mut sources = Vec::new();
    for directory in [host.clone(), host.join("bin")] {
        for entry in std::fs::read_dir(&directory).expect("the source of the host") {
            let path = entry.expect("an entry").path();
            if path.is_file() {
                sources.push(path);
            }
        }
    }
    assert!(sources.len() > 10, "the source of the host was read");
    for path in &sources {
        let source = std::fs::read_to_string(path).expect("a source");
        for word in [
            "Signer::load",
            "Signer::from_seed",
            "ed25519",
            "signing_key",
        ] {
            assert!(
                !source.contains(word),
                "{} holds `{word}`: the host holds no key",
                path.display()
            );
        }
    }

    let dependencies_of = |manifest: &str| -> BTreeSet<String> {
        let text = read(manifest);
        text.split("[dependencies]")
            .nth(1)
            .and_then(|rest| rest.split("\n[").next())
            .expect("the dependencies")
            .lines()
            .filter_map(|line| line.split(" = ").next())
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .collect()
    };
    assert!(
        !dependencies_of("gateway/bindings/gateway-binding/Cargo.toml").contains("ed25519-dalek"),
        "the host links no signature library outside its tests"
    );
    // The crate of the key links exactly these, so no compiler and nothing generated.
    let expected: BTreeSet<String> = ["base64", "ed25519-dalek", "serde_json", "sha2"]
        .map(str::to_owned)
        .into();
    assert_eq!(
        dependencies_of("gateway/bindings/gateway-signer/Cargo.toml"),
        expected,
        "the crate that holds the key"
    );
    assert!(
        read("gateway/Cargo.toml").contains("\"bindings/gateway-signer\""),
        "the crate is a member of the gateway workspace"
    );

    // The key file is read for each request: the service has a path for it, and no field for a key.
    let service = read(SERVICE);
    let head = service.find("pub struct Service {").expect("the service");
    let fields = &service[head..head + service[head..].find('}').expect("its end")];
    assert!(
        fields.contains("key: PathBuf") && !fields.contains("Signer"),
        "the service holds the path of the key, not the key: {fields}"
    );
}

#[test]
fn an_envelope_through_the_socket_is_the_signers_sg_17() {
    answered_by(
        "SG-17",
        &[
            "an_envelope_through_the_socket_is_the_envelope_of_the_signer_sg_17",
            "a_key_file_put_right_or_rotated_is_used_at_the_next_request_sg_17",
            "a_verification_concluded_through_the_socket_is_a_valid_record_sg_17",
        ],
    );
}

#[test]
fn every_failure_of_the_service_is_typed_and_tested_sg_18() {
    answered_by(
        "SG-18",
        &[
            "every_refusal_travels_by_its_name_sg_18",
            "each_fault_of_the_key_file_comes_back_as_itself_sg_18",
            "a_revoked_key_is_refused_through_the_socket_sg_18",
            "a_history_that_cannot_be_written_is_refused_through_the_socket_sg_18",
            "a_request_that_is_not_one_is_refused_and_the_next_is_served_sg_18",
            "a_request_over_the_bound_is_refused_and_the_next_is_served_sg_18",
            "no_answer_at_the_socket_is_unavailable_sg_18",
            "an_answer_that_is_not_one_is_unavailable_sg_18",
            "an_envelope_that_is_not_one_is_unavailable_sg_18",
            "each_start_error_stops_the_program_with_status_2_sg_18",
            "a_signer_lost_before_the_verification_concludes_leaves_no_record_sg_18",
        ],
    );
    let tests = read(SERVICE_TESTS);
    let service = read(SERVICE);
    let names = constructors(&service, "StartError");
    assert_eq!(names.len(), 5, "the constructors of StartError: {names:?}");
    for name in names {
        assert!(
            tests.contains(&format!("\"{name}\"")),
            "StartError::{name} has no negative test"
        );
    }
    // Only the program prints: the line that it listens and the error it stopped for.
    for printing in ["eprintln!", "dbg!"] {
        assert!(!service.contains(printing), "the service uses {printing}");
    }
}

#[test]
fn each_gate_of_the_signer_service_rejects_its_planted_defect_sg_19() {
    let document = read("SIGNING.md");
    let section = document
        .split("## Planted defects of the signer service")
        .nth(1)
        .expect("SIGNING.md has a section Planted defects of the signer service");
    for defect in [
        "the host loading the key itself",
        "the socket left open to other users",
        "the key cached at the start of the program",
        "a refusal name that comes back as another error",
        "the bound on a request removed",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("SIGNING.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("sg_") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}
