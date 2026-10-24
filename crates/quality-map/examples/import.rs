//! Rewrites `lexlean/` and `sources.toml` from a LexLean checkout.
//!
//! `cargo run -p quality-map --example import -- <LexLean checkout>`
//!
//! The checkout must be at the pinned commit. The example copies the six files
//! Q2 and Q8 are measured against, asks git for the blob id of each at the
//! pinned commit and requires it to equal the id it computes from the bytes it
//! copied, writes `sources.toml`, and prints the public functions it finds, so
//! that a change of LexLean's surface is seen when the pin moves.

use std::path::{Path, PathBuf};
use std::process::Command;

use negative_fixtures::{blob_id, sha256, PIN_COMMIT};
use quality_map::{derive, Source, Sources, ENTRIES, REGISTRY, SOURCES_SPEC, SURFACE};

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

/// `(path in LexLean, name under lexlean/, role, module, only)`.
type Row = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static [&'static str],
);

const ROWS: [Row; 6] = [
    ("ERRORS.md", "ERRORS.md", ENTRIES, "", &[]),
    (
        "crates/lexlean/src/api.rs",
        "api.rs.txt",
        SURFACE,
        "api",
        &[],
    ),
    (
        "crates/lexlean/src/api/production.rs",
        "api-production.rs.txt",
        SURFACE,
        "api::production",
        &[],
    ),
    (
        "crates/lexlean/src/sdk.rs",
        "sdk.rs.txt",
        SURFACE,
        "sdk",
        &[],
    ),
    (
        "crates/lexlean/src/verify/lifeline.rs",
        "verify-lifeline.rs.txt",
        SURFACE,
        "verify",
        &["end_live_children", "allow_children"],
    ),
    ("model/errors.toml", "errors.toml", REGISTRY, "", &[]),
];

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

    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let vendored_dir = here.join("lexlean");
    if vendored_dir.exists() {
        std::fs::remove_dir_all(&vendored_dir).expect("the old copy is removed");
    }
    std::fs::create_dir_all(&vendored_dir).expect("lexlean/ is created");

    let mut sources = Sources {
        spec: SOURCES_SPEC.to_owned(),
        commit: PIN_COMMIT.to_owned(),
        source: Vec::new(),
    };
    for (path, name, role, module, only) in ROWS {
        let bytes = std::fs::read(checkout.join(path)).expect("the file reads");
        let git_blob = git(&checkout, &["rev-parse", &format!("HEAD:{path}")]);
        assert_eq!(
            blob_id(&bytes),
            git_blob,
            "{path}: not the blob of the commit"
        );
        std::fs::write(vendored_dir.join(name), &bytes).expect("the copy is written");
        if role == SURFACE {
            let text = std::str::from_utf8(&bytes).expect("source is UTF-8");
            let only: Vec<String> = only.iter().map(|s| (*s).to_owned()).collect();
            for f in derive(module, &only, text) {
                println!("{}\t{}", f.path, f.signature);
            }
        }
        sources.source.push(Source {
            path: path.to_owned(),
            vendored: name.to_owned(),
            git_blob,
            sha256: sha256(&bytes),
            bytes: bytes.len() as u64,
            role: role.to_owned(),
            module: module.to_owned(),
            only: only.iter().map(|s| (*s).to_owned()).collect(),
        });
    }
    std::fs::write(here.join("sources.toml"), sources.render()).expect("sources.toml is written");
}
