//! DR-01 to DR-10: the draft tools of F3 (issue #9). Each test calls `lexlean_draft_write`,
//! `lexlean_draft_read` or an Engine tool through the production core with the real host
//! (`gateway_binding::Native`), on a store in a temporary directory, as a client does. Nothing
//! here knows what a tool does: the core does, and the model (`src/Identity.lex.tex`) is the
//! oracle of the identity of a draft.

use std::path::Path;

use gateway_tool_tests::{tiny, Gateway, Reply, BUNDLE, SOURCE};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

/// The ID of the profile of these tests: 32 opaque octets, written as 64 hexadecimal digits.
const PROFILE_ID: &str = "ad66a1c0c9fa6ad8b6bf64559be0c949e8141a0fa9ad64b80978f57fc83817c2";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// A gateway whose store holds the profile `tiny`, with `src` as its only source root.
fn with_profile() -> Gateway {
    let gateway = Gateway::new();
    gateway.provision("tiny", &Gateway::record(PROFILE_ID));
    gateway
}

fn files(rows: &[(&str, &str)]) -> Value {
    Value::Array(
        rows.iter()
            .map(|(path, content)| json!({"path": path, "content": content}))
            .collect(),
    )
}

fn write(gateway: &mut Gateway, profile: &str, rows: &[(&str, &str)]) -> Reply {
    gateway.call(
        "lexlean_draft_write",
        json!({"profile": profile, "files": files(rows)}),
    )
}

fn read(gateway: &mut Gateway, draft: &str, path: Option<&str>) -> Reply {
    let mut arguments = json!({"draft": draft});
    if let Some(path) = path {
        arguments["path"] = json!(path);
    }
    gateway.call("lexlean_draft_read", arguments)
}

/// The handle of a draft that was written.
fn handle(reply: &Reply) -> String {
    assert!(!reply.is_error(), "a written draft: {}", reply.body);
    reply.structured()["draft"]
        .as_str()
        .expect("a handle")
        .to_string()
}

/// A call that the draft tools refuse by name: an error result whose structured content is the
/// refusal, with the effects it asked for.
fn assert_refusal(reply: &Reply, name: &str, effects: &[&str]) {
    assert!(reply.is_error(), "{name}: {}", reply.body);
    assert_eq!(reply.structured(), &json!({"refusal": name}), "{name}");
    assert_eq!(reply.effects, effects, "{name}: the effects of the call");
}

/// The host failure of an effect: an error result `{failure: <name>}`.
fn assert_failure(reply: &Reply, name: &str, effects: &[&str]) {
    assert!(reply.is_error(), "{name}: {}", reply.body);
    assert_eq!(reply.structured(), &json!({"failure": name}), "{name}");
    assert_eq!(reply.effects, effects, "{name}: the effects of the call");
}

fn stored_drafts(gateway: &Gateway) -> Vec<String> {
    let mut found = Vec::new();
    let Ok(profiles) = std::fs::read_dir(gateway.drafts()) else {
        return found;
    };
    for profile in profiles.flatten() {
        for draft in std::fs::read_dir(profile.path()).expect("a profile").flatten() {
            found.push(format!(
                "{}/{}",
                profile.file_name().to_string_lossy(),
                draft.file_name().to_string_lossy()
            ));
        }
    }
    found.sort();
    found
}

// ---- the model's vectors ---------------------------------------------------------------------

/// The statement of the theorem `name` in `src/Identity.lex.tex`.
fn model_statement(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/Identity.lex.tex");
    let text = std::fs::read_to_string(&path).expect("src/Identity.lex.tex");
    let theorem = text
        .split(&format!("name: {name},"))
        .nth(1)
        .unwrap_or_else(|| panic!("the theorem {name}"));
    theorem
        .split("proof:")
        .next()
        .expect("a statement")
        .to_string()
}

/// The words `hN := (V : UInt32)` of a statement, in order.
fn digest_words(statement: &str) -> Vec<u32> {
    statement
        .split("h")
        .skip(1)
        .filter_map(|part| {
            let rest = part.strip_prefix(|c: char| c.is_ascii_digit())?;
            let value = rest.strip_prefix(" := (")?;
            value.split(' ').next()?.parse().ok()
        })
        .collect()
}

fn digest_hex(words: &[u32]) -> String {
    words
        .iter()
        .map(|word| format!("{word:08x}"))
        .collect::<String>()
}

/// The preimage (`draftPreimageVector`) and the identity (`draftIdVector`) of the model, and the
/// ID of the profile they are for.
fn model_vector() -> (String, String, String) {
    let preimage = model_statement("draftPreimageVector");
    let octets = preimage
        .split("octetsOfBytes 0x")
        .nth(1)
        .expect("the expected octets")
        .split(')')
        .next()
        .expect("hexadecimal digits")
        .to_string();
    let words = digest_words(&model_statement("draftIdVector"));
    assert_eq!(words.len(), 16, "the profile digest and the draft digest");
    (octets, digest_hex(&words[8..]), digest_hex(&words[..8]))
}

