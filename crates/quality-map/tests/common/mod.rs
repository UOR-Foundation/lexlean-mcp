//! What the tests of this crate read: the vendored files, the map, the model.

#![allow(dead_code)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use negative_fixtures::Register;
use quality_map::{load_files, Files, Map, Registry, Sources};
use repo_model::{AuthorityRow, Model};

pub fn crate_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn root() -> PathBuf {
    crate_dir()
        .ancestors()
        .nth(2)
        .expect("crates/quality-map is two below the root")
        .to_path_buf()
}

/// The commit `.github/lexlean.rev` pins.
pub fn rev_commit() -> String {
    let text = std::fs::read_to_string(root().join(".github/lexlean.rev")).expect("lexlean.rev");
    text.lines()
        .find_map(|l| l.strip_prefix("rev = "))
        .expect("lexlean.rev has a rev line")
        .trim()
        .to_owned()
}

pub fn sources() -> Sources {
    Sources::load(&crate_dir().join("sources.toml")).expect("sources.toml loads")
}

pub fn files() -> Files {
    load_files(&crate_dir().join("lexlean"), &sources()).expect("lexlean/ loads")
}

pub fn map() -> Map {
    Map::load(&crate_dir().join("map.toml")).expect("map.toml loads")
}

pub fn text_of(files: &Files, name: &str) -> String {
    String::from_utf8(files.get(name).expect("vendored").clone()).expect("UTF-8")
}

pub fn registry(files: &Files) -> Registry {
    Registry::parse(&text_of(files, "errors.toml")).expect("the registry parses")
}

pub fn quality() -> String {
    std::fs::read_to_string(root().join("src/Quality.lex.tex")).expect("Quality reads")
}

pub fn model() -> Model {
    Model::load(&root().join("model")).expect("model loads")
}

pub fn authority(id: &str) -> AuthorityRow {
    model()
        .authorities
        .authority
        .into_iter()
        .find(|a| a.id == id)
        .unwrap_or_else(|| panic!("{id} is not in model/authorities.toml"))
}

/// The diagnostic codes of the command results of the 297 negative fixtures.
pub fn returned_codes() -> BTreeSet<String> {
    let register = Register::load(&root().join("crates/negative-fixtures/register.toml"))
        .expect("the fixture register loads");
    register
        .fixture
        .iter()
        .flat_map(|f| f.codes.iter().cloned())
        .collect()
}

/// Panic with the violations when there are any.
pub fn none(what: &str, found: Vec<String>) {
    assert!(found.is_empty(), "{what}:\n{}", found.join("\n"));
}

/// Whether some violation contains `needle`.
pub fn reports(found: &[String], needle: &str) -> bool {
    found.iter().any(|v| v.contains(needle))
}
