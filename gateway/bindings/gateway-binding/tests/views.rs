//! The closed `view` of `store.get` and `form` of `store.put` (issue #9, EF-09): what the host
//! reads and stores for each value, the class of an answer the sdk refused, and the typed
//! failure of a value outside the model's sets, which reads nothing, stores nothing and does not
//! reach `lexlean::sdk`.

use gateway_binding::materialize::DOMAIN;
use gateway_binding::{Config, Native};
use gateway_host_api::protocol::{store_get, store_put, Host, HostError};
use lexlean::sdk;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const DRAFTS: u32 = 1;
const PROFILE: &str = "p";
const PROFILE_ID: [u8; 32] = [7; 32];
const PATH: &str = "src/Tiny.lex.tex";
const TINY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../vectors/fixture/tiny/src/Tiny.lex.tex"
);
const PROSE: &str = "\\section{Notes}\nOnly words.\n";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn frame(head: &str, payload: &[u8]) -> Vec<u8> {
    let mut out = (head.len() as u32).to_be_bytes().to_vec();
    out.extend_from_slice(head.as_bytes());
    out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

/// The preimage of the model (`src/Identity.lex.tex`, `encDraft`); the rows are given sorted.
fn preimage(rows: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = frame("domain", DOMAIN);
    out.extend(frame("profile", &PROFILE_ID));
    out.extend_from_slice(&("sources".len() as u32).to_be_bytes());
    out.extend_from_slice(b"sources");
    out.extend_from_slice(&((rows.len() * 2) as u64).to_be_bytes());
    for (path, content) in rows {
        out.extend(frame("source", path.as_bytes()));
        out.extend(frame("source", content));
    }
    out
}

fn tiny() -> String {
    std::fs::read_to_string(TINY).expect("the tiny module")
}

struct Fixture {
    host: Native,
    root: std::path::PathBuf,
    _directory: tempfile::TempDir,
}

/// A host whose drafts area holds the draft `p/<id>` of the rows given, and its handle.
fn with_draft(rows: &[(&str, &[u8])]) -> (Fixture, String) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let root = directory.path().join("store");
    let blob = preimage(rows);
    let id = hex(&Sha256::digest(&blob));
    let place = root.join("drafts").join(PROFILE);
    std::fs::create_dir_all(&place).expect("a directory");
    std::fs::write(place.join(&id), blob).expect("a draft");
    let host = Native::new(Config::at(root.clone()));
    (
        Fixture {
            host,
            root,
            _directory: directory,
        },
        format!("{PROFILE}/{id}"),
    )
}

fn get(
    fixture: &mut Fixture,
    key: &str,
    path: &str,
    view: u32,
) -> Result<store_get::Completed, store_get::Failure> {
    fixture
        .host
        .store_get(store_get::Request {
            area: DRAFTS,
            key: key.to_string(),
            path: path.to_string(),
            view,
        })
        .map_err(|error| match error {
            HostError::Failed(failure) => failure,
            other => panic!("a host error that is not a failure: {other:?}"),
        })
}

fn put(
    fixture: &mut Fixture,
    key: &str,
    content: &[u8],
    form: u32,
) -> Result<store_put::Completed, store_put::Failure> {
    fixture
        .host
        .store_put(store_put::Request {
            area: DRAFTS,
            key: key.to_string(),
            content: content.to_vec(),
            form,
        })
        .map_err(|error| match error {
            HostError::Failed(failure) => failure,
            other => panic!("a host error that is not a failure: {other:?}"),
        })
}

fn tiny_data() -> Value {
    sdk::read_module(&tiny()).expect("reads")
}

fn assemble_request(data: &Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "kind": "semantic",
        "header": {
            "name": "Tiny",
            "glossaries": ["lexlean.std.bool@1.2.0", "lexlean.std.nat@1.2.0"],
            "imports": [],
            "title": "Boolean",
        },
        "data": data,
    }))
    .expect("JSON")
}