// ---- DR-01 -------------------------------------------------------------------------------------

#[test]
fn the_core_reproduces_the_models_draft_identity_dr_01() {
    let (preimage, id, profile) = model_vector();
    assert_eq!(profile, PROFILE_ID, "the profile of the model's vector");
    let mut gateway = with_profile();
    let reply = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", "x"), ("src/B.lex.tex", "yz")],
    );
    assert_eq!(handle(&reply), format!("tiny/{id}"), "{}", reply.body);
    assert_eq!(reply.structured()["id"], id.as_str());
    assert_eq!(reply.effects, ["lexicon.read", "store.put"]);
    let stored = std::fs::read(gateway.drafts().join("tiny").join(&id)).expect("the stored draft");
    assert_eq!(hex(&stored), preimage, "the stored preimage is the model's");
    assert_eq!(sha256(&stored), id, "and its SHA-256 is the identity");
}

#[test]
fn the_rows_of_a_draft_in_any_order_have_one_identity_dr_01() {
    let mut gateway = with_profile();
    let forward = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", "x"), ("src/B.lex.tex", "yz")],
    );
    let backward = write(
        &mut gateway,
        "tiny",
        &[("src/B.lex.tex", "yz"), ("src/A.lex.tex", "x")],
    );
    assert_eq!(handle(&forward), handle(&backward));
    assert_eq!(stored_drafts(&gateway).len(), 1, "one key");
}

// ---- DR-02 -------------------------------------------------------------------------------------

#[test]
fn the_same_files_under_two_profiles_have_two_identities_dr_02() {
    let mut gateway = with_profile();
    gateway.provision("other", &Gateway::record(&"11".repeat(32)));
    let rows = [("src/A.lex.tex", "x")];
    let first = handle(&write(&mut gateway, "tiny", &rows));
    let second = handle(&write(&mut gateway, "other", &rows));
    assert_ne!(
        first.split('/').nth(1),
        second.split('/').nth(1),
        "the identity depends on the profile"
    );
    assert!(first.starts_with("tiny/") && second.starts_with("other/"));
}

#[test]
fn a_byte_and_a_path_each_change_the_identity_dr_02() {
    let mut gateway = with_profile();
    let base = handle(&write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]));
    let byte = handle(&write(&mut gateway, "tiny", &[("src/A.lex.tex", "y")]));
    let renamed = handle(&write(&mut gateway, "tiny", &[("src/B.lex.tex", "x")]));
    let more = handle(&write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", "x"), ("src/B.lex.tex", "")],
    ));
    let all = [&base, &byte, &renamed, &more];
    for (index, one) in all.iter().enumerate() {
        for other in &all[index + 1..] {
            assert_ne!(one, other);
        }
    }
}

#[test]
fn a_draft_written_twice_is_one_key_of_the_drafts_area_dr_02() {
    let mut gateway = with_profile();
    let rows = [("src/A.lex.tex", "x"), ("src/deep/B.lex.tex", "y")];
    let first = handle(&write(&mut gateway, "tiny", &rows));
    let second = handle(&write(&mut gateway, "tiny", &rows));
    assert_eq!(first, second);
    assert_eq!(stored_drafts(&gateway), [first]);
}

// ---- DR-03 (AC-09) -----------------------------------------------------------------------------

/// The call is refused with `refusal` after the lexicon.read of the profile alone: nothing was
/// stored and no Engine tool ran.
fn assert_refused_before_the_store(path: &str, refusal: &str) {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[(path, "x")]);
    assert_refusal(&reply, refusal, &["lexicon.read"]);
    assert!(
        stored_drafts(&gateway).is_empty(),
        "{path}: nothing is stored"
    );
    assert!(
        reply.effects.iter().all(|effect| !effect.starts_with("engine.")),
        "{path}: no Engine call"
    );
}

#[test]
fn a_path_with_a_dot_dot_component_is_refused_before_the_store_and_the_engine_dr_03() {
    assert_refused_before_the_store("src/../A.lex.tex", "DotDotComponent");
    assert_refused_before_the_store("../src/A.lex.tex", "DotDotComponent");
    assert_refused_before_the_store("src/a/../../A.lex.tex", "DotDotComponent");
}

#[test]
fn a_path_that_is_not_lex_tex_is_refused_before_the_store_and_the_engine_dr_03() {
    assert_refused_before_the_store("src/A.txt", "NotLexTex");
    assert_refused_before_the_store("src/A.lex", "NotLexTex");
    assert_refused_before_the_store("src/lex.tex", "NotLexTex");
}

