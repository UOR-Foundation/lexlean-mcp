//! `QM-09`: a draft yields the same `build_id` on every supported host.

use std::path::{Path, PathBuf};

use camino::Utf8PathBuf;
use lexlean::{BuildRequest, Engine, Selection};

/// The IDs of one build, as hex.
#[derive(Debug, PartialEq, Eq)]
struct Ids {
    source: String,
    semantic: String,
    build: String,
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixture")
}

/// The IDs `fixture/expected` records, as a `key = "hex"` line each.
fn recorded() -> Ids {
    let text = std::fs::read_to_string(fixture().join("expected")).expect("fixture/expected");
    let value = |key: &str| {
        text.lines()
            .find_map(|line| line.strip_prefix(&format!("{key} = \"")))
            .and_then(|rest| rest.strip_suffix('"'))
            .unwrap_or_else(|| panic!("fixture/expected has no {key}"))
            .to_string()
    };
    Ids {
        source: value("source_id"),
        semantic: value("semantic_id"),
        build: value("build_id"),
    }
}

/// Copy `from` below `to`, without a build the project may hold.
fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a directory").flatten() {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy(&entry.path(), &target);
        } else if entry.file_name() != "expected" {
            std::fs::copy(entry.path(), target).expect("a file");
        }
    }
}

/// Build the fixture with the `Engine` below `directory`, which names no part of the result.
fn built_in(directory: &Path) -> Ids {
    let project = directory.join("project");
    copy(&fixture(), &project);
    let file = Utf8PathBuf::from_path_buf(project.join("lexlean.toml")).expect("a UTF-8 path");
    let engine = Engine::load(&file).expect("the project loads");
    let set = engine
        .build(BuildRequest {
            selection: Selection::Entrypoints,
        })
        .unwrap_or_else(|error| panic!("the fixture builds: {error:?}"));
    assert_eq!(set.units.len(), 1, "the one module of the fixture");
    Ids {
        source: set.source_id.to_hex(),
        semantic: set.semantic_id.to_hex(),
        build: set.build_id.expect("a build has a build ID").to_hex(),
    }
}

/// The IDs of the build equal the recorded ones. On `linux/amd64` and `linux/arm64` alike, the
/// recorded value is the same, so the two platforms agree through it.
#[test]
fn the_build_id_of_the_fixture_is_the_recorded_one_on_this_host_qm_09() {
    let directory = tempfile::tempdir().expect("a directory");
    let found = built_in(directory.path());
    assert_eq!(
        found,
        recorded(),
        "the build of the fixture on {}",
        std::env::consts::ARCH
    );
}

/// The build ID does not depend on where the project is: two directories, one ID.
#[test]
fn the_build_id_does_not_depend_on_the_directory_qm_09() {
    let first = tempfile::tempdir().expect("a directory");
    let second = tempfile::tempdir().expect("a directory");
    assert_eq!(built_in(first.path()), built_in(second.path()));
}

/// The comparison can fail: a source changed by one byte changes the source ID, and so the build
/// ID, and the recorded value rejects it.
#[test]
fn a_changed_source_is_not_the_recorded_build_id_qm_09() {
    let directory = tempfile::tempdir().expect("a directory");
    let project = directory.path().join("project");
    copy(&fixture(), &project);
    let source = project.join("src/Tiny.lex.tex");
    let text = std::fs::read_to_string(&source).expect("the module");
    std::fs::write(
        &source,
        text.replace("\\title{Boolean}", "\\title{Boolean }"),
    )
    .expect("a file");
    let file = Utf8PathBuf::from_path_buf(project.join("lexlean.toml")).expect("a UTF-8 path");
    let engine = Engine::load(&file).expect("the project loads");
    let set = engine
        .build(BuildRequest {
            selection: Selection::Entrypoints,
        })
        .unwrap_or_else(|error| panic!("the changed fixture builds: {error:?}"));
    let planted = set.build_id.expect("a build ID").to_hex();
    assert_ne!(planted, recorded().build);
}
