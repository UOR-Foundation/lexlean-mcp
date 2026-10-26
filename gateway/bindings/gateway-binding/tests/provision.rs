//! VO-01, VO-02, VO-04 to VO-08 and VO-15 at the level of the host: the operator provisions a
//! profile with the commands of `gateway-provision` (`gateway_binding::provision`), and the
//! profile ID that the host records is `Identity.profileId` of the model
//! (`gateway_binding::identity`). The fixtures are the projects of `gateway/vectors/fixture`,
//! locked by the compiler of the pin; every test works on a copy.

use std::collections::BTreeMap;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use std::process::Command;

use gateway_binding::identity;
use gateway_binding::provision::{
    identify, init, install, lock, needs_review, toolchain, Installed, Refusal, CATALOGUE,
    CATALOGUE_AREA, REVIEWS, VOCABULARY, VOCABULARY_LIMIT,
};
use sha2::{Digest, Sha256};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../vectors/fixture")
}

/// A scratch directory with a copy of the fixture project `name` in `project/` and a store path
/// `store/` that does not exist yet.
struct Scratch {
    directory: tempfile::TempDir,
}

impl Scratch {
    fn new(name: &str) -> Scratch {
        let scratch = Scratch {
            directory: tempfile::tempdir().expect("a temporary directory"),
        };
        copy_tree(&fixtures().join(name), &scratch.project());
        scratch
    }

    fn project(&self) -> PathBuf {
        self.directory.path().join("project")
    }

    fn store(&self) -> PathBuf {
        self.directory.path().join("store")
    }

    fn profile(&self, name: &str) -> PathBuf {
        self.store().join("profiles").join(name)
    }

    fn review(&self) -> Vec<u8> {
        std::fs::read(fixtures().join("vocab.review")).expect("the review record of the fixture")
    }

    fn review_file(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let path = self.directory.path().join(name);
        std::fs::write(&path, bytes).expect("a review file");
        path
    }

    /// Install the project as `name` with the review records in the files given.
    fn install(&self, name: &str, reviews: &[&Path]) -> Result<Installed, Refusal> {
        install(&self.store(), name, &self.project(), reviews)
    }

    /// Install `vocab` with its fixture review record.
    fn install_vocab(&self, name: &str) -> Installed {
        let review = self.review_file("vocab.review", &self.review());
        self.install(name, &[&review])
            .expect("the fixture installs")
    }