#[test]
fn a_path_that_is_not_nfc_is_refused_before_the_store_and_the_engine_dr_03() {
    // `e` and a combining acute accent: the same text as U+00E9, but not in normal form C.
    assert_refused_before_the_store("src/e\u{301}.lex.tex", "NotNfc");
    // A Hangul syllable in its decomposed form.
    assert_refused_before_the_store("src/\u{1112}\u{1161}\u{11ab}.lex.tex", "NotNfc");
}

#[test]
fn a_path_the_form_of_which_admission_refuses_is_refused_before_the_store_dr_03() {
    assert_refused_before_the_store("/src/A.lex.tex", "AbsolutePath");
    assert_refused_before_the_store("src//A.lex.tex", "EmptyComponent");
    assert_refused_before_the_store("src/./A.lex.tex", "DotComponent");
    assert_refused_before_the_store("lib/A.lex.tex", "OutsideRoots");
    assert_refused_before_the_store("src.lex.tex", "OutsideRoots");
    assert_refused_before_the_store("srcs/A.lex.tex", "OutsideRoots");
}

#[test]
fn a_path_that_is_admitted_is_stored_with_the_lexicon_read_and_the_put_dr_03() {
    let mut gateway = with_profile();
    for path in ["src/A.lex.tex", "src/a/b/c.lex.tex", "src/..a.lex.tex", "src/.lex.tex.lex.tex"] {
        let reply = write(&mut gateway, "tiny", &[(path, "x")]);
        assert!(!reply.is_error(), "{path}: {}", reply.body);
        assert_eq!(reply.effects, ["lexicon.read", "store.put"], "{path}");
    }
}

// ---- DR-04 -------------------------------------------------------------------------------------

#[test]
fn a_draft_of_256_files_is_written_and_a_257th_is_refused_dr_04() {
    let mut gateway = with_profile();
    let names: Vec<String> = (0..257).map(|n| format!("src/F{n:03}.lex.tex")).collect();
    let rows: Vec<(&str, &str)> = names.iter().map(|n| (n.as_str(), "")).collect();
    let reply = write(&mut gateway, "tiny", &rows[..256]);
    assert!(!reply.is_error(), "256 files: {}", reply.body);
    assert_eq!(reply.structured()["files"], 256);
    let reply = write(&mut gateway, "tiny", &rows);
    assert_refusal(&reply, "TooManyFiles", &["lexicon.read"]);
}

#[test]
fn a_draft_of_8_mib_is_written_and_one_byte_more_is_refused_dr_04() {
    let mut gateway = with_profile();
    let at_the_limit = "a".repeat(8_388_608);
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", &at_the_limit)]);
    assert!(!reply.is_error(), "8388608 bytes: {}", reply.body);
    assert_eq!(reply.structured()["bytes"], 8_388_608);
    let id = handle(&reply);
    let stored = std::fs::read(gateway.drafts().join(&id)).expect("the stored draft");
    assert!(stored.len() > 8_388_608, "the preimage is a little larger");
    let over = "a".repeat(8_388_609);
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", &over)]);
    assert_refusal(&reply, "TooManyBytes", &["lexicon.read"]);
}

#[test]
fn the_bytes_of_a_draft_are_counted_over_all_its_files_dr_04() {
    let mut gateway = with_profile();
    let half = "b".repeat(4_194_304);
    let reply = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", &half), ("src/B.lex.tex", &half)],
    );
    assert!(!reply.is_error(), "two halves: {}", reply.body);
    let reply = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", &half), ("src/B.lex.tex", &half), ("src/C.lex.tex", "c")],
    );
    assert_refusal(&reply, "TooManyBytes", &["lexicon.read"]);
}

#[test]
fn a_repeated_path_is_refused_dr_04() {
    let mut gateway = with_profile();
    let reply = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", "x"), ("src/B.lex.tex", "y"), ("src/A.lex.tex", "z")],
    );
    assert_refusal(&reply, "DuplicatePath", &["lexicon.read"]);
}

// ---- DR-05 -------------------------------------------------------------------------------------

#[test]
fn a_draft_is_the_one_key_of_its_profile_and_its_content_is_the_preimage_dr_05() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "alpha"), ("src/B.lex.tex", "beta")]);
    let draft = handle(&reply);
    assert_eq!(stored_drafts(&gateway), [draft.clone()], "one key, nothing else");
    let stored = std::fs::read(gateway.drafts().join(&draft)).expect("the stored draft");
    assert_eq!(draft, format!("tiny/{}", sha256(&stored)));
    assert!(
        stored.starts_with(b"\0\0\0\x06domain\0\0\0\0\0\0\0\x14lexlean-mcp-draft-v1"),
        "the preimage begins with its domain"
    );
}

