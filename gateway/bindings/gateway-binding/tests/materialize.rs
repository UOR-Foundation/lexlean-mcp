//! DR-07 and DR-08 at the level of the host: a draft and the bundle of its profile are copied
//! into a directory of the `builds` area that links nothing, and a bundle that holds a symbolic
//! link is refused before a byte of it is read.

use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use gateway_binding::materialize::{decode, materialize, split_handle, Refused, DOMAIN, RECORD};
use sha2::{Digest, Sha256};

const PROFILE: &str = "p";
const PROFILE_ID: [u8; 32] = [7; 32];
const LIMIT: usize = 8 * 1024 * 1024 + (1 << 20);

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
fn preimage(profile: &[u8; 32], rows: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = frame("domain", DOMAIN);
    out.extend(frame("profile", profile));
    out.extend_from_slice(&("sources".len() as u32).to_be_bytes());
    out.extend_from_slice(b"sources");
    out.extend_from_slice(&((rows.len() * 2) as u64).to_be_bytes());
    for (path, content) in rows {
        out.extend(frame("source", path.as_bytes()));
        out.extend(frame("source", content));
    }
    out
}

struct Store {
    directory: tempfile::TempDir,
}

impl Store {
    fn root(&self) -> PathBuf {
        self.directory.path().join("store")
    }

    fn bundle(&self) -> PathBuf {
        self.root().join("profiles").join(PROFILE)
    }

    fn builds(&self) -> PathBuf {
        self.root().join("builds")
    }

    fn builds_listing(&self) -> Vec<String> {
        match std::fs::read_dir(self.builds()) {
            Ok(entries) => entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Store the draft of `rows` for the profile and return its identity.
    fn draft(&self, rows: &[(&str, &[u8])]) -> String {
        let blob = preimage(&PROFILE_ID, rows);
        let id = hex(&Sha256::digest(&blob));
        let directory = self.root().join("drafts").join(PROFILE);
        std::fs::create_dir_all(&directory).expect("a directory");
        std::fs::write(directory.join(&id), blob).expect("a draft");
        id
    }
}

/// A store whose profile `p` holds its record and a bundle of a file and a subdirectory.
fn store() -> Store {
    let store = Store {
        directory: tempfile::tempdir().expect("a temporary directory"),
    };
    let bundle = store.bundle();
    std::fs::create_dir_all(bundle.join("language/sub")).expect("a bundle");
    std::fs::write(bundle.join(RECORD), format!("{}\nsrc\n", hex(&PROFILE_ID))).expect("a record");
    std::fs::write(bundle.join("lexlean.toml"), b"[package]\n").expect("a file");
    std::fs::write(bundle.join("language/sub/entry.toml"), b"entry\n").expect("a file");
    store
}

fn tree(directory: &Path, prefix: &str, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).expect("a directory").flatten() {
        let name = format!("{prefix}{}", entry.file_name().to_string_lossy());
        let kind = std::fs::symlink_metadata(entry.path())
            .expect("metadata")
            .file_type();
        assert!(!kind.is_symlink(), "{name} is a link");
        if kind.is_dir() {
            tree(&entry.path(), &format!("{name}/"), found);
        } else {
            found.push(name);
        }
    }
}

#[test]
fn a_handle_is_a_profile_name_and_64_lower_case_digits_dr_07() {
    let id = "ab".repeat(32);
    assert_eq!(split_handle(&format!("p/{id}")), Some(("p", id.as_str())));
    for handle in [
        "p".to_string(),
        format!("p/{}", "AB".repeat(32)),
        format!("p/{}", "ab".repeat(31)),
        format!("../{id}"),
        format!("p/{id}/x"),
        format!("/{id}"),
    ] {
        assert_eq!(split_handle(&handle), None, "{handle}");
    }
}

#[test]
fn the_preimage_is_decoded_to_its_profile_and_rows_dr_07() {
    let blob = preimage(
        &PROFILE_ID,
        &[("src/A.lex.tex", b"a"), ("src/B.lex.tex", b"")],
    );
    let (profile, rows) = decode(&blob).expect("a preimage");
    assert_eq!(profile, PROFILE_ID);
    assert_eq!(
        rows,
        vec![
            (b"src/A.lex.tex".as_slice(), b"a".as_slice()),
            (b"src/B.lex.tex".as_slice(), b"".as_slice())
        ]
    );
    let mut longer = blob.clone();
    longer.push(0);
    assert!(decode(&longer).is_none(), "a trailing byte");
    assert!(decode(&blob[..blob.len() - 1]).is_none(), "a missing byte");
    assert!(decode(b"").is_none());
}

#[test]
fn the_bundle_and_the_draft_are_copied_into_a_directory_of_their_own_dr_07() {
    let store = store();
    let id = store.draft(&[
        ("src/Main.lex.tex", b"main\n"),
        ("src/deep/Two.lex.tex", b"two\n"),
    ]);
    let project = materialize(&store.root(), PROFILE, &id, LIMIT).expect("a project");
    assert!(project.starts_with(store.builds()));
    let mut found = Vec::new();
    tree(&project, "", &mut found);
    found.sort();
    assert_eq!(
        found,
        [
            "language/sub/entry.toml",
            "lexlean.toml",
            "src/Main.lex.tex",
            "src/deep/Two.lex.tex"
        ],
        "the record of the profile is not a file of the bundle"
    );
    assert_eq!(
        std::fs::read(project.join("src/deep/Two.lex.tex")).expect("a file"),
        b"two\n"
    );
    assert_eq!(
        std::fs::read(project.join("lexlean.toml")).expect("a file"),
        b"[package]\n"
    );
}

#[test]
fn two_materializations_of_one_draft_are_two_directories_dr_07() {
    let store = store();
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    let first = materialize(&store.root(), PROFILE, &id, LIMIT).expect("a project");
    let second = materialize(&store.root(), PROFILE, &id, LIMIT).expect("a project");
    assert_ne!(first, second);
    std::fs::remove_dir_all(&first).expect("remove");
    assert!(second.join("src/Main.lex.tex").is_file());
}

#[test]
fn an_absent_draft_and_a_changed_draft_are_refused_dr_07() {
    let store = store();
    assert_eq!(
        materialize(&store.root(), PROFILE, &"0".repeat(64), LIMIT),
        Err(Refused::Draft)
    );
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    let file = store.root().join("drafts").join(PROFILE).join(&id);
    let mut blob = std::fs::read(&file).expect("a draft");
    let last = blob.len() - 1;
    blob[last] ^= 1;
    std::fs::write(&file, blob).expect("a change");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Draft)
    );
    assert!(store.builds_listing().is_empty(), "nothing is left behind");
}

