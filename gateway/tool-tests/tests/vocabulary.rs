//! VO-04, VO-09 to VO-15: `lexlean_profiles`, `lexlean_lexicon_search`, the resource template
//! `lexlean://profiles/{profile}/lexicon/{package}@{version}` and the completion of its arguments
//! (issue #8), through the production core with the real host, on a store in a temporary
//! directory whose profiles the operator's `install` provisioned. Nothing here knows what a tool
//! does: the core does, and the store, the catalogue and the vocabulary are what `install` wrote.

use std::path::{Path, PathBuf};

use gateway_tool_tests::{copy_tree, tiny, Gateway, Reply};
use serde_json::{json, Value};

const TEMPLATE: &str = "lexlean://profiles/{profile}/lexicon/{package}@{version}";

fn vectors() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../vectors/fixture")
}

fn vocab_project() -> PathBuf {
    vectors().join("vocab")
}

fn review() -> PathBuf {
    vectors().join("vocab.review")
}

/// A gateway whose store holds the profiles `tiny` (builtin packages only) and `vocab`.
fn stocked() -> Gateway {
    let gateway = Gateway::new();
    gateway.install_profile("tiny", &tiny(), &[]);
    gateway.install_profile("vocab", &vocab_project(), &[&review()]);
    gateway
}

fn catalogue_path(gateway: &Gateway) -> PathBuf {
    gateway.store().join("index/profiles")
}

fn catalogue(gateway: &Gateway) -> Value {
    serde_json::from_slice(&std::fs::read(catalogue_path(gateway)).expect("the catalogue"))
        .expect("JSON")
}

fn vocabulary(gateway: &Gateway, profile: &str) -> Value {
    let path = gateway.profile(profile).join("vocabulary");
    serde_json::from_slice(&std::fs::read(path).expect("the vocabulary")).expect("JSON")
}

fn write_catalogue(gateway: &Gateway, document: &Value) {
    let path = catalogue_path(gateway);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
    std::fs::write(path, serde_json::to_vec(document).expect("JSON")).expect("a catalogue");
}

fn write_vocabulary(gateway: &Gateway, profile: &str, document: &Value) {
    let directory = gateway.profile(profile);
    std::fs::create_dir_all(&directory).expect("a directory");
    std::fs::write(
        directory.join("vocabulary"),
        serde_json::to_vec(document).expect("JSON"),
    )
    .expect("a vocabulary");
}

fn names(listed: &Value, key: &str) -> Vec<String> {
    listed
        .as_array()
        .expect("an array")
        .iter()
        .map(|item| item[key].as_str().expect("a string").to_string())
        .collect()
}

fn refusal(reply: &Reply, name: &str, effects: &[&str]) {
    assert!(reply.is_error(), "{name}: {}", reply.body);
    assert_eq!(reply.structured()["refusal"], name, "{}", reply.body);
    assert_eq!(reply.effects, effects, "{name}: the effects of the call");
}

fn failure(reply: &Reply, name: &str, effects: &[&str]) {
    assert!(reply.is_error(), "{name}: {}", reply.body);
    assert_eq!(
        reply.structured(),
        &json!({"failure": name}),
        "{}",
        reply.body
    );
    assert_eq!(reply.effects, effects, "{name}: the effects of the call");
}

fn search(gateway: &mut Gateway, arguments: Value) -> Reply {
    gateway.call("lexlean_lexicon_search", arguments)
}

fn uri(profile: &str, package: &str, version: &str) -> String {
    format!("lexlean://profiles/{profile}/lexicon/{package}@{version}")
}

fn read(gateway: &mut Gateway, uri: &str) -> Reply {
    gateway.rpc("resources/read", Some(uri), json!({"uri": uri}))
}

fn complete(gateway: &mut Gateway, argument: &str, value: &str, context: Option<Value>) -> Reply {
    let mut params = json!({
        "ref": {"type": "ref/resource", "uri": TEMPLATE},
        "argument": {"name": argument, "value": value},
    });
    if let Some(context) = context {
        params["context"] = json!({"arguments": context});
    }
    gateway.rpc("completion/complete", None, params)
}

/// The package ID and version of a catalogue package.
fn package_ids(profile: &Value) -> Vec<(String, String, String, u64)> {
    profile["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .map(|package| {
            (
                package["package"].as_str().expect("id").to_string(),
                package["version"].as_str().expect("version").to_string(),
                package["language"].as_str().expect("language").to_string(),
                package["entries"].as_u64().expect("entries"),
            )
        })
        .collect()
}

/// A catalogue in which `vocab` also holds a package of language 1.0.
fn mixed_catalogue(gateway: &Gateway) -> Value {
    let mut document = catalogue(gateway);
    for profile in document["profiles"].as_array_mut().expect("profiles") {
        if profile["name"] == "vocab" {
            profile["packages"]
                .as_array_mut()
                .expect("packages")
                .push(json!({"package": "old.words", "version": "1.0.0", "language": "1.0", "entries": 1}));
        }
    }
    document
}