#[test]
fn the_text_and_the_listing_of_a_draft_are_read_back_dr_05() {
    let mut gateway = with_profile();
    let draft = handle(&write(
        &mut gateway,
        "tiny",
        &[("src/B.lex.tex", "beta \u{e9}\n"), ("src/A.lex.tex", "alpha")],
    ));
    let file = read(&mut gateway, &draft, Some("src/B.lex.tex"));
    assert!(!file.is_error(), "{}", file.body);
    assert_eq!(file.text(), "beta \u{e9}\n");
    assert_eq!(file.structured(), &json!({"path": "src/B.lex.tex", "bytes": 8}));
    assert_eq!(file.effects, ["store.get"]);
    let listing = read(&mut gateway, &draft, None);
    assert!(!listing.is_error(), "{}", listing.body);
    assert_eq!(
        listing.structured(),
        &json!({
            "files": [
                {"path": "src/A.lex.tex", "bytes": 5},
                {"path": "src/B.lex.tex", "bytes": 8},
            ],
            "bytes": 13,
        })
    );
    assert_eq!(listing.text(), "src/A.lex.tex\nsrc/B.lex.tex");
    assert_eq!(listing.effects, ["store.get"]);
}

#[test]
fn an_empty_file_and_a_large_file_are_read_back_whole_dr_05() {
    let mut gateway = with_profile();
    let large = "x".repeat(300_000);
    let draft = handle(&write(
        &mut gateway,
        "tiny",
        &[("src/Empty.lex.tex", ""), ("src/Large.lex.tex", &large)],
    ));
    let empty = read(&mut gateway, &draft, Some("src/Empty.lex.tex"));
    assert!(!empty.is_error(), "{}", empty.body);
    assert_eq!(empty.text(), "");
    let whole = read(&mut gateway, &draft, Some("src/Large.lex.tex"));
    assert_eq!(whole.text(), large);
}

// ---- DR-06 -------------------------------------------------------------------------------------

#[test]
fn the_profile_record_is_read_by_the_first_effect_of_a_write_dr_06() {
    for record in [
        format!("{PROFILE_ID}\nsrc\n"),
        format!("{PROFILE_ID}\nsrc"),
        format!("{PROFILE_ID}\nlib\nsrc\n"),
        format!("{PROFILE_ID}\nlib/deep\nsrc/nested\n"),
    ] {
        let mut gateway = Gateway::new();
        gateway.provision("tiny", &record);
        let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]);
        assert!(!reply.is_error(), "{record:?}: {}", reply.body);
        assert_eq!(reply.effects, ["lexicon.read", "store.put"], "{record:?}");
    }
}

#[test]
fn a_second_root_admits_its_files_dr_06() {
    let mut gateway = Gateway::new();
    gateway.provision("tiny", &format!("{PROFILE_ID}\nsrc\nlib\n"));
    let reply = write(
        &mut gateway,
        "tiny",
        &[("src/A.lex.tex", "x"), ("lib/B.lex.tex", "y")],
    );
    assert!(!reply.is_error(), "{}", reply.body);
    let reply = write(&mut gateway, "tiny", &[("other/B.lex.tex", "y")]);
    assert_refusal(&reply, "OutsideRoots", &["lexicon.read"]);
}

#[test]
fn an_unknown_profile_is_the_host_failure_profile_unknown_dr_06() {
    let mut gateway = Gateway::new();
    let reply = write(&mut gateway, "absent", &[("src/A.lex.tex", "x")]);
    assert_failure(&reply, "profile-unknown", &["lexicon.read"]);
    assert_eq!(reply.text(), "lexicon.read failed: profile-unknown");
    // A profile name that is not a name is no profile either.
    let reply = write(&mut gateway, "../tiny", &[("src/A.lex.tex", "x")]);
    assert_failure(&reply, "profile-unknown", &["lexicon.read"]);
}

#[test]
fn a_record_that_is_not_a_profile_record_is_refused_without_a_put_dr_06() {
    let short = &PROFILE_ID[..62];
    let long = format!("{PROFILE_ID}00");
    let upper = PROFILE_ID.to_uppercase();
    let bad = format!("{}zz", &PROFILE_ID[..62]);
    let records = [
        String::new(),
        "\n".to_string(),
        format!("{short}\nsrc\n"),
        format!("{long}\nsrc\n"),
        format!("{upper}\nsrc\n"),
        format!("{bad}\nsrc\n"),
        format!("{PROFILE_ID}\n"),
        PROFILE_ID.to_string(),
        format!("{PROFILE_ID}\n/src\n"),
        format!("{PROFILE_ID}\nsrc/\n"),
        format!("{PROFILE_ID}\nsrc/../x\n"),
        format!("{PROFILE_ID}\nsrc\n\n"),
        format!("{PROFILE_ID}\n\nsrc\n"),
        format!("{PROFILE_ID} \nsrc\n"),
    ];
    for record in records {
        let mut gateway = Gateway::new();
        gateway.provision("tiny", &record);
        let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]);
        assert_refusal(&reply, "ProfileRecordMalformed", &["lexicon.read"]);
        assert!(stored_drafts(&gateway).is_empty(), "{record:?}: nothing stored");
    }
}

