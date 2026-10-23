//! Rewrites `lexlean/` and `register.toml` from a LexLean checkout.
//!
//! `cargo run -p negative-fixtures --example import -- <LexLean checkout>`
//!
//! The checkout must be at the pinned commit. The example copies `case.toml`
//! and `expected/*` of every directory of `tests/negative`, reads the git tree
//! of each `project/` and checks that the tree it recomputes for the fixture
//! directory is the one git reports, then writes the register. The classes come
//! from `class_of`, never from the checkout.

use std::path::{Path, PathBuf};
use std::process::Command;

use negative_fixtures::{derive, root_tree, Register, Vendored, PIN_COMMIT, PIN_TREE, SPEC};

fn git(checkout: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .arg("-C")
        .arg(checkout)
        .args(args)
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout)
        .expect("git prints UTF-8")
        .trim()
        .to_owned()
}

fn main() {
    let checkout = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("usage: import <LexLean checkout at the pinned commit>"),
    );
    assert_eq!(
        git(&checkout, &["rev-parse", "HEAD"]),
        PIN_COMMIT,
        "not the pinned commit"
    );
    assert_eq!(
        git(&checkout, &["rev-parse", "HEAD:tests/negative"]),
        PIN_TREE,
        "tests/negative is not the pinned tree"
    );

    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let vendored_dir = here.join("lexlean");
    if vendored_dir.exists() {
        std::fs::remove_dir_all(&vendored_dir).expect("the old copy is removed");
    }
    let source = checkout.join("tests/negative");
    let mut names: Vec<String> = std::fs::read_dir(&source)
        .expect("tests/negative reads")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for name in &names {
        let from = source.join(name);
        let to = vendored_dir.join(name);
        std::fs::create_dir_all(to.join("expected")).expect("directory");
        std::fs::copy(from.join("case.toml"), to.join("case.toml")).expect("case.toml copies");
        for file in std::fs::read_dir(from.join("expected")).expect("expected reads") {
            let file = file.expect("entry").file_name();
            std::fs::copy(
                from.join("expected").join(&file),
                to.join("expected").join(&file),
            )
            .expect("expected file copies");
        }
    }

    let vendored = Vendored::load(&vendored_dir).expect("the copy loads");
    let mut register = Register {
        spec: SPEC.to_owned(),
        commit: PIN_COMMIT.to_owned(),
        tree: PIN_TREE.to_owned(),
        fixture: Vec::new(),
    };
    for name in &names {
        let project_tree = git(
            &checkout,
            &["rev-parse", &format!("HEAD:tests/negative/{name}/project")],
        );
        let listing = git(
            &checkout,
            &["ls-tree", &format!("HEAD:tests/negative/{name}")],
        );
        let mut extra = std::collections::BTreeMap::new();
        for line in listing.lines() {
            let (meta, entry) = line.split_once('\t').expect("ls-tree prints a tab");
            let fields: Vec<&str> = meta.split(' ').collect();
            if ["case.toml", "expected", "project"].contains(&entry) {
                continue;
            }
            assert_eq!(fields[1], "tree", "{name}: `{entry}` is not a directory");
            extra.insert(entry.to_owned(), fields[2].to_owned());
        }
        let row = derive(name, &vendored, &project_tree, &extra).expect("the fixture derives");
        let reported = git(
            &checkout,
            &["rev-parse", &format!("HEAD:tests/negative/{name}")],
        );
        assert_eq!(
            row.tree, reported,
            "{name}: the recomputed tree is not git's"
        );
        register.fixture.push(row);
    }
    assert_eq!(root_tree(&register).expect("root tree"), PIN_TREE);
    std::fs::write(here.join("register.toml"), register.render()).expect("register writes");
    println!("import: {} fixtures", register.fixture.len());
}