    fn edit(&self, relative: &str, change: impl FnOnce(Vec<u8>) -> Vec<u8>) {
        let path = self.project().join(relative);
        let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{relative}: {error}"));
        std::fs::write(&path, change(bytes)).expect("an edit");
    }
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a directory").flatten() {
        let target = to.join(entry.file_name());
        if entry.file_type().expect("a type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

/// Every file beneath `root` with its bytes, by path; a link is its target's name.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, at: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let relative = path
                .strip_prefix(base)
                .expect("inside")
                .to_string_lossy()
                .into_owned();
            let kind = std::fs::symlink_metadata(&path)
                .expect("metadata")
                .file_type();
            if kind.is_symlink() {
                out.insert(relative, b"<link>".to_vec());
            } else if kind.is_dir() {
                out.insert(format!("{relative}/"), Vec::new());
                walk(base, &path, out);
            } else {
                out.insert(relative, std::fs::read(&path).expect("a file"));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn refusal(result: Result<Installed, Refusal>) -> String {
    match result {
        Ok(installed) => panic!("installed {}", installed.name),
        Err(refused) => refused.code(),
    }
}

// ---- the model's vectors -----------------------------------------------------------------------

/// The statement of the theorem `name` in `src/Identity.lex.tex`.
fn model_statement(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../src/Identity.lex.tex");
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

/// The hexadecimal digits that follow `octetsOfBytes 0x` in a statement.
fn expected_preimage(statement: &str) -> String {
    statement
        .split("octetsOfBytes 0x")
        .nth(1)
        .expect("the expected octets")
        .split(')')
        .next()
        .expect("hexadecimal digits")
        .to_string()
}

/// The eight words `hN := (V : UInt32)` of the digest in a statement, as 64 hexadecimal digits.
fn digest_hex(statement: &str) -> String {
    let words: Vec<u32> = statement
        .split('h')
        .skip(1)
        .filter_map(|part| {
            let rest = part.strip_prefix(|c: char| c.is_ascii_digit())?;
            let value = rest.strip_prefix(" := (")?;
            value.split(' ').next()?.parse().ok()
        })
        .collect();
    assert_eq!(words.len(), 8, "a digest has eight words");
    words.iter().map(|word| format!("{word:08x}")).collect()
}

fn row(path: &str, bytes: &str) -> (Vec<u8>, Vec<u8>) {
    (path.as_bytes().to_vec(), bytes.as_bytes().to_vec())
}

/// The files, the review and the toolchain of `profilePreimageVector`.
fn vector_inputs() -> (Vec<(Vec<u8>, Vec<u8>)>, Vec<Vec<u8>>, Vec<u8>) {
    (
        vec![row("lexlean.lock", "L"), row("lexlean.toml", "T")],
        vec![b"r1".to_vec()],
        b"lexlean-v0.4".to_vec(),
    )
}

// ---- VO-01 ---------------------------------------------------------------------------------------

#[test]
fn the_host_reproduces_the_models_profile_vectors_vo_01() {
    let (files, reviews, toolchain) = vector_inputs();
    let preimage = identity::preimage(&files, &reviews, &toolchain).expect("a preimage");
    assert_eq!(
        hex(&preimage),
        expected_preimage(&model_statement("profilePreimageVector")),
        "the preimage of profilePreimageVector"
    );
    let id = identity::profile_id(&files, &reviews, &toolchain).expect("an ID");
    assert_eq!(
        hex(&id),
        digest_hex(&model_statement("profileIdVector")),
        "the ID of profileIdVector"
    );
    assert_eq!(
        hex(&id),
        digest_hex(&model_statement("profileDigestVector")),
        "and the digest of profileDigestVector"
    );
    let mut changed = files.clone();
    changed[1].1 = b"U".to_vec();
    assert_eq!(
        hex(&identity::profile_id(&changed, &reviews, &toolchain).expect("an ID")),
        digest_hex(&model_statement("changedFileProfileId")),
        "the ID of changedFileProfileId"
    );
}

#[test]
fn the_host_reproduces_the_models_review_framing_vo_01() {
    let statement = model_statement("profileFramingReviewsVector");
    let files = vec![row("lexlean.lock", "L"), row("lexlean.toml", "T")];
    let reviews = vec![b"r2".to_vec(), b"r1a".to_vec(), b"r1".to_vec()];
    let preimage = identity::preimage(&files, &reviews, b"lexlean-v0.4").expect("a preimage");
    assert_eq!(hex(&preimage), expected_preimage(&statement));
    let sorted = vec![b"r1".to_vec(), b"r1a".to_vec(), b"r2".to_vec()];
    assert_eq!(
        preimage,
        identity::preimage(&files, &sorted, b"lexlean-v0.4").expect("a preimage"),
        "the order of the reviews does not matter"
    );
    let mut backwards = files.clone();
    backwards.reverse();
    assert_eq!(
        preimage,
        identity::preimage(&backwards, &reviews, b"lexlean-v0.4").expect("a preimage"),
        "nor does the order of the files"
    );
}

#[test]
fn the_toolchain_of_the_id_is_the_compilers_vo_01() {
    let lock_bytes = std::fs::read(fixtures().join("vocab/lexlean.lock")).expect("a lock");
    let wanted = "compiler_semantics 81a5f530241bed4ca42585e2345d91be731fe46121284fbe4c199ea356489b10\nlean_toolchain leanprover/lean4:v4.32.1\n";
    assert_eq!(toolchain(&lock_bytes), Some(wanted.as_bytes().to_vec()));
    assert_eq!(
        toolchain(b"spec = \"lexlean/lock/2\"\n"),
        None,
        "a lock without the header"
    );
    let scratch = Scratch::new("vocab");
    let identified = identify(&scratch.project(), &[scratch.review()]).expect("an ID");
    assert!(
        String::from_utf8_lossy(&identified.preimage).contains(wanted),
        "the toolchain frame of the preimage holds the lock's header"
    );
}

#[test]
fn the_recomputation_in_python_agrees_with_the_host_vo_01() {
    let scratch = Scratch::new("vocab");
    let generate = fixtures().join("../generate.py");
    let review = scratch.review_file("vocab.review", &scratch.review());
    let output = Command::new("python3")
        .arg("-I")
        .arg(&generate)
        .arg("profile-id")
        .arg(scratch.project())
        .arg(&review)
        .output()
        .expect("python3 runs generate.py");
    assert!(
        output.status.success(),
        "generate.py profile-id: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let identified = identify(&scratch.project(), &[scratch.review()]).expect("an ID");
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        hex(&identified.id)
    );
}

// ---- VO-02 ---------------------------------------------------------------------------------------

fn id_of(scratch: &Scratch) -> String {
    hex(&identify(&scratch.project(), &[scratch.review()])
        .expect("an ID")
        .id)
}

#[test]
fn a_byte_of_each_profile_file_changes_the_id_vo_02() {
    let original = id_of(&Scratch::new("vocab"));
    for file in [
        "lexlean.toml",
        "lexlean.lock",
        "language/shapes.words/lexicon.toml",
        "language/shapes.words/entries/shapes-circle.toml",
        "language/shapes.words/entries/shapes-ghost.toml",
        "artifacts/notes.bin",
    ] {
        let scratch = Scratch::new("vocab");
        scratch.edit(file, |mut bytes| {
            let last = bytes.len() - 1;
            bytes[last] ^= 1;
            bytes
        });
        assert_ne!(id_of(&scratch), original, "a byte of {file}");
    }
}

#[test]
fn a_renamed_an_added_and_a_removed_file_change_the_id_vo_02() {
    let original = id_of(&Scratch::new("vocab"));
    let scratch = Scratch::new("vocab");
    let entries = scratch.project().join("language/shapes.words/entries");
    std::fs::rename(
        entries.join("shapes-ghost.toml"),
        entries.join("shapes-ghosts.toml"),
    )
    .expect("a rename");
    assert_ne!(id_of(&scratch), original, "a renamed file");
    let scratch = Scratch::new("vocab");
    std::fs::write(
        scratch
            .project()
            .join("language/shapes.words/entries/extra.toml"),
        b"x",
    )
    .expect("an added file");
    assert_ne!(id_of(&scratch), original, "an added file");
    let scratch = Scratch::new("vocab");
    std::fs::remove_file(
        scratch
            .project()
            .join("language/shapes.words/entries/shapes-ghost.toml"),
    )
    .expect("a removed file");
    assert_ne!(id_of(&scratch), original, "a removed file");
}

#[test]
fn a_review_record_changes_the_id_vo_02() {
    let scratch = Scratch::new("vocab");
    let with = hex(&identify(&scratch.project(), &[scratch.review()])
        .expect("an ID")
        .id);
    let without = hex(&identify(&scratch.project(), &[]).expect("an ID").id);
    let mut other = scratch.review();
    *other.last_mut().expect("bytes") = b'\n';
    other.insert(other.len() - 1, b'!');
    let changed = hex(&identify(&scratch.project(), &[other]).expect("an ID").id);
    assert_ne!(with, without, "a review record");
    assert_ne!(with, changed, "the bytes of a review record");
}

#[test]
fn a_source_file_leaves_the_id_alone_vo_02() {
    let original = id_of(&Scratch::new("vocab"));
    let scratch = Scratch::new("vocab");
    scratch.edit("src/Main.lex.tex", |mut bytes| {
        bytes.extend_from_slice(b"% more\n");
        bytes
    });
    std::fs::create_dir_all(scratch.project().join("src/deeper")).expect("a directory");
    std::fs::write(scratch.project().join("src/deeper/Other.lex.tex"), b"x").expect("a source");
    assert_eq!(id_of(&scratch), original, "files under the source root");
    let outside = Scratch::new("vocab");
    std::fs::create_dir_all(outside.project().join("notes")).expect("a directory");
    std::fs::write(outside.project().join("notes/README.md"), b"x").expect("a file");
    std::fs::write(outside.project().join("README.md"), b"x").expect("a file");
    assert_eq!(
        id_of(&outside),
        original,
        "a file that is none of the four kinds of profile file is not a row"
    );
}

#[test]
fn install_records_the_id_the_host_computes_vo_02() {
    let scratch = Scratch::new("vocab");
    let expected = id_of(&scratch);
    let installed = scratch.install_vocab("vocab");
    assert_eq!(hex(&installed.id), expected);
    let record =
        std::fs::read_to_string(scratch.profile("vocab").join("profile")).expect("a record");
    assert_eq!(record, format!("{expected}\nsrc\n"));
}

// ---- VO-04 ---------------------------------------------------------------------------------------

#[test]
fn a_package_of_another_language_is_refused_at_install_with_llc0103_vo_04() {
    let scratch = Scratch::new("vocab");
    scratch.edit("language/shapes.words/lexicon.toml", |bytes| {
        String::from_utf8(bytes)
            .expect("text")
            .replace("language = \"1.2\"", "language = \"1.1\"")
            .into_bytes()
    });
    let review = scratch.review_file("vocab.review", &scratch.review());
    assert_eq!(refusal(scratch.install("vocab", &[&review])), "LLC0103");
    assert_eq!(
        lock(&scratch.project(), false)
            .expect_err("lock refuses")
            .code(),
        "LLC0103",
        "the compiler's own refusal at lock"
    );
    assert!(
        !scratch.store().join("profiles/vocab").exists(),
        "nothing installed"
    );
    assert!(!scratch.store().join("index").exists(), "no catalogue");
}

// ---- VO-05 ---------------------------------------------------------------------------------------

#[test]
fn init_makes_a_project_that_locks_and_installs_vo_05() {
    let scratch = tempfile::tempdir().expect("a directory");
    let project = scratch.path().join("made");
    init(&project, "made", "Made", "1.2").expect("init");
    for file in [
        "lexlean.toml",
        "lexlean.lock",
        "lakefile.toml",
        "lake-manifest.json",
        "lean-toolchain",
        "src/Main.lex.tex",
    ] {
        assert!(project.join(file).is_file(), "{file}");
    }
    lock(&project, false).expect("lock");
    let store = scratch.path().join("store");
    let installed =
        install(&store, "made", &project, &[]).expect("a project of builtin packages installs");
    assert_eq!(installed.language, "1.2");
    assert!(
        init(&project, "made", "Made", "1.2").is_err(),
        "init does not overwrite a project"
    );
}

#[test]
fn a_stale_lock_is_llc0102_and_install_does_not_write_it_vo_05() {
    let scratch = Scratch::new("vocab");
    scratch.edit(
        "language/shapes.words/entries/shapes-circle.toml",
        |bytes| {
            String::from_utf8(bytes)
                .expect("text")
                .replace("Shapes Circle", "Shapes Disc")
                .into_bytes()
        },
    );
    let before = std::fs::read(scratch.project().join("lexlean.lock")).expect("a lock");
    let review = scratch.review_file("vocab.review", &scratch.review());
    assert_eq!(refusal(scratch.install("vocab", &[&review])), "LLC0102");
    assert_eq!(
        std::fs::read(scratch.project().join("lexlean.lock")).expect("a lock"),
        before,
        "install left the lock as it found it"
    );
    lock(&scratch.project(), false).expect("the operator locks");
    assert_ne!(
        std::fs::read(scratch.project().join("lexlean.lock")).expect("a lock"),
        before,
        "lock writes it"
    );
}

#[test]
fn a_missing_lock_is_refused_and_not_made_vo_05() {
    let scratch = Scratch::new("vocab");
    std::fs::remove_file(scratch.project().join("lexlean.lock")).expect("a removed lock");
    let review = scratch.review_file("vocab.review", &scratch.review());
    let code = refusal(scratch.install("vocab", &[&review]));
    assert!(code.starts_with("LLC"), "a compiler code: {code}");
    assert!(
        !scratch.project().join("lexlean.lock").exists(),
        "no lock was made"
    );
}

#[test]
fn allow_network_is_accepted_by_lock_alone_vo_05() {
    let scratch = Scratch::new("vocab");
    let binary = env!("CARGO_BIN_EXE_gateway-provision");
    let run = |arguments: &[&str]| {
        Command::new(binary)
            .args(arguments)
            .output()
            .expect("the provisioning binary")
    };
    let project = scratch.project();
    let project = project.to_str().expect("a path");
    let store = scratch.store();
    let store = store.to_str().expect("a path");
    let made = scratch.directory.path().join("made");
    let made = made.to_str().expect("a path");
    assert!(run(&["lock", project]).status.success(), "lock");
    assert!(
        run(&["lock", project, "--allow-network"]).status.success(),
        "lock --allow-network"
    );
    assert_eq!(
        run(&["lock", project, "--allow-network", "--check"])
            .status
            .code(),
        Some(2),
        "lock has no --check"
    );
    for arguments in [
        vec![
            "install",
            "--store",
            store,
            "--allow-network",
            "vocab",
            project,
        ],
        vec![
            "init",
            made,
            "--name",
            "m",
            "--module-prefix",
            "M",
            "--allow-network",
        ],
        vec!["identify", project, "--allow-network"],
    ] {
        let output = run(&arguments);
        assert_eq!(
            output.status.code(),
            Some(2),
            "{arguments:?} is a usage error"
        );
        assert!(output.stdout.is_empty(), "{arguments:?} prints nothing");
    }
    assert!(
        !Path::new(store).exists(),
        "a refused command wrote nothing"
    );
}

#[test]
fn the_binary_provisions_a_profile_vo_05() {
    let scratch = Scratch::new("vocab");
    let binary = env!("CARGO_BIN_EXE_gateway-provision");
    let review = scratch.review_file("vocab.review", &scratch.review());
    let output = Command::new(binary)
        .args(["install", "--store"])
        .arg(scratch.store())
        .args(["vocab"])
        .arg(scratch.project())
        .arg("--review")
        .arg(&review)
        .output()
        .expect("the provisioning binary");
    assert!(
        output.status.success(),
        "install: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        id_of(&scratch),
        "the binary prints the profile ID"
    );
    let refused = Command::new(binary)
        .args(["install", "--store"])
        .arg(scratch.store())
        .args(["vocab"])
        .arg(scratch.project())
        .arg("--review")
        .arg(&review)
        .output()
        .expect("the provisioning binary");
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr).trim(),
        "refused ProfileExists",
        "the binary prints the code of the refusal"
    );
}

// ---- VO-06 ---------------------------------------------------------------------------------------

fn digest_of_package(scratch: &Scratch) -> String {
    let lock = std::fs::read_to_string(scratch.project().join("lexlean.lock")).expect("a lock");
    let after = lock
        .split("id = \"shapes.words\"")
        .nth(1)
        .expect("the package row");
    after
        .lines()
        .find_map(|line| line.strip_prefix("tree_sha256 = \""))
        .expect("a tree digest")
        .trim_end_matches('"')
        .to_string()
}

fn review_text(package: &str, digest: &str, reviewer: &str) -> String {
    format!("lexlean-mcp-review-v1\npackage {package}\ntree_sha256 {digest}\nreviewer {reviewer}\n")
}

#[test]
fn a_path_package_without_a_review_is_refused_vo_06() {
    let scratch = Scratch::new("vocab");
    assert_eq!(refusal(scratch.install("vocab", &[])), "ReviewMissing");
    assert!(
        !scratch.store().join("profiles").exists(),
        "nothing was installed"
    );
}

#[test]
fn a_valid_review_installs_and_is_stored_by_its_digest_vo_06() {
    let scratch = Scratch::new("vocab");
    let installed = scratch.install_vocab("vocab");
    let record = scratch.review();
    let stored = scratch
        .profile("vocab")
        .join(REVIEWS)
        .join(hex(&Sha256::digest(&record)));
    assert_eq!(std::fs::read(&stored).expect("the stored record"), record);
    assert_eq!(
        installed.packages.len(),
        4,
        "core, bool, nat and shapes.words"
    );
}

#[test]
fn a_review_of_another_digest_is_stale_vo_06() {
    let scratch = Scratch::new("vocab");
    let other = review_text("shapes.words@1.0.0", &"0".repeat(64), "maintainer");
    let file = scratch.review_file("stale.review", other.as_bytes());
    assert_eq!(refusal(scratch.install("vocab", &[&file])), "ReviewStale");
}

#[test]
fn a_review_that_names_no_locked_package_is_stale_vo_06() {
    let scratch = Scratch::new("vocab");
    let digest = digest_of_package(&scratch);
    let good = scratch.review_file("good.review", &scratch.review());
    let other = review_text("shapes.other@1.0.0", &digest, "maintainer");
    let stray = scratch.review_file("stray.review", other.as_bytes());
    assert_eq!(
        refusal(scratch.install("vocab", &[&good, &stray])),
        "ReviewStale"
    );
    let version = review_text("shapes.words@2.0.0", &digest, "maintainer");
    let wrong = scratch.review_file("version.review", version.as_bytes());
    assert_eq!(
        refusal(scratch.install("vocab", &[&good, &wrong])),
        "ReviewStale"
    );
}

#[test]
fn a_malformed_review_is_refused_vo_06() {
    let scratch = Scratch::new("vocab");
    let digest = digest_of_package(&scratch);
    let good = review_text("shapes.words@1.0.0", &digest, "maintainer");
    let cases: Vec<(&str, String)> = vec![
        (
            "three lines",
            good.lines().take(3).collect::<Vec<_>>().join("\n") + "\n",
        ),
        ("no final line feed", good.trim_end().to_string()),
        ("a fifth line", good.clone() + "more\n"),
        ("another first line", good.replace("review-v1", "review-v2")),
        ("a carriage return", good.replace('\n', "\r\n")),
        (
            "an upper case digest",
            good.replace(&digest, &digest.to_uppercase()),
        ),
        ("a short digest", good.replace(&digest, &digest[..63])),
        ("no package version", good.replace("@1.0.0", "")),
        (
            "an empty reviewer",
            good.replace("reviewer maintainer", "reviewer "),
        ),
        ("swapped lines", {
            let mut lines: Vec<&str> = good.lines().collect();
            lines.swap(1, 2);
            lines.join("\n") + "\n"
        }),
        ("an empty file", String::new()),
    ];
    for (name, text) in cases {
        let file = scratch.review_file("bad.review", text.as_bytes());
        assert_eq!(
            refusal(scratch.install("vocab", &[&file])),
            "ReviewMalformed",
            "{name}"
        );
    }
    let invalid = scratch.review_file("utf8.review", &[0xff, 0xfe, b'\n']);
    assert_eq!(
        refusal(scratch.install("vocab", &[&invalid])),
        "ReviewMalformed",
        "bytes that are not UTF-8"
    );
}

#[test]
fn a_repeated_review_is_refused_vo_06() {
    let scratch = Scratch::new("vocab");
    let first = scratch.review_file("first.review", &scratch.review());
    let second = scratch.review_file("second.review", &scratch.review());
    assert_eq!(
        refusal(scratch.install("vocab", &[&first, &second])),
        "ReviewDuplicate"
    );
    assert_eq!(
        refusal(scratch.install("vocab", &[&first, &first])),
        "ReviewDuplicate"
    );
}

#[test]
fn a_builtin_package_needs_no_review_vo_06() {
    let scratch = Scratch::new("tiny");
    scratch
        .install("tiny", &[])
        .expect("a project of builtin packages");
    assert!(!needs_review("builtin"));
    assert!(needs_review("path"));
    assert!(needs_review("git"));
    assert!(
        needs_review("anything else"),
        "an unknown kind needs a review"
    );
}

// ---- VO-07 ---------------------------------------------------------------------------------------

#[test]
fn install_lays_out_the_profile_vo_07() {
    let scratch = Scratch::new("vocab");
    std::fs::create_dir_all(scratch.project().join(".lexlean/build")).expect("build output");
    std::fs::write(scratch.project().join(".lexlean/build/x"), b"x").expect("build output");
    std::fs::create_dir_all(scratch.project().join(".lake/packages")).expect("lake output");
    std::fs::create_dir_all(scratch.project().join(".git")).expect("git state");
    std::fs::write(scratch.project().join(".git/HEAD"), b"ref").expect("git state");
    std::fs::create_dir_all(scratch.project().join("notes")).expect("notes");
    std::fs::write(scratch.project().join("notes/extra.txt"), b"x").expect("notes");
    std::fs::create_dir_all(scratch.project().join(".lexlean/cache/git/abc/def")).expect("a cache");
    std::fs::write(
        scratch
            .project()
            .join(".lexlean/cache/git/abc/def/lexicon.toml"),
        b"c",
    )
    .expect("a cached package");
    let installed = scratch.install_vocab("vocab");
    let found = tree(&scratch.profile("vocab"));
    let names: Vec<&str> = found.keys().map(String::as_str).collect();
    for wanted in [
        "profile",
        VOCABULARY,
        "lexlean.toml",
        "lexlean.lock",
        "lakefile.toml",
        "lean-toolchain",
        "lake-manifest.json",
        "src/Main.lex.tex",
        "language/shapes.words/lexicon.toml",
        "language/shapes.words/entries/shapes-circle.toml",
        "artifacts/notes.bin",
        ".lexlean/cache/git/abc/def/lexicon.toml",
    ] {
        assert!(
            names.contains(&wanted),
            "{wanted} is in the profile: {names:?}"
        );
    }
    assert!(
        !names.iter().any(|name| name.starts_with(".git")
            || name.starts_with(".lake")
            || name.starts_with(".lexlean/build")
            || name.starts_with("notes")),
        "no VCS, Lake or build output and nothing outside the project's closure: {names:?}"
    );
    assert_eq!(
        names
            .iter()
            .filter(|name| name.starts_with("reviews/") && !name.ends_with('/'))
            .count(),
        1,
        "one review record"
    );
    let catalogue: serde_json::Value = serde_json::from_slice(
        &std::fs::read(scratch.store().join(CATALOGUE_AREA).join(CATALOGUE))
            .expect("the catalogue"),
    )
    .expect("JSON");
    let listed = &catalogue["profiles"][0];
    assert_eq!(listed["name"], "vocab");
    assert_eq!(listed["id"], hex(&installed.id).as_str());
    assert_eq!(listed["language"], "1.2");
    assert_eq!(listed["roots"], serde_json::json!(["src"]));
    assert_eq!(listed["packages"].as_array().expect("packages").len(), 4);
    assert!(
        std::fs::read_dir(scratch.store().join("profiles"))
            .expect("profiles")
            .flatten()
            .all(|entry| !entry.file_name().to_string_lossy().starts_with('.')),
        "no staging directory is left"
    );
}

#[test]
fn a_name_that_is_not_a_key_segment_is_refused_vo_07() {
    let scratch = Scratch::new("tiny");
    let long = "a".repeat(65);
    for name in ["", ".hidden", "a/b", "a b", "ü", "..", &long] {
        assert_eq!(
            refusal(scratch.install(name, &[])),
            "NameInvalid",
            "{name:?}"
        );
    }
    assert!(!scratch.store().join("profiles").exists());
    scratch
        .install(&"a".repeat(64), &[])
        .expect("64 characters are a name");
}

#[test]
fn a_name_already_installed_is_refused_and_the_store_is_unchanged_vo_07() {
    let scratch = Scratch::new("tiny");
    scratch.install("tiny", &[]).expect("a first install");
    let before = tree(&scratch.store());
    assert_eq!(refusal(scratch.install("tiny", &[])), "ProfileExists");
    assert_eq!(tree(&scratch.store()), before);
}

#[test]
fn a_link_in_the_project_is_refused_vo_07() {
    for place in [
        "src/link",
        "language/shapes.words/link",
        "language/shapes.words/entries/link",
        "artifacts/link",
        "dangling",
        "src/deep/link",
    ] {
        let scratch = Scratch::new("vocab");
        let link = scratch.project().join(place);
        std::fs::create_dir_all(link.parent().expect("a parent")).expect("a directory");
        let target = if place == "dangling" {
            "/nonexistent"
        } else {
            "lexlean.toml"
        };
        symlink(target, &link).expect("a link");
        let review = scratch.review_file("vocab.review", &scratch.review());
        assert_eq!(
            refusal(scratch.install("vocab", &[&review])),
            "LinkInBundle",
            "{place}"
        );
        assert!(
            !scratch.store().join("profiles").exists(),
            "{place}: nothing installed"
        );
    }
}

#[test]
fn a_reserved_entry_is_refused_vo_07() {
    for (name, directory) in [("profile", false), ("vocabulary", false), ("reviews", true)] {
        let scratch = Scratch::new("vocab");
        let path = scratch.project().join(name);
        if directory {
            std::fs::create_dir(&path).expect("a directory");
        } else {
            std::fs::write(&path, b"x").expect("a file");
        }
        let review = scratch.review_file("vocab.review", &scratch.review());
        assert_eq!(
            refusal(scratch.install("vocab", &[&review])),
            "ReservedEntry",
            "{name}"
        );
    }
}

#[test]
fn a_project_with_source_packages_is_refused_vo_07() {
    let scratch = Scratch::new("vocab");
    scratch.edit("lexlean.toml", |bytes| {
        String::from_utf8(bytes)
            .expect("text")
            .replace(
                "[limits]",
                "[[source_package]]\npackage = \"std\"\nversion = \"1.0.0\"\nkind = \"path\"\npath = \"stdlib\"\n\n[limits]",
            )
            .into_bytes()
    });
    let review = scratch.review_file("vocab.review", &scratch.review());
    assert_eq!(
        refusal(scratch.install("vocab", &[&review])),
        "SourcePackagesUnsupported"
    );
}

#[test]
fn a_project_that_is_not_a_directory_or_has_no_configuration_is_refused_vo_07() {
    let scratch = Scratch::new("tiny");
    let missing = scratch.directory.path().join("missing");
    assert_eq!(
        install(&scratch.store(), "tiny", &missing, &[])
            .expect_err("refused")
            .code(),
        "ProjectUnreadable"
    );
    std::fs::remove_file(scratch.project().join("lexlean.toml")).expect("a removed file");
    assert_eq!(refusal(scratch.install("tiny", &[])), "ProjectUnreadable");
    let file = scratch.directory.path().join("file");
    std::fs::write(&file, b"x").expect("a file");
    assert_eq!(
        install(&scratch.store(), "tiny", &file, &[])
            .expect_err("refused")
            .code(),
        "ProjectUnreadable"
    );
    let absent_review = scratch.directory.path().join("absent.review");
    assert_eq!(
        install(
            &scratch.store(),
            "tiny",
            &scratch.project(),
            &[&absent_review]
        )
        .expect_err("refused")
        .code(),
        "ProjectUnreadable",
        "a review file that cannot be read"
    );
}

#[test]
fn a_refused_install_leaves_the_store_as_it_was_vo_07() {
    let scratch = Scratch::new("vocab");
    let other = Scratch::new("tiny");
    install(&scratch.store(), "tiny", &other.project(), &[]).expect("a first profile");
    let before = tree(&scratch.store());
    let stale = review_text("shapes.words@1.0.0", &"1".repeat(64), "maintainer");
    let file = scratch.review_file("stale.review", stale.as_bytes());
    assert_eq!(refusal(scratch.install("vocab", &[&file])), "ReviewStale");
    assert_eq!(refusal(scratch.install("vocab", &[])), "ReviewMissing");
    assert_eq!(refusal(scratch.install("tiny", &[])), "ProfileExists");
    assert_eq!(tree(&scratch.store()), before);
}

#[test]
fn materialization_copies_the_bundle_without_the_derived_entries_vo_07() {
    use gateway_binding::materialize::{materialize, DOMAIN};
    let scratch = Scratch::new("vocab");
    let installed = scratch.install_vocab("vocab");
    let frame = |head: &str, payload: &[u8]| {
        let mut out = (head.len() as u32).to_be_bytes().to_vec();
        out.extend_from_slice(head.as_bytes());
        out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
        out.extend_from_slice(payload);
        out
    };
    let mut blob = frame("domain", DOMAIN);
    blob.extend(frame("profile", &installed.id));
    blob.extend_from_slice(&("sources".len() as u32).to_be_bytes());
    blob.extend_from_slice(b"sources");
    blob.extend_from_slice(&2u64.to_be_bytes());
    blob.extend(frame("source", b"src/New.lex.tex"));
    blob.extend(frame("source", b"new"));
    let id = hex(&Sha256::digest(&blob));
    let drafts = scratch.store().join("drafts/vocab");
    std::fs::create_dir_all(&drafts).expect("a directory");
    std::fs::write(drafts.join(&id), &blob).expect("a draft");
    let project = materialize(&scratch.store(), "vocab", &id, 8 << 20).expect("materialized");
    let found = tree(&project);
    for absent in ["profile", VOCABULARY, "reviews/"] {
        assert!(
            !found
                .keys()
                .any(|name| name == absent || name.starts_with("reviews/")),
            "{absent}"
        );
    }
    for present in [
        "lexlean.toml",
        "src/New.lex.tex",
        "language/shapes.words/lexicon.toml",
    ] {
        assert!(found.contains_key(present), "{present}");
    }
}

// ---- VO-08 ---------------------------------------------------------------------------------------

fn vocabulary_of(scratch: &Scratch, name: &str) -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(scratch.profile(name).join(VOCABULARY)).expect("the vocabulary"),
    )
    .expect("JSON")
}