#[test]
fn a_profile_without_a_record_is_refused_without_a_put_dr_06() {
    let mut gateway = Gateway::new();
    gateway.provision("tiny", &Gateway::record(PROFILE_ID));
    std::fs::remove_file(gateway.profile("tiny").join("profile")).expect("a record");
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]);
    assert_refusal(&reply, "ProfileRecordMalformed", &["lexicon.read"]);
}

// ---- DR-07 -------------------------------------------------------------------------------------

/// A gateway that holds the tiny project as a directory project and as a profile with a draft.
fn with_tiny_both_ways() -> (Gateway, String) {
    let mut gateway = with_profile();
    gateway.install("dir", &tiny());
    let source = std::fs::read_to_string(tiny().join(SOURCE)).expect("the source");
    let draft = handle(&write(&mut gateway, "tiny", &[(SOURCE, &source)]));
    (gateway, draft)
}

fn builds(gateway: &Gateway) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(gateway.store().join("builds")) else {
        return Vec::new();
    };
    entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn an_engine_tool_on_the_handle_of_a_draft_agrees_with_the_directory_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    for tool in ["lexlean_check", "lexlean_snapshot", "lexlean_build"] {
        let by_handle = gateway.call(tool, json!({"project": draft, "files": []}));
        let by_directory = gateway.call(tool, json!({"project": "dir", "files": []}));
        assert!(!by_handle.is_error(), "{tool}: {}", by_handle.body);
        assert_eq!(by_handle.effects, [tool.replace("lexlean_", "engine.")]);
        assert_eq!(
            by_handle.structured(),
            by_directory.structured(),
            "{tool}: the result of the draft is the result of the directory"
        );
    }
    let by_handle = gateway.call(
        "lexlean_format",
        json!({"project": draft, "files": [], "check_only": true}),
    );
    let by_directory = gateway.call(
        "lexlean_format",
        json!({"project": "dir", "files": [], "check_only": true}),
    );
    assert_eq!(by_handle.structured(), by_directory.structured());
}

#[test]
fn the_directory_of_an_engine_call_is_gone_after_it_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert!(builds(&gateway).is_empty(), "{:?}", builds(&gateway));
}

#[test]
fn a_draft_the_engine_refuses_is_the_refusal_of_the_engine_dr_07() {
    let mut gateway = with_profile();
    let refused = "\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n";
    let draft = handle(&write(&mut gateway, "tiny", &[(SOURCE, refused)]));
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert!(reply.is_error(), "{}", reply.body);
    assert_eq!(reply.structured()["success"], false);
    assert_eq!(reply.effects, ["engine.check"]);
}

#[test]
fn a_handle_whose_draft_or_profile_is_absent_is_project_refused_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    let id = draft.split('/').nth(1).expect("an identity").to_string();
    let zeros = "0".repeat(64);
    for project in [
        format!("tiny/{zeros}"),
        format!("absent/{id}"),
        format!("other/{id}"),
    ] {
        let reply = gateway.call("lexlean_check", json!({"project": project, "files": []}));
        assert_failure(&reply, "project-refused", &["engine.check"]);
    }
    // The draft is there, but the profile has gone: the record is the proof that it is this one.
    std::fs::remove_dir_all(gateway.profile("tiny")).expect("a profile");
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert_failure(&reply, "project-refused", &["engine.check"]);
}

#[test]
fn a_draft_of_another_profile_than_its_record_is_project_refused_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    // The profile is provisioned again under another ID: the draft is no longer of it.
    gateway.provision("tiny", &Gateway::record(&"22".repeat(32)));
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert_failure(&reply, "project-refused", &["engine.check"]);
}

#[test]
fn a_changed_draft_is_project_refused_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    let path = gateway.drafts().join(&draft);
    let mut bytes = std::fs::read(&path).expect("the draft");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&path, bytes).expect("a draft");
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert_failure(&reply, "project-refused", &["engine.check"]);
}