// ---- the views (DR-12) -------------------------------------------------------------------------

#[test]
fn view_0_is_the_stored_bytes_and_never_the_sdk_dr_12() {
    let text = tiny();
    let (mut fixture, draft) = with_draft(&[(PATH, text.as_bytes())]);
    // The whole preimage, as stored.
    let whole = get(&mut fixture, &draft, "", 0).expect("read");
    assert!(whole.found);
    assert_eq!(whole.class, 0);
    assert_eq!(whole.content, preimage(&[(PATH, text.as_bytes())]));
    // A file of the draft, as stored, whatever it holds.
    for (path, bytes) in [
        (PATH, text.as_bytes()),
        ("src/Prose.lex.tex", PROSE.as_bytes()),
    ] {
        let (mut fixture, draft) = with_draft(&[
            (PATH, text.as_bytes()),
            ("src/Prose.lex.tex", PROSE.as_bytes()),
        ]);
        let done = get(&mut fixture, &draft, path, 0).expect("read");
        assert!(done.found, "{path}");
        assert_eq!(done.class, 0, "{path}");
        assert_eq!(done.content, bytes, "{path}");
    }
}

#[test]
fn views_1_and_2_are_what_the_sdk_gives_for_the_file_of_a_draft_dr_12() {
    let text = tiny();
    let (mut fixture, draft) = with_draft(&[(PATH, text.as_bytes())]);
    let data = get(&mut fixture, &draft, PATH, 1).expect("read");
    assert!(data.found);
    assert_eq!(data.class, 0);
    assert_eq!(
        data.content,
        serde_json::to_vec(&sdk::read_module(&text).expect("reads")).expect("JSON")
    );
    let spans = get(&mut fixture, &draft, PATH, 2).expect("read");
    assert_eq!(spans.class, 0);
    assert_eq!(
        spans.content,
        serde_json::to_vec(&sdk::declaration_spans(&text).expect("spans")).expect("JSON")
    );
    // A module written as data has no spans: null.
    let as_data = sdk::to_data_form(&text).expect("the data form");
    let (mut fixture, draft) = with_draft(&[(PATH, as_data.as_bytes())]);
    let none = get(&mut fixture, &draft, PATH, 2).expect("read");
    assert_eq!(none.class, 0);
    assert_eq!(none.content, b"null");
}

#[test]
fn a_view_of_text_the_sdk_refuses_is_class_1_with_its_message_dr_12() {
    let (mut fixture, draft) = with_draft(&[("src/Prose.lex.tex", PROSE.as_bytes())]);
    for view in [1, 2] {
        let done = get(&mut fixture, &draft, "src/Prose.lex.tex", view).expect("read");
        assert!(done.found, "view {view}");
        assert_eq!(done.class, 1, "view {view}");
        assert!(
            !done.content.is_empty(),
            "view {view}: the message of the sdk"
        );
    }
    let (mut fixture, draft) = with_draft(&[("src/Bytes.lex.tex", &[0xff, 0xfe])]);
    let done = get(&mut fixture, &draft, "src/Bytes.lex.tex", 1).expect("read");
    assert_eq!(done.class, 1, "bytes that are not UTF-8 are not a module");
}

#[test]
fn a_path_the_draft_lacks_is_class_2_and_a_draft_that_is_absent_is_not_found_dr_12() {
    let (mut fixture, draft) = with_draft(&[(PATH, tiny().as_bytes())]);
    for view in [0, 1, 2] {
        let done = get(&mut fixture, &draft, "src/Missing.lex.tex", view).expect("read");
        assert!(done.found, "view {view}: the draft is there");
        assert_eq!(done.class, 2, "view {view}");
        assert!(done.content.is_empty());
    }
    let absent = format!("{PROFILE}/{}", "0".repeat(64));
    for view in [0, 1, 2] {
        let done = get(&mut fixture, &absent, PATH, view).expect("read");
        assert!(!done.found, "view {view}");
        assert_eq!(done.class, 0);
    }
}