#[test]
fn the_vocabulary_lists_every_locked_package_and_the_served_entries_vo_08() {
    let scratch = Scratch::new("vocab");
    scratch.install_vocab("vocab");
    let vocabulary = vocabulary_of(&scratch, "vocab");
    assert_eq!(vocabulary["profile"], "vocab");
    assert_eq!(vocabulary["language"], "1.2");
    let packages: Vec<(String, String, String, u64)> = vocabulary["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .map(|package| {
            (
                package["package"].as_str().expect("id").to_string(),
                package["version"].as_str().expect("version").to_string(),
                package["language"].as_str().expect("language").to_string(),
                package["entries"].as_u64().expect("count"),
            )
        })
        .collect();
    let ids: Vec<&str> = packages.iter().map(|p| p.0.as_str()).collect();
    assert_eq!(
        ids,
        [
            "lexlean.core",
            "lexlean.std.bool",
            "lexlean.std.nat",
            "shapes.words"
        ]
    );
    assert!(packages.iter().all(|p| p.2 == "1.2"), "one language");
    let shapes = packages
        .iter()
        .find(|p| p.0 == "shapes.words")
        .expect("shapes.words");
    assert_eq!(
        shapes.3, 2,
        "the ghost entry denotes a module the project lacks"
    );
    let entries = vocabulary["entries"].as_array().expect("entries");
    let total: u64 = packages.iter().map(|p| p.3).sum();
    assert_eq!(entries.len() as u64, total, "the counts are the entries");
    let served: Vec<&str> = entries
        .iter()
        .filter(|entry| entry["package"] == "shapes.words")
        .map(|entry| entry["id"].as_str().expect("id"))
        .collect();
    assert_eq!(served, ["shapes-circle", "shapes-square"]);
    let circle = entries
        .iter()
        .find(|entry| entry["id"] == "shapes-circle")
        .expect("circle");
    assert_eq!(circle["version"], "1.0.0");
    assert_eq!(circle["category"], "label-word");
    assert_eq!(circle["frame"], "atom");
    assert_eq!(circle["arity"], 0);
    assert_eq!(
        circle["denotes"],
        serde_json::json!({"kind": "document", "module": "Main", "component": "shapes-circleValid"})
    );
    assert_eq!(
        circle["forms"],
        serde_json::json!([{"id": "shapes-circle", "channel": "text", "surface": "Shapes Circle", "canonical": true, "features": ["sentence-case", "singular"]}])
    );
    let order: Vec<(&str, &str)> = entries
        .iter()
        .map(|entry| {
            (
                entry["package"].as_str().expect("p"),
                entry["id"].as_str().expect("id"),
            )
        })
        .collect();
    let mut sorted = order.clone();
    sorted.sort();
    assert_eq!(order, sorted, "package and local ID order");
}