#[test]
fn a_verification_of_a_draft_starts_on_a_copy_dr_07() {
    let (mut gateway, draft) = with_tiny_both_ways();
    let reply = gateway.call("lexlean_verify", json!({"project": draft, "files": []}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(reply.effects, ["verify.start"]);
    assert!(reply.structured()["task"].as_str().is_some());
}

// ---- DR-08 (AC-09) -----------------------------------------------------------------------------

/// A symbolic link in a bundle: `place` is where it lies below the profile, `target` what it
/// points at. The draft of the profile is refused as `project-refused`, and the target is never
/// opened (a target that is a pipe would block the host).
fn assert_a_link_is_refused(label: &str, place: &str, target: &Path, make_directory: bool) {
    let (mut gateway, draft) = with_tiny_both_ways();
    let link = gateway.profile("tiny").join(place);
    if make_directory {
        std::fs::create_dir_all(link.parent().expect("a parent")).expect("a directory");
    }
    std::os::unix::fs::symlink(target, &link).expect("a link");
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert_failure(&reply, "project-refused", &["engine.check"]);
    assert!(builds(&gateway).is_empty(), "{label}: nothing is left behind");
    let verify = gateway.call("lexlean_verify", json!({"project": draft, "files": []}));
    assert_failure(&verify, "project-refused", &["verify.start"]);
}

#[test]
fn a_profile_bundle_with_a_link_to_a_file_is_project_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    let target = outside.path().join("secret");
    std::fs::write(&target, "do not read").expect("a file");
    assert_a_link_is_refused("file", "linked.toml", &target, false);
    // Inside the bundle too: a link is refused whatever it points at.
    assert_a_link_is_refused("inside", "linked.toml", Path::new("lexlean.toml"), false);
}

#[test]
fn a_profile_bundle_with_a_link_to_a_directory_is_project_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    assert_a_link_is_refused("directory", "linked", outside.path(), false);
    assert_a_link_is_refused("itself", "again", Path::new("."), false);
}

#[test]
fn a_profile_bundle_with_a_dangling_link_is_project_refused_dr_08() {
    assert_a_link_is_refused("dangling", "dangling.toml", Path::new("/nonexistent/target"), false);
}

#[test]
fn a_profile_bundle_with_a_link_below_a_subdirectory_is_project_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    let target = outside.path().join("secret");
    std::fs::write(&target, "do not read").expect("a file");
    assert_a_link_is_refused("below", "language/deep/linked.toml", &target, true);
}

#[test]
fn no_byte_is_read_through_a_link_of_a_bundle_dr_08() {
    // A pipe that nothing writes to: opening it for reading would block the host for ever.
    let outside = tempfile::tempdir().expect("a directory");
    let pipe = outside.path().join("pipe");
    let status = std::process::Command::new("mkfifo")
        .arg(&pipe)
        .status()
        .expect("mkfifo");
    assert!(status.success());
    assert_a_link_is_refused("pipe", "pipe.toml", &pipe, false);
}

#[test]
fn a_profile_bundle_without_a_link_is_materialized_dr_08() {
    let (mut gateway, draft) = with_tiny_both_ways();
    let reply = gateway.call("lexlean_check", json!({"project": draft, "files": []}));
    assert!(!reply.is_error(), "{}", reply.body);
}

// ---- DR-09 -------------------------------------------------------------------------------------

#[test]
fn the_two_draft_tools_are_listed_with_closed_schemas_and_no_assemble_dr_09() {
    let mut gateway = Gateway::new();
    let reply = gateway.list();
    let tools = reply.result()["tools"].as_array().expect("tools").clone();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().expect("a name")).collect();
    assert!(names.contains(&"lexlean_draft_write"), "{names:?}");
    assert!(names.contains(&"lexlean_draft_read"), "{names:?}");
    assert!(
        !names.iter().any(|name| name.contains("assemble")),
        "the four functions of lexlean::sdk that no effect reaches have no tool: {names:?}"
    );
    let tool = |name: &str| tools.iter().find(|t| t["name"] == name).expect("a tool").clone();
    let written = tool("lexlean_draft_write");
    assert_eq!(written["inputSchema"]["additionalProperties"], false);
    assert_eq!(written["inputSchema"]["required"], json!(["profile", "files"]));
    assert_eq!(written["inputSchema"]["properties"]["files"]["type"], "array");
    let item = &written["inputSchema"]["properties"]["files"]["items"];
    assert_eq!(item["additionalProperties"], false);
    assert_eq!(item["required"], json!(["path", "content"]));
    let read_tool = tool("lexlean_draft_read");
    assert_eq!(read_tool["inputSchema"]["additionalProperties"], false);
    assert_eq!(read_tool["inputSchema"]["required"], json!(["draft"]));
}

#[test]
fn the_effects_of_the_draft_tools_are_the_ones_stated_dr_09() {
    let mut gateway = with_profile();
    let written = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]);
    assert_eq!(written.effects, ["lexicon.read", "store.put"]);
    let draft = handle(&written);
    assert_eq!(read(&mut gateway, &draft, None).effects, ["store.get"]);
    assert_eq!(
        read(&mut gateway, &draft, Some("src/A.lex.tex")).effects,
        ["store.get"]
    );
}