#[test]
fn a_path_asks_for_a_draft_and_a_changed_draft_is_unreadable_dr_12() {
    let text = tiny();
    let (mut fixture, draft) = with_draft(&[(PATH, text.as_bytes())]);
    // A key that is not the handle of a draft has no files.
    put(&mut fixture, "p/notes.txt", b"words", 0).expect("stored");
    assert_eq!(
        get(&mut fixture, "p/notes.txt", PATH, 1).err(),
        Some(store_get::Failure::Unreadable)
    );
    // A draft whose bytes are not those of its identity.
    let place = fixture.root.join("drafts").join(&draft);
    let mut bytes = std::fs::read(&place).expect("the draft");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&place, bytes).expect("changed");
    for view in [0, 1, 2] {
        assert_eq!(
            get(&mut fixture, &draft, PATH, view).err(),
            Some(store_get::Failure::Unreadable),
            "view {view}"
        );
    }
    // With no path, the view applies to the bytes at the key, whatever they are.
    let done = get(&mut fixture, "p/notes.txt", "", 1).expect("read");
    assert_eq!(done.class, 1, "words are not a module");
}

// ---- the forms (DR-13) -------------------------------------------------------------------------

#[test]
fn form_0_stores_the_bytes_as_given_dr_13() {
    let (mut fixture, _draft) = with_draft(&[]);
    let done = put(&mut fixture, "p/modules/a", b"as given", 0).expect("stored");
    assert_eq!((done.length, done.class), (8, 0));
    assert!(done.rendered.is_empty());
    let read = get(&mut fixture, "p/modules/a", "", 0).expect("read");
    assert_eq!(read.content, b"as given");
}

#[test]
fn form_1_stores_what_sdk_assemble_returns_dr_13() {
    let (mut fixture, _draft) = with_draft(&[]);
    let data = tiny_data();
    let done = put(&mut fixture, "p/modules/m", &assemble_request(&data), 1).expect("stored");
    assert_eq!(done.class, 0);
    let expected = sdk::assemble(
        &sdk::SourceHeader {
            name: "Tiny",
            glossaries: &["lexlean.std.bool@1.2.0", "lexlean.std.nat@1.2.0"],
            imports: &[],
            title: "Boolean",
        },
        sdk::ModuleKind::Semantic,
        &data,
    )
    .expect("assembles");
    assert_eq!(done.rendered, expected.as_bytes());
    assert_eq!(done.length, expected.len() as u64);
    let read = get(&mut fixture, "p/modules/m", "", 0).expect("read");
    assert_eq!(
        read.content,
        expected.as_bytes(),
        "the stored bytes are the render"
    );
}

#[test]
fn form_2_stores_what_sdk_write_module_returns_dr_13() {
    let (mut fixture, _draft) = with_draft(&[]);
    let text = tiny();
    let mut changed = tiny_data();
    changed["declarations"][0]["name"] = json!("twice");
    let request = serde_json::to_vec(&json!({"text": text, "data": changed})).expect("JSON");
    let done = put(&mut fixture, "p/modules/r", &request, 2).expect("stored");
    assert_eq!(done.class, 0);
    let expected = sdk::write_module(&text, &changed).expect("rewrites");
    assert_eq!(done.rendered, expected.as_bytes());
    let read = get(&mut fixture, "p/modules/r", "", 0).expect("read");
    assert_eq!(read.content, expected.as_bytes());
}