#[test]
fn a_vocabulary_over_the_bound_is_refused_vo_08() {
    let scratch = Scratch::new("vocab");
    let entries = scratch.project().join("language/shapes.words/entries");
    let surface = "word ".repeat(80);
    for index in 0..4000 {
        let text = format!(
            "spec = \"lexlean/entry/1\"\nid = \"bulk-{index:05}\"\ncategory = \"label-word\"\nsurface_arity = 0\nframe = \"atom\"\n\n[denotation]\nkind = \"document\"\nmodule = \"Main\"\ncomponent = \"bulk{index:05}Valid\"\n\n[[form]]\nid = \"bulk-{index:05}\"\nchannel = \"text\"\nsurface = \"{}\"\ncanonical_source = true\nfeatures = [\"sentence-case\", \"singular\"]\n",
            surface.trim_end()
        );
        std::fs::write(entries.join(format!("bulk-{index:05}.toml")), text).expect("an entry");
    }
    lock(&scratch.project(), false).expect("the operator locks");
    let digest = digest_of_package(&scratch);
    let review = scratch.review_file(
        "big.review",
        review_text("shapes.words@1.0.0", &digest, "m").as_bytes(),
    );
    assert_eq!(
        refusal(scratch.install("vocab", &[&review])),
        "VocabularyTooLarge"
    );
    assert_eq!(VOCABULARY_LIMIT, 1 << 20);
    assert!(!scratch.store().join("profiles/vocab").exists());
}