#[test]
fn a_call_that_breaks_a_schema_of_the_draft_tools_asks_for_no_effect_dr_09() {
    let mut gateway = with_profile();
    let calls = [
        ("lexlean_draft_write", json!({})),
        ("lexlean_draft_write", json!({"profile": "tiny"})),
        ("lexlean_draft_write", json!({"files": []})),
        ("lexlean_draft_write", json!({"profile": "tiny", "files": "src"})),
        ("lexlean_draft_write", json!({"profile": "tiny", "files": [{"path": "src/A.lex.tex"}]})),
        ("lexlean_draft_write", json!({"profile": "tiny", "files": [{"path": 1, "content": ""}]})),
        ("lexlean_draft_write", json!({"profile": "tiny", "files": [], "extra": 1})),
        ("lexlean_draft_write", json!({"profile": "", "files": []})),
        ("lexlean_draft_read", json!({})),
        ("lexlean_draft_read", json!({"draft": 3})),
        ("lexlean_draft_read", json!({"draft": "tiny/x", "path": ""})),
        ("lexlean_draft_read", json!({"draft": "tiny/x", "extra": true})),
    ];
    for (tool, arguments) in calls {
        let reply = gateway.call(tool, arguments.clone());
        let result = reply.result();
        assert!(reply.is_error(), "{tool} {arguments}: {}", reply.body);
        assert!(
            result["content"][0]["text"]
                .as_str()
                .is_some_and(|text| text.starts_with("Invalid arguments:")),
            "{tool} {arguments}: {}",
            reply.body
        );
        assert!(reply.effects.is_empty(), "{tool} {arguments}");
    }
}

// ---- DR-10 -------------------------------------------------------------------------------------

#[test]
fn not_utf8_is_refused_by_the_json_reader_before_the_tool_is_reached_dr_10() {
    // `Refusal.NotUtf8` is the verdict for octets that are not UTF-8. A path reaches the tool as a
    // JSON string, and the reader refuses a body with invalid UTF-8 or a lone surrogate before
    // any tool runs: the constructor is covered at the exported admission (DL-03).
    let mut gateway = with_profile();
    let body = br#"{"profile":"tiny","files":[{"path":"src/\ud800.lex.tex","content":""}]}"#;
    let reply = gateway.call_raw_arguments("lexlean_draft_write", body);
    assert!(reply.effects.is_empty(), "no effect is asked for");
    assert!(
        reply.body.get("error").is_some() || reply.is_error(),
        "refused: {}",
        reply.body
    );
}

#[test]
fn dot_dot_component_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/../A.lex.tex", "x")]);
    assert_refusal(&reply, "DotDotComponent", &["lexicon.read"]);
}

#[test]
fn not_nfc_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/e\u{301}.lex.tex", "x")]);
    assert_refusal(&reply, "NotNfc", &["lexicon.read"]);
}

#[test]
fn absolute_path_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("/src/A.lex.tex", "x")]);
    assert_refusal(&reply, "AbsolutePath", &["lexicon.read"]);
}

#[test]
fn empty_component_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src//A.lex.tex", "x")]);
    assert_refusal(&reply, "EmptyComponent", &["lexicon.read"]);
}

#[test]
fn dot_component_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/./A.lex.tex", "x")]);
    assert_refusal(&reply, "DotComponent", &["lexicon.read"]);
}

#[test]
fn outside_roots_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("lib/A.lex.tex", "x")]);
    assert_refusal(&reply, "OutsideRoots", &["lexicon.read"]);
}

#[test]
fn not_lex_tex_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/A.md", "x")]);
    assert_refusal(&reply, "NotLexTex", &["lexicon.read"]);
}

#[test]
fn too_many_files_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let names: Vec<String> = (0..257).map(|n| format!("src/F{n:03}.lex.tex")).collect();
    let rows: Vec<(&str, &str)> = names.iter().map(|n| (n.as_str(), "")).collect();
    let reply = write(&mut gateway, "tiny", &rows);
    assert_refusal(&reply, "TooManyFiles", &["lexicon.read"]);
}

#[test]
fn too_many_bytes_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let over = "a".repeat(8_388_609);
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", &over)]);
    assert_refusal(&reply, "TooManyBytes", &["lexicon.read"]);
}

#[test]
fn duplicate_path_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x"), ("src/A.lex.tex", "y")]);
    assert_refusal(&reply, "DuplicatePath", &["lexicon.read"]);
}