/// A vocabulary of `vocab` that holds a package of language 1.0.
fn mixed_vocabulary(gateway: &Gateway) -> Value {
    let mut document = vocabulary(gateway, "vocab");
    document["packages"]
        .as_array_mut()
        .expect("packages")
        .push(json!({"package": "old.words", "version": "1.0.0", "language": "1.0", "entries": 1}));
    document["entries"].as_array_mut().expect("entries").push(json!({
        "package": "old.words", "version": "1.0.0", "id": "old-word", "category": "label-word",
        "frame": "atom", "arity": 0,
        "denotes": {"kind": "document", "module": "Main", "component": "oldValid"},
        "forms": [{"id": "old-word", "channel": "text", "surface": "Old word", "canonical": true, "features": []}]
    }));
    document
}

// ---- VO-09: lexlean_profiles -----------------------------------------------------------------

#[test]
fn lexlean_profiles_lists_the_installed_profiles_vo_09() {
    let mut gateway = stocked();
    let reply = gateway.call("lexlean_profiles", json!({}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(
        reply.effects,
        ["store.get"],
        "one store.get of the catalogue"
    );
    let listed = &reply.structured()["profiles"];
    assert_eq!(names(listed, "name"), ["tiny", "vocab"], "in name order");
    assert_eq!(reply.structured()["refused"], json!([]));
    let stored = catalogue(&gateway);
    assert_eq!(listed, &stored["profiles"], "what install wrote");
    for profile in listed.as_array().expect("profiles") {
        let id = profile["id"].as_str().expect("an ID");
        assert_eq!(id.len(), 64);
        assert!(id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
        assert_eq!(profile["language"], "1.2");
        assert_eq!(profile["roots"], json!(["src"]));
        let record = std::fs::read_to_string(
            gateway
                .profile(profile["name"].as_str().expect("n"))
                .join("profile"),
        )
        .expect("a record");
        assert_eq!(
            record.lines().next(),
            Some(id),
            "the ID of the record of the profile"
        );
    }
    let vocab = package_ids(&listed[1]);
    let shapes = vocab
        .iter()
        .find(|p| p.0 == "shapes.words")
        .expect("shapes.words");
    assert_eq!(
        (shapes.1.as_str(), shapes.2.as_str(), shapes.3),
        ("1.0.0", "1.2", 2)
    );
    assert!(vocab.iter().any(|p| p.0 == "lexlean.core"));
    assert_eq!(package_ids(&listed[0]).len(), 3, "tiny: core, bool and nat");
    assert_eq!(
        serde_json::from_str::<Value>(reply.text()).expect("JSON"),
        *reply.structured(),
        "the text is the structured content"
    );
}

#[test]
fn a_named_profile_is_listed_alone_and_an_unknown_one_is_refused_vo_09() {
    let mut gateway = stocked();
    let reply = gateway.call("lexlean_profiles", json!({"profile": "vocab"}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(names(&reply.structured()["profiles"], "name"), ["vocab"]);
    assert_eq!(reply.effects, ["store.get"]);
    let reply = gateway.call("lexlean_profiles", json!({"profile": "absent"}));
    refusal(&reply, "ProfileUnknown", &["store.get"]);
}

#[test]
fn an_empty_store_has_no_profiles_vo_09() {
    let mut gateway = Gateway::new();
    let reply = gateway.call("lexlean_profiles", json!({}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(reply.structured(), &json!({"profiles": [], "refused": []}));
    assert_eq!(reply.effects, ["store.get"]);
    refusal(
        &gateway.call("lexlean_profiles", json!({"profile": "vocab"})),
        "ProfileUnknown",
        &["store.get"],
    );
}

#[test]
fn a_catalogue_that_is_not_the_layout_is_refused_vo_09() {
    let mut gateway = stocked();
    let good = catalogue(&gateway);
    let mut cases: Vec<(&str, Vec<u8>)> = vec![
        ("not JSON", b"{".to_vec()),
        ("an array", b"[]".to_vec()),
        ("no profiles member", b"{}".to_vec()),
        ("profiles that is not an array", br#"{"profiles":3}"#.to_vec()),
        ("a profile that is not an object", br#"{"profiles":[3]}"#.to_vec()),
        ("a profile without a name", br#"{"profiles":[{"language":"1.2","packages":[]}]}"#.to_vec()),
        ("a profile without a language", br#"{"profiles":[{"name":"a","packages":[]}]}"#.to_vec()),
        ("a profile without packages", br#"{"profiles":[{"name":"a","language":"1.2"}]}"#.to_vec()),
        (
            "a package without a language",
            br#"{"profiles":[{"name":"a","language":"1.2","packages":[{"package":"p","version":"1"}]}]}"#.to_vec(),
        ),
    ];
    let mut text = serde_json::to_vec(&good).expect("JSON");
    text.push(b' ');
    text.extend_from_slice(b"{}");
    cases.push(("trailing bytes", text));
    for (name, bytes) in cases {
        std::fs::write(catalogue_path(&gateway), bytes).expect("a catalogue");
        let reply = gateway.call("lexlean_profiles", json!({}));
        assert!(reply.is_error(), "{name}: {}", reply.body);
        assert_eq!(
            reply.structured()["refusal"],
            "CatalogueMalformed",
            "{name}"
        );
        assert_eq!(reply.effects, ["store.get"], "{name}");
    }
}

#[test]
fn a_catalogue_with_a_package_of_another_language_lists_the_profile_under_refused_vo_04() {
    let mut gateway = stocked();
    write_catalogue(&gateway, &mixed_catalogue(&gateway));
    let reply = gateway.call("lexlean_profiles", json!({}));
    assert!(!reply.is_error(), "{}", reply.body);
    assert_eq!(names(&reply.structured()["profiles"], "name"), ["tiny"]);
    assert_eq!(
        reply.structured()["refused"],
        json!([{"name": "vocab", "code": "LLC0103"}])
    );
    let named = gateway.call("lexlean_profiles", json!({"profile": "vocab"}));
    assert!(named.is_error(), "{}", named.body);
    assert_eq!(
        named.structured(),
        &json!({"refusal": "LanguageMismatch", "code": "LLC0103"})
    );
}

#[test]
fn the_schema_of_lexlean_profiles_is_closed_vo_09() {
    let mut gateway = stocked();
    let listed = gateway.list();
    let tool = listed.result()["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .find(|tool| tool["name"] == "lexlean_profiles")
        .expect("lexlean_profiles is listed")
        .clone();
    assert_eq!(tool["inputSchema"]["additionalProperties"], false);
    assert_eq!(tool["inputSchema"]["required"], json!([]));
    assert_eq!(tool["inputSchema"]["properties"]["profile"]["minLength"], 1);
    for arguments in [
        json!({"profile": ""}),
        json!({"profile": 3}),
        json!({"other": 1}),
    ] {
        let reply = gateway.call("lexlean_profiles", arguments.clone());
        assert_eq!(reply.error()["code"], -32602, "{arguments}: {}", reply.body);
        assert!(reply.effects.is_empty(), "{arguments}: no effect");
    }
}

// ---- VO-10: lexlean_lexicon_search -----------------------------------------------------------

fn entry_ids(reply: &Reply) -> Vec<String> {
    names(&reply.structured()["entries"], "id")
}

#[test]
fn search_pages_through_the_vocabulary_in_order_vo_10() {
    let mut gateway = stocked();
    let all = vocabulary(&gateway, "vocab");
    let stored: Vec<String> = names(&all["entries"], "id");
    let total = stored.len();
    assert!(
        total > 100,
        "the vocabulary of vocab holds the entries of core, bool, nat and shapes.words"
    );
    let mut seen = Vec::new();
    let mut offset = 0;
    while offset < total {
        let reply = search(
            &mut gateway,
            json!({"profile": "vocab", "offset": offset, "limit": 100}),
        );
        assert!(!reply.is_error(), "{}", reply.body);
        assert_eq!(reply.effects, ["lexicon.read"]);
        assert_eq!(reply.structured()["total"], total);
        assert_eq!(reply.structured()["offset"], offset);
        assert_eq!(reply.structured()["profile"], "vocab");
        let page = entry_ids(&reply);
        assert!(page.len() <= 100);
        seen.extend(page);
        offset += 100;
    }
    assert_eq!(seen, stored, "the pages are the vocabulary in its order");
    let reply = search(&mut gateway, json!({"profile": "vocab"}));
    assert_eq!(entry_ids(&reply).len(), 20, "the default limit is 20");
    let past = search(&mut gateway, json!({"profile": "vocab", "offset": 9999}));
    assert!(!past.is_error(), "{}", past.body);
    assert_eq!(entry_ids(&past), Vec::<String>::new());
    assert_eq!(past.structured()["total"], total);
    let one = search(&mut gateway, json!({"profile": "vocab", "limit": 1}));
    assert_eq!(entry_ids(&one), stored[..1]);
    assert_eq!(
        one.structured()["entries"][0],
        all["entries"][0],
        "an entry is the vocabulary's own"
    );
    assert_eq!(
        serde_json::from_str::<Value>(one.text()).expect("JSON"),
        *one.structured()
    );
}

#[test]
fn search_matches_ids_and_spellings_without_regard_to_case_vo_10() {
    let mut gateway = stocked();
    let by_id = search(
        &mut gateway,
        json!({"profile": "vocab", "query": "shapes-circle"}),
    );
    assert_eq!(entry_ids(&by_id), ["shapes-circle"]);
    assert_eq!(by_id.structured()["query"], "shapes-circle");
    let by_spelling = search(
        &mut gateway,
        json!({"profile": "vocab", "query": "SHAPES CIRCLE"}),
    );
    assert_eq!(
        entry_ids(&by_spelling),
        ["shapes-circle"],
        "a spelling, in another case"
    );
    let by_part = search(&mut gateway, json!({"profile": "vocab", "query": "shapes"}));
    assert_eq!(entry_ids(&by_part), ["shapes-circle", "shapes-square"]);
    assert_eq!(by_part.structured()["total"], 2);
    let upper_id = search(
        &mut gateway,
        json!({"profile": "vocab", "query": "SHAPES-SQUARE"}),
    );
    assert_eq!(
        entry_ids(&upper_id),
        ["shapes-square"],
        "an ID, in another case"
    );
    let none = search(
        &mut gateway,
        json!({"profile": "vocab", "query": "no such word anywhere"}),
    );
    assert!(!none.is_error(), "{}", none.body);
    assert_eq!(none.structured()["total"], 0);
    assert_eq!(entry_ids(&none), Vec::<String>::new());
    let ghost = search(&mut gateway, json!({"profile": "vocab", "query": "ghost"}));
    assert_eq!(
        entry_ids(&ghost),
        Vec::<String>::new(),
        "an entry into a missing module is not served"
    );
    let non_ascii = search(&mut gateway, json!({"profile": "vocab", "query": "ÿ"}));
    assert_eq!(
        non_ascii.structured()["total"],
        0,
        "case folding is ASCII only"
    );
    let paged = search(
        &mut gateway,
        json!({"profile": "vocab", "query": "shapes", "limit": 1, "offset": 1}),
    );
    assert_eq!(entry_ids(&paged), ["shapes-square"]);
    assert_eq!(paged.structured()["total"], 2, "total counts every match");
}

#[test]
fn search_filters_by_package_vo_10() {
    let mut gateway = stocked();
    let only = search(
        &mut gateway,
        json!({"profile": "vocab", "package": "shapes.words"}),
    );
    assert_eq!(entry_ids(&only), ["shapes-circle", "shapes-square"]);
    for entry in only.structured()["entries"].as_array().expect("entries") {
        assert_eq!(entry["package"], "shapes.words");
        assert_eq!(entry["version"], "1.0.0");
    }
    let nat = search(
        &mut gateway,
        json!({"profile": "vocab", "package": "lexlean.std.nat", "limit": 100}),
    );
    assert!(nat.structured()["total"].as_u64().expect("total") > 0);
    for entry in nat.structured()["entries"].as_array().expect("entries") {
        assert_eq!(entry["package"], "lexlean.std.nat");
    }
    let other = search(
        &mut gateway,
        json!({"profile": "vocab", "package": "no.such", "query": "shapes"}),
    );
    assert_eq!(other.structured()["total"], 0);
    let both = search(
        &mut gateway,
        json!({"profile": "vocab", "package": "shapes.words", "query": "square"}),
    );
    assert_eq!(
        entry_ids(&both),
        ["shapes-square"],
        "the package and the query together"
    );
}

#[test]
fn search_refusals_are_typed_vo_10() {
    let mut gateway = stocked();
    failure(
        &search(&mut gateway, json!({"profile": "absent"})),
        "profile-unknown",
        &["lexicon.read"],
    );
    failure(
        &search(&mut gateway, json!({"profile": "../vocab"})),
        "key-refused",
        &["lexicon.read"],
    );
    std::fs::remove_file(gateway.profile("tiny").join("vocabulary")).expect("a removed vocabulary");
    refusal(
        &search(&mut gateway, json!({"profile": "tiny"})),
        "VocabularyMissing",
        &["lexicon.read"],
    );
    let good = vocabulary(&gateway, "vocab");
    let mut cases: Vec<(&str, Vec<u8>)> = vec![
        ("not JSON", b"{".to_vec()),
        ("an array", b"[]".to_vec()),
        ("no entries", br#"{"profile":"vocab","language":"1.2","packages":[]}"#.to_vec()),
        ("no packages", br#"{"profile":"vocab","language":"1.2","entries":[]}"#.to_vec()),
        ("no language", br#"{"profile":"vocab","packages":[],"entries":[]}"#.to_vec()),
        (
            "an entry without an ID",
            br#"{"profile":"vocab","language":"1.2","packages":[],"entries":[{"package":"p","version":"1","forms":[]}]}"#.to_vec(),
        ),
        (
            "an entry without forms",
            br#"{"profile":"vocab","language":"1.2","packages":[],"entries":[{"package":"p","version":"1","id":"x"}]}"#.to_vec(),
        ),
    ];
    let mut trailing = serde_json::to_vec(&good).expect("JSON");
    trailing.extend_from_slice(b" 1");
    cases.push(("trailing bytes", trailing));
    for (name, bytes) in cases {
        std::fs::write(gateway.profile("vocab").join("vocabulary"), &bytes).expect("a vocabulary");
        let reply = search(&mut gateway, json!({"profile": "vocab", "query": "x"}));
        assert!(reply.is_error(), "{name}: {}", reply.body);
        assert_eq!(
            reply.structured()["refusal"],
            "VocabularyMalformed",
            "{name}"
        );
        assert_eq!(reply.effects, ["lexicon.read"], "{name}");
    }
}

#[test]
fn search_refuses_a_vocabulary_of_mixed_languages_with_llc0103_vo_04() {
    let mut gateway = stocked();
    write_vocabulary(&gateway, "vocab", &mixed_vocabulary(&gateway));
    let reply = search(&mut gateway, json!({"profile": "vocab", "query": "word"}));
    assert!(reply.is_error(), "{}", reply.body);
    assert_eq!(
        reply.structured(),
        &json!({"refusal": "LanguageMismatch", "code": "LLC0103"}),
        "no entry is returned"
    );
    for package in ["old.words", "shapes.words"] {
        let resource = read(&mut gateway, &uri("vocab", package, "1.0.0"));
        assert_eq!(
            resource.error()["code"],
            -32602,
            "{package}: {}",
            resource.body
        );
        let message = resource.error()["message"].as_str().expect("a message");
        assert!(
            message.contains("LanguageMismatch") && message.contains("LLC0103"),
            "{package}: {message}"
        );
        assert_eq!(resource.effects, ["lexicon.read"]);
    }
    let tiny = search(&mut gateway, json!({"profile": "tiny", "query": "x"}));
    assert!(!tiny.is_error(), "a profile of one language is served");
}

#[test]
fn the_schema_of_lexlean_lexicon_search_is_closed_vo_10() {
    let mut gateway = stocked();
    let listed = gateway.list();
    let tool = listed.result()["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .find(|tool| tool["name"] == "lexlean_lexicon_search")
        .expect("lexlean_lexicon_search is listed")
        .clone();
    let schema = &tool["inputSchema"];
    assert_eq!(schema["additionalProperties"], false);
    assert_eq!(schema["required"], json!(["profile"]));
    assert_eq!(schema["properties"]["limit"]["maximum"], 100);
    for arguments in [
        json!({}),
        json!({"profile": ""}),
        json!({"profile": "vocab", "limit": 0}),
        json!({"profile": "vocab", "limit": 101}),
        json!({"profile": "vocab", "limit": 1.5}),
        json!({"profile": "vocab", "offset": -1}),
        json!({"profile": "vocab", "offset": 10001}),
        json!({"profile": "vocab", "query": 3}),
        json!({"profile": "vocab", "package": ""}),
        json!({"profile": "vocab", "extra": true}),
    ] {
        let reply = search(&mut gateway, arguments.clone());
        assert_eq!(reply.error()["code"], -32602, "{arguments}: {}", reply.body);
        assert!(reply.effects.is_empty(), "{arguments}: no effect");
    }
    let names_listed: Vec<&str> = listed.result()["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .map(|tool| tool["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(
        names_listed.len(),
        12,
        "the ten tools and the two of issue 8"
    );
}

// ---- VO-11: the resource template ------------------------------------------------------------

#[test]
fn the_template_is_listed_public_with_the_owner_ttl_vo_11() {
    let mut gateway = stocked();
    let reply = gateway.rpc("resources/templates/list", None, json!({}));
    assert!(
        reply.effects.is_empty(),
        "no effect: the template is the gateway's"
    );
    let result = reply.result();
    assert_eq!(result["cacheScope"], "public");
    assert_eq!(result["ttlMs"], 60000);
    let templates = result["resourceTemplates"].as_array().expect("templates");
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0]["uriTemplate"], TEMPLATE);
    assert_eq!(templates[0]["mimeType"], "application/json");
    assert!(templates[0]["name"]
        .as_str()
        .is_some_and(|name| !name.is_empty()));
    let paged = gateway.rpc("resources/templates/list", None, json!({"cursor": "x"}));
    assert_eq!(
        paged.error()["code"],
        -32602,
        "there is one page: {}",
        paged.body
    );
}

#[test]
fn resources_list_one_resource_per_package_of_every_profile_vo_11() {
    let mut gateway = stocked();
    let reply = gateway.rpc("resources/list", None, json!({}));
    assert_eq!(reply.effects, ["store.get"]);
    let result = reply.result();
    assert_eq!(result["cacheScope"], "public");
    assert_eq!(result["ttlMs"], 60000);
    let listed = result["resources"].as_array().expect("resources");
    let stored = catalogue(&gateway);
    let mut wanted = Vec::new();
    for profile in stored["profiles"].as_array().expect("profiles") {
        for (package, version, _, _) in package_ids(profile) {
            wanted.push(uri(
                profile["name"].as_str().expect("name"),
                &package,
                &version,
            ));
        }
    }
    assert_eq!(names(&Value::Array(listed.clone()), "uri"), wanted);
    for resource in listed {
        assert_eq!(resource["mimeType"], "application/json");
        assert!(resource["name"]
            .as_str()
            .is_some_and(|name| name.contains('@')));
    }
    let mut empty = Gateway::new();
    assert_eq!(
        empty.rpc("resources/list", None, json!({})).result()["resources"],
        json!([]),
        "an empty store lists nothing"
    );
    write_catalogue(&gateway, &mixed_catalogue(&gateway));
    let mixed = gateway.rpc("resources/list", None, json!({}));
    assert!(
        !names(&mixed.result()["resources"], "uri")
            .iter()
            .any(|uri| uri.contains("/vocab/")),
        "a profile of mixed languages lists no resource"
    );
    assert_eq!(
        gateway
            .rpc("resources/list", None, json!({"cursor": "x"}))
            .error()["code"],
        -32602
    );
}

#[test]
fn a_resource_is_read_as_the_entries_of_its_package_vo_11() {
    let mut gateway = stocked();
    let wanted = uri("vocab", "shapes.words", "1.0.0");
    let reply = read(&mut gateway, &wanted);
    assert_eq!(
        reply.effects,
        ["lexicon.read"],
        "one lexicon.read of the vocabulary"
    );
    let result = reply.result();
    assert_eq!(result["cacheScope"], "public");
    assert_eq!(result["ttlMs"], 60000);
    let contents = result["contents"].as_array().expect("contents");
    assert_eq!(contents.len(), 1);
    assert_eq!(contents[0]["uri"], wanted.as_str());
    assert_eq!(contents[0]["mimeType"], "application/json");
    let document: Value =
        serde_json::from_str(contents[0]["text"].as_str().expect("text")).expect("JSON");
    assert_eq!(document["package"], "shapes.words");
    assert_eq!(document["version"], "1.0.0");
    assert_eq!(document["language"], "1.2");
    assert_eq!(
        names(&document["entries"], "id"),
        ["shapes-circle", "shapes-square"]
    );
    let stored = vocabulary(&gateway, "vocab");
    let from_store: Vec<&Value> = stored["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .filter(|entry| entry["package"] == "shapes.words")
        .collect();
    assert_eq!(
        document["entries"]
            .as_array()
            .expect("entries")
            .iter()
            .collect::<Vec<_>>(),
        from_store
    );
    let core = read(&mut gateway, &uri("tiny", "lexlean.core", "1.2.0"));
    let core_document: Value =
        serde_json::from_str(core.result()["contents"][0]["text"].as_str().expect("text"))
            .expect("JSON");
    let count = stored["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .find(|package| package["package"] == "lexlean.core")
        .expect("core")["entries"]
        .as_u64()
        .expect("count");
    assert_eq!(
        core_document["entries"].as_array().expect("entries").len() as u64,
        count
    );
}

#[test]
fn a_resource_that_is_not_found_is_32602_with_its_uri_vo_11() {
    let mut gateway = stocked();
    let not_found = [
        uri("absent", "shapes.words", "1.0.0"),
        uri("vocab", "no.such", "1.0.0"),
        uri("vocab", "shapes.words", "9.9.9"),
        uri("vocab", "shapes.words", ""),
        "lexlean://profiles/vocab/lexicon/shapes.words".to_string(),
        "lexlean://profiles/vocab/lexicon/@1.0.0".to_string(),
        "lexlean://profiles//lexicon/shapes.words@1.0.0".to_string(),
        "lexlean://profiles/vocab/other/shapes.words@1.0.0".to_string(),
        "lexlean://profile/vocab/lexicon/shapes.words@1.0.0".to_string(),
        "http://profiles/vocab/lexicon/shapes.words@1.0.0".to_string(),
        "lexlean://profiles/vocab/lexicon/shapes.words@1.0.0/".to_string(),
        "lexlean://profiles/vocab/lexicon/shapes.words@1.0.0?x=1".to_string(),
        "lexlean://profiles/a/b/lexicon/shapes.words@1.0.0".to_string(),
        "test://static-text".to_string(),
    ];
    for wanted in not_found {
        let reply = read(&mut gateway, &wanted);
        assert_eq!(reply.error()["code"], -32602, "{wanted:?}: {}", reply.body);
        assert_eq!(reply.error()["data"]["uri"], wanted.as_str(), "{wanted:?}");
        assert!(
            reply.effects.len() <= 1 && reply.effects.iter().all(|effect| effect == "lexicon.read"),
            "{wanted:?}: at most one lexicon.read: {:?}",
            reply.effects
        );
    }
    std::fs::remove_file(gateway.profile("vocab").join("vocabulary"))
        .expect("a removed vocabulary");
    let missing = read(&mut gateway, &uri("vocab", "shapes.words", "1.0.0"));
    assert_eq!(
        missing.error()["code"],
        -32602,
        "a profile without a vocabulary has no resources"
    );
}

// ---- VO-12: completion -----------------------------------------------------------------------

fn values(reply: &Reply) -> Vec<String> {
    reply.result()["completion"]["values"]
        .as_array()
        .expect("values")
        .iter()
        .map(|value| value.as_str().expect("a string").to_string())
        .collect()
}

#[test]
fn the_profile_argument_completes_by_prefix_vo_12() {
    let mut gateway = stocked();
    let all = complete(&mut gateway, "profile", "", None);
    assert_eq!(values(&all), ["tiny", "vocab"]);
    assert_eq!(all.effects, ["store.get"]);
    assert_eq!(all.result()["completion"]["total"], 2);
    assert_eq!(all.result()["completion"]["hasMore"], false);
    assert_eq!(
        values(&complete(&mut gateway, "profile", "v", None)),
        ["vocab"]
    );
    assert_eq!(
        values(&complete(&mut gateway, "profile", "x", None)),
        Vec::<String>::new()
    );
    assert_eq!(
        values(&complete(&mut gateway, "profile", "V", None)),
        Vec::<String>::new(),
        "a prefix, not a fold"
    );
}

#[test]
fn the_package_argument_completes_within_the_context_profile_vo_12() {
    let mut gateway = stocked();
    let in_vocab = complete(
        &mut gateway,
        "package",
        "",
        Some(json!({"profile": "vocab"})),
    );
    assert_eq!(
        values(&in_vocab),
        [
            "lexlean.core",
            "lexlean.std.bool",
            "lexlean.std.nat",
            "shapes.words"
        ]
    );
    assert_eq!(in_vocab.effects, ["store.get"]);
    assert_eq!(
        values(&complete(
            &mut gateway,
            "package",
            "lexlean.std.n",
            Some(json!({"profile": "vocab"}))
        )),
        ["lexlean.std.nat"]
    );
    assert_eq!(
        values(&complete(
            &mut gateway,
            "package",
            "sh",
            Some(json!({"profile": "tiny"}))
        )),
        Vec::<String>::new(),
        "tiny has no shapes.words"
    );
    assert_eq!(
        values(&complete(&mut gateway, "package", "sh", None)),
        ["shapes.words"],
        "without a profile in the context, the packages of every profile"
    );
    assert_eq!(
        values(&complete(&mut gateway, "package", "lexlean.core", None)),
        ["lexlean.core"],
        "each package once"
    );
    assert_eq!(
        values(&complete(
            &mut gateway,
            "package",
            "",
            Some(json!({"profile": "absent"}))
        )),
        Vec::<String>::new(),
        "a profile that is not installed has no package"
    );
}

#[test]
fn completion_is_bounded_at_100_values_vo_12() {
    let mut gateway = stocked();
    let mut document = catalogue(&gateway);
    let profiles = document["profiles"].as_array_mut().expect("profiles");
    for index in 0..150 {
        profiles.push(json!({"name": format!("p{index:03}"), "id": "0".repeat(64), "language": "1.2", "roots": ["src"], "packages": []}));
    }
    write_catalogue(&gateway, &document);
    let reply = complete(&mut gateway, "profile", "p", None);
    assert_eq!(values(&reply).len(), 100);
    assert_eq!(reply.result()["completion"]["total"], 150);
    assert_eq!(reply.result()["completion"]["hasMore"], true);
    assert_eq!(values(&reply)[0], "p000");
}

#[test]
fn another_reference_or_argument_is_32602_vo_12() {
    let mut gateway = stocked();
    let version = complete(
        &mut gateway,
        "version",
        "",
        Some(json!({"profile": "vocab", "package": "shapes.words"})),
    );
    assert_eq!(version.error()["code"], -32602, "{}", version.body);
    assert!(version.effects.is_empty());
    let other = gateway.rpc(
        "completion/complete",
        None,
        json!({"ref": {"type": "ref/resource", "uri": "lexlean://other/{x}"}, "argument": {"name": "profile", "value": ""}}),
    );
    assert_eq!(other.error()["code"], -32602, "{}", other.body);
    let prompt = gateway.rpc(
        "completion/complete",
        None,
        json!({"ref": {"type": "ref/prompt", "name": "test_prompt_with_arguments"}, "argument": {"name": "arg1", "value": ""}}),
    );
    assert_eq!(prompt.error()["code"], -32602, "{}", prompt.body);
    assert!(prompt.effects.is_empty());
}

// ---- VO-13: the capabilities ------------------------------------------------------------------

#[test]
fn server_discover_declares_the_served_capabilities_vo_13() {
    let mut gateway = stocked();
    let reply = gateway.rpc("server/discover", None, json!({}));
    let capabilities = &reply.result()["capabilities"];
    assert!(capabilities["tools"].is_object());
    assert!(capabilities["resources"].is_object());
    assert!(capabilities["completions"].is_object());
    assert!(
        capabilities.get("prompts").is_none(),
        "no prompt is offered: {capabilities}"
    );
    assert!(capabilities.get("logging").is_none());
    assert!(capabilities.get("tasks").is_none());
}

#[test]
fn a_method_of_every_declared_capability_is_served_vo_13() {
    let mut gateway = stocked();
    assert!(gateway.list().result()["tools"].is_array());
    assert!(gateway.rpc("resources/list", None, json!({})).result()["resources"].is_array());
    assert!(gateway
        .rpc("resources/templates/list", None, json!({}))
        .result()["resourceTemplates"]
        .is_array());
    assert!(
        read(&mut gateway, &uri("vocab", "shapes.words", "1.0.0")).result()["contents"].is_array()
    );
    assert!(complete(&mut gateway, "profile", "", None).result()["completion"].is_object());
}

#[test]
fn prompts_and_the_tasks_methods_are_not_declared_and_are_refused_vo_13() {
    let mut gateway = stocked();
    for (method, name, params) in [
        ("prompts/list", None, json!({})),
        ("prompts/get", Some("anything"), json!({"name": "anything"})),
        ("tasks/get", None, json!({"taskId": "x"})),
    ] {
        let reply = gateway.rpc(method, name, params);
        assert!(
            reply.body.get("error").is_some(),
            "{method}: {}",
            reply.body
        );
        assert!(reply.effects.is_empty(), "{method}: no effect");
    }
}

// ---- VO-14: the Atlas profile ----------------------------------------------------------------

/// The entry IDs of the model's Atlas data (`crates/model/tests/atlas`, held equal to
/// `src/Profiles.lex.tex` by PF-07), in the order of the vocabulary: by local ID.
fn atlas_ids() -> Vec<String> {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/model/tests/atlas/entries.tsv");
    let mut ids: Vec<String> = std::fs::read_to_string(path)
        .expect("the Atlas extract")
        .lines()
        .map(|line| line.split('\t').next().expect("an ID").to_string())
        .collect();
    ids.sort();
    ids
}

fn atlas_project() -> PathBuf {
    let sources = gateway_tool_tests::lexlean_sources();
    let project = sources.join("examples/uor-atlas");
    assert!(
        project.join("lexlean.toml").is_file(),
        "examples/uor-atlas at the pin"
    );
    project
}

#[test]
#[ignore = "needs LEXLEAN_SOURCES (gateway/lay-out-lexlean.sh) and the lexlean binary: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn an_atlas_profile_serves_all_293_entries_vo_14() {
    let mut gateway = Gateway::new();
    let installed = gateway.install_profile("uor-atlas", &atlas_project(), &[]);
    assert_eq!(installed.language, "1.0");
    let wanted = atlas_ids();
    assert_eq!(wanted.len(), 293);
    let listed = gateway.call("lexlean_profiles", json!({"profile": "uor-atlas"}));
    assert!(!listed.is_error(), "{}", listed.body);
    let profile = &listed.structured()["profiles"][0];
    assert_eq!(profile["language"], "1.0");
    let atlas = package_ids(profile)
        .into_iter()
        .find(|package| package.0 == "lexlean.uor.atlas")
        .expect("the Atlas lexicon is locked");
    assert_eq!(
        (atlas.1.as_str(), atlas.2.as_str(), atlas.3),
        ("1.0.0", "1.0", 293)
    );
    let mut seen = Vec::new();
    let mut offset = 0;
    loop {
        let page = search(
            &mut gateway,
            json!({"profile": "uor-atlas", "package": "lexlean.uor.atlas", "offset": offset, "limit": 100}),
        );
        assert!(!page.is_error(), "{}", page.body);
        assert_eq!(page.structured()["total"], 293);
        let ids = entry_ids(&page);
        if ids.is_empty() {
            break;
        }
        for entry in page.structured()["entries"].as_array().expect("entries") {
            assert_eq!(entry["denotes"]["kind"], "document");
            assert_eq!(entry["denotes"]["module"], "Atlas");
        }
        seen.extend(ids);
        offset += 100;
    }
    assert_eq!(seen, wanted, "exactly the model's 293 entries");
    let resource = read(
        &mut gateway,
        &uri("uor-atlas", "lexlean.uor.atlas", "1.0.0"),
    );
    let document: Value = serde_json::from_str(
        resource.result()["contents"][0]["text"]
            .as_str()
            .expect("text"),
    )
    .expect("JSON");
    assert_eq!(
        names(&document["entries"], "id"),
        wanted,
        "the resource serves the same entries"
    );
    let modules = std::fs::read_dir(gateway.profile("uor-atlas").join("src"))
        .expect("the source root of the profile")
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|e| e == "tex"))
        .count();
    assert_eq!(
        modules + 6,
        86,
        "80 modules at the top of the source root and 6 under Prelude"
    );
}

#[test]
#[ignore = "needs LEXLEAN_SOURCES (gateway/lay-out-lexlean.sh) and the lexlean binary: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn a_profile_without_the_module_atlas_serves_none_of_the_entries_vo_14() {
    let scratch = tempfile::tempdir().expect("a directory");
    let project = scratch.path().join("uor-atlas");
    let source = atlas_project();
    std::fs::create_dir_all(&project).expect("a directory");
    for file in [
        "lexlean.toml",
        "lexlean.lock",
        "lakefile.toml",
        "lean-toolchain",
        "lake-manifest.json",
    ] {
        std::fs::copy(source.join(file), project.join(file)).expect("a project file");
    }
    copy_tree(&source.join("src"), &project.join("src"));
    std::fs::remove_file(project.join("src/Atlas.lex.tex")).expect("without the module Atlas");
    let mut gateway = Gateway::new();
    gateway.install_profile("no-atlas", &project, &[]);
    let listed = gateway.call("lexlean_profiles", json!({"profile": "no-atlas"}));
    let atlas = package_ids(&listed.structured()["profiles"][0])
        .into_iter()
        .find(|package| package.0 == "lexlean.uor.atlas")
        .expect("the Atlas lexicon is locked");
    assert_eq!(atlas.3, 0, "no entry denotes a module the profile has");
    let page = search(
        &mut gateway,
        json!({"profile": "no-atlas", "package": "lexlean.uor.atlas"}),
    );
    assert_eq!(page.structured()["total"], 0);
}