#[test]
fn a_module_the_sdk_refuses_is_class_1_and_stores_nothing_dr_15() {
    let (mut fixture, _draft) = with_draft(&[]);
    let refused = [
        (1, b"not JSON".to_vec()),
        (
            1,
            serde_json::to_vec(&json!({"kind": "native"})).expect("JSON"),
        ),
        (
            1,
            assemble_request(&json!({"spec": "lexlean/semantic-module/9"})),
        ),
        (
            2,
            serde_json::to_vec(&json!({"text": PROSE, "data": {}})).expect("JSON"),
        ),
        (
            2,
            serde_json::to_vec(&json!({"text": 3, "data": {}})).expect("JSON"),
        ),
    ];
    for (form, request) in refused {
        let key = format!("p/modules/refused{form}");
        let done = put(&mut fixture, &key, &request, form).expect("answered");
        assert_eq!(done.class, 1, "form {form}");
        assert_eq!(done.length, 0, "form {form}");
        assert!(!done.rendered.is_empty(), "form {form}: the message");
        let read = get(&mut fixture, &key, "", 0).expect("read");
        assert!(!read.found, "form {form}: nothing is stored");
    }
}

// ---- the closed fields (DR-15, GW-05) ----------------------------------------------------------

const PAST_THE_SET: [u32; 5] = [3, 4, 255, 65_536, u32::MAX];

#[test]
fn an_unknown_view_is_a_typed_failure_dr_15() {
    let text = tiny();
    let (mut fixture, draft) = with_draft(&[(PATH, text.as_bytes())]);
    for view in PAST_THE_SET {
        assert_eq!(
            get(&mut fixture, &draft, PATH, view).err(),
            Some(store_get::Failure::ViewUnknown),
            "view {view}"
        );
        assert_eq!(
            get(&mut fixture, &draft, "", view).err(),
            Some(store_get::Failure::ViewUnknown),
            "view {view}, no path"
        );
    }
}

#[test]
fn an_unknown_view_is_a_typed_failure_gw_05() {
    // The view is checked before anything is read: an area or a key the store would refuse, and
    // an entry that does not exist, are still `view-unknown`, and no directory is made.
    let directory = tempfile::tempdir().expect("a temporary directory");
    let root = directory.path().join("store");
    let mut host = Native::new(Config::at(root.clone()));
    for view in PAST_THE_SET {
        for (area, key) in [(DRAFTS, "p/a"), (99, "p/a"), (DRAFTS, "../x"), (DRAFTS, "")] {
            let failure = host
                .store_get(store_get::Request {
                    area,
                    key: key.to_string(),
                    path: String::new(),
                    view,
                })
                .err();
            assert!(
                matches!(
                    failure,
                    Some(HostError::Failed(store_get::Failure::ViewUnknown))
                ),
                "view {view}, area {area}, key {key:?}: {failure:?}"
            );
        }
    }
    assert!(!root.exists(), "the store was not touched");
}

#[test]
fn an_unknown_form_is_a_typed_failure_dr_15() {
    let (mut fixture, _draft) = with_draft(&[]);
    for form in PAST_THE_SET {
        assert_eq!(
            put(
                &mut fixture,
                "p/modules/x",
                &assemble_request(&tiny_data()),
                form
            )
            .err(),
            Some(store_put::Failure::FormUnknown),
            "form {form}"
        );
    }
    let read = get(&mut fixture, "p/modules/x", "", 0).expect("read");
    assert!(!read.found, "nothing was stored under an unknown form");
}

#[test]
fn an_unknown_form_is_a_typed_failure_gw_05() {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let root = directory.path().join("store");
    let mut host = Native::new(Config::at(root.clone()));
    for form in PAST_THE_SET {
        for (area, key) in [(DRAFTS, "p/a"), (99, "p/a"), (DRAFTS, "../x"), (DRAFTS, "")] {
            let failure = host
                .store_put(store_put::Request {
                    area,
                    key: key.to_string(),
                    content: b"x".to_vec(),
                    form,
                })
                .err();
            assert!(
                matches!(
                    failure,
                    Some(HostError::Failed(store_put::Failure::FormUnknown))
                ),
                "form {form}, area {area}, key {key:?}: {failure:?}"
            );
        }
    }
    assert!(!root.exists(), "the store was not touched");
}