#[test]
fn profile_record_malformed_is_a_typed_refusal_dr_10() {
    let mut gateway = Gateway::new();
    gateway.provision("tiny", "not a record\nsrc\n");
    let reply = write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]);
    assert_refusal(&reply, "ProfileRecordMalformed", &["lexicon.read"]);
}

#[test]
fn draft_unknown_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let reply = read(&mut gateway, &format!("tiny/{}", "0".repeat(64)), None);
    assert_refusal(&reply, "DraftUnknown", &["store.get"]);
    // A handle that is not a handle names no draft, and asks for no effect.
    for draft in ["tiny", "tiny/abc", "/", "tiny/ZZ", &format!("tiny/{}", "A".repeat(64))] {
        let reply = read(&mut gateway, draft, None);
        assert_refusal(&reply, "DraftUnknown", &[]);
    }
}

#[test]
fn path_unknown_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let draft = handle(&write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]));
    let reply = read(&mut gateway, &draft, Some("src/B.lex.tex"));
    assert_refusal(&reply, "PathUnknown", &["store.get"]);
}

#[test]
fn draft_corrupt_is_a_typed_refusal_dr_10() {
    let mut gateway = with_profile();
    let draft = handle(&write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]));
    let path = gateway.drafts().join(&draft);
    let mut bytes = std::fs::read(&path).expect("the draft");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&path, &bytes).expect("a draft");
    assert_refusal(&read(&mut gateway, &draft, None), "DraftCorrupt", &["store.get"]);
    assert_refusal(
        &read(&mut gateway, &draft, Some("src/A.lex.tex")),
        "DraftCorrupt",
        &["store.get"],
    );
}

#[test]
fn bytes_that_are_not_a_preimage_are_a_corrupt_draft_dr_10() {
    // The bytes hash to their key, so the digest agrees: it is the framing that is wrong.
    let mut gateway = with_profile();
    let candidates: [Vec<u8>; 4] = [
        b"not a draft".to_vec(),
        Vec::new(),
        b"\0\0\0\x06domain\0\0\0\0\0\0\0\x03abc".to_vec(),
        b"\0\0\0\x06domain\0\0\0\0\0\0\0\x14lexlean-mcp-draft-v1\0\0\0\x07profile\0\0\0\0\0\0\0\x01x".to_vec(),
    ];
    for bytes in candidates {
        let id = sha256(&bytes);
        let directory = gateway.drafts().join("tiny");
        std::fs::create_dir_all(&directory).expect("a directory");
        std::fs::write(directory.join(&id), &bytes).expect("a draft");
        let reply = read(&mut gateway, &format!("tiny/{id}"), None);
        assert_refusal(&reply, "DraftCorrupt", &["store.get"]);
    }
}

#[test]
fn a_preimage_with_trailing_bytes_is_a_corrupt_draft_dr_10() {
    let mut gateway = with_profile();
    let draft = handle(&write(&mut gateway, "tiny", &[("src/A.lex.tex", "x")]));
    let mut bytes = std::fs::read(gateway.drafts().join(&draft)).expect("the draft");
    bytes.push(0);
    let id = sha256(&bytes);
    std::fs::write(gateway.drafts().join("tiny").join(&id), &bytes).expect("a draft");
    assert_refusal(
        &read(&mut gateway, &format!("tiny/{id}"), None),
        "DraftCorrupt",
        &["store.get"],
    );
}

#[test]
fn profile_unknown_is_a_typed_host_failure_dr_10() {
    let mut gateway = Gateway::new();
    let reply = write(&mut gateway, "absent", &[("src/A.lex.tex", "x")]);
    assert_failure(&reply, "profile-unknown", &["lexicon.read"]);
}

#[test]
fn project_refused_is_a_typed_host_failure_dr_10() {
    let mut gateway = with_profile();
    let reply = gateway.call(
        "lexlean_check",
        json!({"project": format!("tiny/{}", "0".repeat(64)), "files": []}),
    );
    assert_failure(&reply, "project-refused", &["engine.check"]);
}

#[test]
fn a_key_the_store_refuses_is_a_typed_host_failure_dr_10() {
    let mut gateway = with_profile();
    let reply = read(&mut gateway, &format!("../{}", "0".repeat(64)), None);
    assert_failure(&reply, "key-refused", &["store.get"]);
}

#[test]
fn a_bundle_file_is_never_a_file_of_a_draft_dr_10() {
    // The bundle is what the profile provisions; a draft cannot name it, because a bundle file is
    // not a `.lex.tex` file under a source root.
    let mut gateway = with_profile();
    for file in BUNDLE {
        let reply = write(&mut gateway, "tiny", &[(file, "x")]);
        assert!(reply.is_error(), "{file}: {}", reply.body);
        assert_eq!(reply.effects, ["lexicon.read"], "{file}");
    }
}