#[test]
fn bytes_that_are_not_a_preimage_are_a_refused_draft_dr_07() {
    let store = store();
    let blob = b"not a preimage".to_vec();
    let id = hex(&Sha256::digest(&blob));
    let directory = store.root().join("drafts").join(PROFILE);
    std::fs::create_dir_all(&directory).expect("a directory");
    std::fs::write(directory.join(&id), blob).expect("a draft");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Draft)
    );
}

#[test]
fn a_draft_with_a_path_that_leaves_the_project_is_refused_dr_07() {
    let store = store();
    for path in [
        "../x.lex.tex",
        "/x.lex.tex",
        "a//b.lex.tex",
        "./x.lex.tex",
        "x.txt",
        "",
    ] {
        let id = store.draft(&[(path, b"x")]);
        assert_eq!(
            materialize(&store.root(), PROFILE, &id, LIMIT),
            Err(Refused::Draft),
            "{path:?}"
        );
    }
    assert!(store.builds_listing().is_empty());
    assert!(!store.directory.path().join("x.lex.tex").exists());
}

#[test]
fn an_absent_profile_and_a_record_of_another_profile_are_refused_dr_07() {
    let store = store();
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    std::fs::write(
        store.bundle().join(RECORD),
        format!("{}\nsrc\n", hex(&[8u8; 32])),
    )
    .expect("a record");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Profile),
        "the record states another profile ID"
    );
    std::fs::remove_file(store.bundle().join(RECORD)).expect("remove");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Profile),
        "no record"
    );
    std::fs::remove_dir_all(store.bundle()).expect("remove");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Profile),
        "no profile"
    );
    assert!(store.builds_listing().is_empty());
}

/// Put a link at `place` of the bundle, and expect the materialization to refuse it.
fn assert_a_link_is_refused(label: &str, place: &str, target: &Path) {
    let store = store();
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    let link = store.bundle().join(place);
    std::fs::create_dir_all(link.parent().expect("a parent")).expect("a parent");
    symlink(target, &link).expect("a link");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Link),
        "{label}"
    );
    assert!(
        store.builds_listing().is_empty(),
        "{label}: no directory of the builds area is left"
    );
}

#[test]
fn a_link_to_a_file_is_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    let target = outside.path().join("secret");
    std::fs::write(&target, b"secret").expect("a file");
    assert_a_link_is_refused("file", "linked.toml", &target);
}

#[test]
fn a_link_to_a_directory_is_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    assert_a_link_is_refused("directory", "linked", outside.path());
}

#[test]
fn a_dangling_link_is_refused_dr_08() {
    assert_a_link_is_refused("dangling", "dangling", Path::new("/nonexistent/target"));
}

#[test]
fn a_link_below_a_subdirectory_is_refused_dr_08() {
    let outside = tempfile::tempdir().expect("a directory");
    let target = outside.path().join("secret");
    std::fs::write(&target, b"secret").expect("a file");
    assert_a_link_is_refused("below a subdirectory", "language/sub/deeper/entry", &target);
}

#[test]
fn a_link_to_the_record_is_refused_dr_08() {
    let store = store();
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    let record = store.bundle().join(RECORD);
    let kept = store.bundle().join("record.copy");
    std::fs::rename(&record, &kept).expect("move");
    symlink(&kept, &record).expect("a link");
    assert_eq!(
        materialize(&store.root(), PROFILE, &id, LIMIT),
        Err(Refused::Profile),
        "a record that is a link is not read"
    );
}

#[test]
fn a_link_to_a_pipe_does_not_block_and_is_refused_dr_08() {
    // A link whose target is a pipe would block a reader that follows it; the refusal comes first.
    let outside = tempfile::tempdir().expect("a directory");
    let pipe = outside.path().join("pipe");
    let made = std::process::Command::new("mkfifo")
        .arg(&pipe)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    assert!(made, "mkfifo");
    assert_a_link_is_refused("pipe", "fifo", &pipe);
}

#[test]
fn a_bundle_without_a_link_is_materialized_dr_08() {
    let store = store();
    let id = store.draft(&[("src/Main.lex.tex", b"main\n")]);
    let project = materialize(&store.root(), PROFILE, &id, LIMIT).expect("a project");
    assert!(project.join("src/Main.lex.tex").is_file());
}