// ---- VO-15 ---------------------------------------------------------------------------------------

#[test]
fn a_duplicate_path_and_a_duplicate_review_are_refused_by_the_identity_vo_15() {
    let files = vec![row("a", "1"), row("a", "2")];
    assert!(matches!(
        identity::preimage(&files, &[], b"t"),
        Err(identity::Refused::DuplicatePath)
    ));
    let reviews = vec![b"r".to_vec(), b"r".to_vec()];
    assert!(matches!(
        identity::preimage(&[row("a", "1")], &reviews, b"t"),
        Err(identity::Refused::DuplicateReview)
    ));
}

#[test]
fn a_store_that_cannot_be_written_is_refused_vo_15() {
    let scratch = Scratch::new("tiny");
    let blocked = scratch.directory.path().join("blocked");
    std::fs::write(&blocked, b"a file where the store should be").expect("a file");
    assert_eq!(
        install(&blocked, "tiny", &scratch.project(), &[])
            .expect_err("refused")
            .code(),
        "StoreUnwritable"
    );
    assert_eq!(
        Refusal::from(identity::Refused::DuplicatePath).code(),
        "IdentityRefused"
    );
    assert_eq!(
        Refusal::from(identity::Refused::FieldTooLong).code(),
        "IdentityRefused"
    );
}

#[test]
fn every_refusal_has_a_code_and_the_compiler_refusals_carry_the_compilers_vo_15() {
    let codes: Vec<String> = [
        Refusal::NameInvalid,
        Refusal::ProfileExists,
        Refusal::ProjectUnreadable,
        Refusal::Compiler("LLC0103".to_string()),
        Refusal::LinkInBundle,
        Refusal::ReservedEntry,
        Refusal::SourcePackagesUnsupported,
        Refusal::ReviewMissing,
        Refusal::ReviewMalformed,
        Refusal::ReviewStale,
        Refusal::ReviewDuplicate,
        Refusal::VocabularyTooLarge,
        Refusal::IdentityRefused,
        Refusal::StoreUnwritable,
    ]
    .iter()
    .map(Refusal::code)
    .collect();
    assert_eq!(
        codes,
        [
            "NameInvalid",
            "ProfileExists",
            "ProjectUnreadable",
            "LLC0103",
            "LinkInBundle",
            "ReservedEntry",
            "SourcePackagesUnsupported",
            "ReviewMissing",
            "ReviewMalformed",
            "ReviewStale",
            "ReviewDuplicate",
            "VocabularyTooLarge",
            "IdentityRefused",
            "StoreUnwritable",
        ]
    );
}
