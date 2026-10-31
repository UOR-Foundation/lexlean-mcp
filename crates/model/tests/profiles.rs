//! PF-01 to PF-09: the model-side slice of the profiles and vocabulary of issue #8.
//!
//! The model is LexLean source (`src/Profiles.lex.tex`, on `src/Identity.lex.tex`)
//! and `lexlean verify` checks the kernel side. These tests read the same source
//! as text and hold it against what the issue states (the domain tag, the files a
//! profile identity covers, the language code, the Atlas counts), against the
//! pinned LexLean as copied under `crates/model/tests/atlas`, and against
//! `PROFILES.md`, so that a law dropped from the model, a fixture that no longer
//! differs from its base in the one way it names, a copy edited by hand or a
//! planted defect left unrecorded fails here and names the law.

use std::collections::BTreeSet;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn bytes(relative: &str) -> Vec<u8> {
    std::fs::read(root().join(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The source with runs of white space collapsed and braces tightened, so the
/// formatter's one-line and multi-line spellings of a declaration read the same.
fn tight(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("{ ", "{")
        .replace(" }", "}")
}

fn profiles() -> String {
    tight(&read("src/Profiles.lex.tex"))
}

/// Index of the brace that closes the one opened just before `text`.
fn closing_brace(text: &str) -> usize {
    let mut depth = 1usize;
    let mut in_string = false;
    for (index, c) in text.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            _ => {}
        }
    }
    panic!("a declaration is not closed");
}

/// The text of the declaration `kind{name: <name>, ...}`.
fn declaration(source: &str, kind: &str, name: &str) -> String {
    let marker = format!("{kind}{{name: {name},");
    let at = source
        .find(&marker)
        .unwrap_or_else(|| panic!("the source has no {kind} {name}"));
    let chunk = &source[at + kind.len() + 1..];
    chunk[..closing_brace(chunk)].to_string()
}

fn field<'a>(text: &'a str, marker: &str) -> &'a str {
    let at = text
        .find(marker)
        .unwrap_or_else(|| panic!("`{marker}` is not in `{text}`"));
    &text[at + marker.len()..]
}

fn theorems_exist(source: &str, names: &[&str]) {
    for name in names {
        let theorem = declaration(source, "theorem", name);
        assert!(theorem.contains("proof: "), "theorem {name} has no proof");
        assert!(
            !theorem.contains("sorry"),
            "theorem {name} is not proved: it mentions sorry"
        );
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> String {
    let digits = text.trim_start_matches("0x");
    let decoded: Vec<u8> = (0..digits.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&digits[at..at + 2], 16).expect("a hex literal"))
        .collect();
    String::from_utf8(decoded).expect("an ASCII literal")
}

/// The body of a definition without parameters.
fn body(source: &str, name: &str) -> String {
    let definition = declaration(source, "definition", name);
    let body = field(&definition, "body: ");
    let end = [", axioms:", ", executable:"]
        .iter()
        .filter_map(|stop| body.find(stop))
        .min()
        .unwrap_or(body.len());
    body[..end].to_string()
}

/// A profile file fixture `pf...`: its decoded (path, bytes).
fn file_fixture(source: &str, name: &str) -> (String, String) {
    let text = body(source, name);
    let inner = text.trim_start_matches('(').trim_end_matches(')');
    let (path, content) = inner.split_once(", ").expect("a pair");
    (unhex(path), unhex(content))
}

/// A list of fixture names `(cons (pfX) (cons (pfY) (nil ...)))` as decoded files.
fn file_list(source: &str, name: &str) -> Vec<(String, String)> {
    let text = body(source, name);
    text.split("(cons (")
        .skip(1)
        .map(|part| {
            let fixture = part.split(')').next().expect("a fixture name");
            file_fixture(source, fixture)
        })
        .collect()
}

fn digest_of(files: &[&str]) -> String {
    let mut manifest = String::new();
    for file in files {
        manifest.push_str(&format!(
            "{}  {file}\n",
            hex(&Sha256::digest(bytes(&format!(
                "crates/model/tests/atlas/{file}"
            ))))
        ));
    }
    hex(&Sha256::digest(manifest.as_bytes()))
}

/// PF-01: the domain tag is the issue's and the identity is `Identity.profileId`.
#[test]
fn the_identity_is_the_framed_domain_files_reviews_and_toolchain_pf_01() {
    let source = profiles();
    theorems_exist(&source, &["profileDomainIsTheIssuesTag"]);
    let theorem = declaration(&source, "theorem", "profileDomainIsTheIssuesTag");
    let list = field(&theorem, "statement: ((Identity::profileDomain) = ");
    let tag: String = list
        .split("(cons ")
        .skip(1)
        .map(|part| {
            let number: u8 = part.split(' ').next().unwrap().parse().expect("an octet");
            number as char
        })
        .collect();
    assert_eq!(tag, "lexlean-mcp-profile-v1", "the tag is the issue's");
    assert_eq!(tag.len(), 22);

    // the same tag as Identity spells it in its own definition
    let identity = tight(&read("src/Identity.lex.tex"));
    let domain = declaration(&identity, "definition", "profileDomain");
    let spelled: String = field(&domain, "body: ")
        .split("(cons ")
        .skip(1)
        .map(|part| {
            let number: u8 = part.split(' ').next().unwrap().parse().expect("an octet");
            number as char
        })
        .collect();
    assert_eq!(spelled, tag, "Identity frames the issue's tag");
    let framing = declaration(&identity, "definition", "encProfile");
    assert!(
        framing.contains("(encFrame (hDomain) domain)")
            && framing.contains("(encFrame (hToolchain) toolchain)")
            && framing.contains("(encList (hReviewsCount) (hReviewItem) reviews)"),
        "the preimage frames the domain, the files, the reviews and the toolchain"
    );

    let identity_of = declaration(&source, "definition", "profileIdOf");
    assert!(
        identity_of
            .contains("body: (Identity::profileId (profileFiles roots files) reviews toolchain)"),
        "the identity of a project is Identity.profileId over its profile files"
    );
}

/// PF-02: roots match whole components.
#[test]
fn a_root_matches_whole_components_pf_02() {
    let source = profiles();
    theorems_exist(&source, &["rootsMatchWholeComponents"]);
    assert_eq!(body(&source, "separator"), "47", "the separator is `/`");
    let theorem = declaration(&source, "theorem", "rootsMatchWholeComponents");
    let mut expected = Vec::new();
    for part in theorem.split("(underRoot ").skip(1) {
        let mut words = part.split(' ');
        let root = unhex(words.next().unwrap());
        let path = unhex(words.next().unwrap().trim_end_matches(')'));
        expected.push((root, path));
    }
    assert_eq!(
        expected,
        vec![
            ("src".to_string(), "src/Main.lex.tex".to_string()),
            ("src".to_string(), "src/deep/Other.lex.tex".to_string()),
            ("src".to_string(), "srcs/Main.lex.tex".to_string()),
            ("src".to_string(), "src".to_string()),
            ("src".to_string(), "lexlean.toml".to_string()),
        ],
        "the five paths of the claim, two under the root and three not"
    );
    let under = declaration(&source, "definition", "underRoot");
    assert!(
        under.contains("(c =? (separator))"),
        "the test needs the separator after the root"
    );
}

/// PF-03: each variant differs from the base in the one way it names.
#[test]
fn every_change_to_a_profile_file_changes_the_identity_pf_03() {
    let source = profiles();
    let base = file_list(&source, "baseFiles");
    assert_eq!(base.len(), 4, "the four profile file kinds of the issue");
    let paths: Vec<&str> = base.iter().map(|(path, _)| path.as_str()).collect();
    assert_eq!(
        paths,
        [
            "lexlean.toml",
            "lexlean.lock",
            "language/pack/lexicon.toml",
            "artifacts/data.bin"
        ],
        "configuration, lock, a local lexicon package file and an artifact_source file"
    );

    // the kinds exist in this repository's own project file
    let project = read("lexlean.toml");
    assert!(project.contains("[[lexicon_source]]") && project.contains("kind = \"path\""));
    assert!(project.contains("[[artifact_source]]"));
    assert!(project.contains("source_roots = [\"src\"]"));

    let differs_in_bytes_only = |name: &str, at: usize| {
        let variant = file_list(&source, name);
        assert_eq!(variant.len(), base.len(), "{name} keeps four files");
        for (index, (left, right)) in base.iter().zip(&variant).enumerate() {
            if index == at {
                assert_eq!(left.0, right.0, "{name} keeps the path");
                assert_ne!(left.1, right.1, "{name} changes the bytes");
            } else {
                assert_eq!(left, right, "{name} leaves file {index} alone");
            }
        }
    };
    differs_in_bytes_only("changedConfig", 0);
    differs_in_bytes_only("changedLock", 1);
    differs_in_bytes_only("changedLexicon", 2);
    differs_in_bytes_only("changedArtifact", 3);

    let renamed = file_list(&source, "renamedConfig");
    assert_ne!(renamed[0].0, base[0].0, "a renamed file has another path");
    assert_eq!(renamed[0].1, base[0].1, "and the same bytes");
    assert_eq!(&renamed[1..], &base[1..]);

    let added = file_list(&source, "addedFile");
    assert_eq!(&added[..4], &base[..], "an added file leaves the four");
    assert_eq!(added.len(), 5);
    let removed = file_list(&source, "removedFile");
    assert_eq!(&removed[..], &base[..3], "a removed file leaves three");

    let lookalike = file_list(&source, "withLookalike");
    assert_eq!(lookalike[4].0, "srcs/Main.lex.tex");
    let bare = file_list(&source, "withBareRoot");
    assert_eq!(bare[4].0, "src");

    let preimage_theorems = [
        "changedBytesOfConfigChangeThePreimage",
        "changedBytesOfLockChangeThePreimage",
        "changedBytesOfLexiconChangeThePreimage",
        "changedBytesOfArtifactChangeThePreimage",
        "aRenamedFileChangesThePreimage",
        "anAddedFileChangesThePreimage",
        "aRemovedFileChangesThePreimage",
        "aLookalikeDirectoryCounts",
        "aFileNamedLikeTheRootCounts",
    ];
    theorems_exist(&source, &preimage_theorems);
    for theorem in preimage_theorems {
        assert!(
            declaration(&source, "theorem", theorem)
                .contains("((otherFraming (framingOf (baseFiles)) (framingOf ("),
            "{theorem} compares with the preimage of the base"
        );
    }

    // two digests are computed, on the smallest project, because SHA-256 in the kernel is costly
    theorems_exist(
        &source,
        &[
            "changedBytesOfConfigChangeTheDigest",
            "aRenamedFileChangesTheDigest",
        ],
    );
    for theorem in [
        "changedBytesOfConfigChangeTheDigest",
        "aRenamedFileChangesTheDigest",
    ] {
        assert!(
            declaration(&source, "theorem", theorem)
                .contains("((otherId (idOf (tinyBase)) (idOf ("),
            "{theorem} compares two profile IDs"
        );
    }
    let tiny = file_list(&source, "tinyBase");
    let changed = file_list(&source, "tinyChanged");
    let renamed_tiny = file_list(&source, "tinyRenamed");
    assert_eq!(tiny.len(), 1);
    assert_eq!(
        (&tiny[0].0, &changed[0].0),
        (&tiny[0].0, &tiny[0].0),
        "bytes only"
    );
    assert_ne!(tiny[0].1, changed[0].1);
    assert_ne!(tiny[0].0, renamed_tiny[0].0);
    assert_eq!(tiny[0].1, renamed_tiny[0].1);
}

/// PF-04: a source file is not a profile file.
#[test]
fn a_source_file_is_not_a_profile_file_pf_04() {
    let source = profiles();
    theorems_exist(
        &source,
        &[
            "sourceFilesAreNotProfileFiles",
            "profileFilesKeepEverythingOutsideTheRoots",
            "aSourceFileNeverChangesTheId",
            "aDeeperSourceFileNeverChangesTheId",
            "aSourceFileLeavesThePreimageAlone",
        ],
    );
    for name in [
        "aSourceFileNeverChangesTheId",
        "aDeeperSourceFileNeverChangesTheId",
    ] {
        let theorem = declaration(&source, "theorem", name);
        assert!(
            theorem.contains(
                "parameters: [files : (List (Bytes * Bytes)), reviews : (List Bytes), toolchain : Bytes]"
            ),
            "{name} holds for every file list, review list and toolchain"
        );
    }
    let sources = file_list(&source, "withSources");
    let under_src: Vec<&str> = sources
        .iter()
        .map(|(path, _)| path.as_str())
        .filter(|path| path.starts_with("src/"))
        .collect();
    assert_eq!(under_src, ["src/Main.lex.tex", "src/deep/Other.lex.tex"]);
    let filter = declaration(&source, "definition", "profileFiles");
    assert!(
        filter.contains("(if (insideRoots roots (first file)) then (profileFiles roots rest) else (cons file (profileFiles roots rest)))"),
        "a file under a root is dropped and every other file is kept in place"
    );
}

/// PF-05: the order of the rows does not matter.
#[test]
fn the_order_of_the_rows_does_not_matter_pf_05() {
    let source = profiles();
    theorems_exist(
        &source,
        &[
            "rowOrder2GivesTheSamePreimage",
            "rowOrder3GivesTheSamePreimage",
            "rowOrder4GivesTheSamePreimage",
            "rowOrder5GivesTheSamePreimage",
            "rowOrder2GivesTheSameId",
            "idOfIsTheProfileId",
            "aDuplicatePathIsRefused",
        ],
    );
    let mut base = file_list(&source, "baseFiles");
    base.sort();
    let mut orders = BTreeSet::new();
    for name in ["order2", "order3", "order4"] {
        let order = file_list(&source, name);
        assert_ne!(order, file_list(&source, "baseFiles"), "{name} reorders");
        let mut sorted = order.clone();
        sorted.sort();
        assert_eq!(sorted, base, "{name} holds the same four files");
        orders.insert(order);
    }
    assert_eq!(orders.len(), 3, "three different orders");
    let with_source = file_list(&source, "order5");
    assert_eq!(with_source.len(), 5);
    assert!(with_source
        .iter()
        .any(|(path, _)| path == "src/Main.lex.tex"));
    let duplicate = file_list(&source, "duplicatePath");
    let paths: BTreeSet<&String> = duplicate.iter().map(|(path, _)| path).collect();
    assert!(paths.len() < duplicate.len(), "a path is repeated");

    // Identity proves the general permutation laws this concrete case rests on
    let identity = tight(&read("src/Identity.lex.tex"));
    for name in [
        "canonicalizePermutation1",
        "canonicalizePermutation6",
        "canonicalizeRefusesDuplicate",
    ] {
        theorems_exist(&identity, &[name]);
    }
}

/// PF-06: one language per profile, LLC0103 for another.
#[test]
fn a_package_of_another_language_is_refused_with_llc0103_pf_06() {
    let source = profiles();
    assert_eq!(body(&source, "languageMismatchCode"), "\"LLC0103\"");
    theorems_exist(
        &source,
        &[
            "everyLanguagePairIsDecided",
            "anotherLanguageIsRefusedWithLLC0103",
            "samePackageLanguagesAreAdmitted",
        ],
    );
    let table = body(&source, "languageTable");
    for language in ["1.0", "1.1", "1.2"] {
        for other in ["1.0", "1.1", "1.2"] {
            let pair = format!("(checkPackage \"{language}\" \"{other}\")");
            assert!(table.contains(&pair), "the table lacks the pair {pair}");
        }
    }
    let owner = tight(&read("src/OwnerValues.lex.tex"));
    let admitted = declaration(&owner, "definition", "languageAdmitted");
    for language in ["1.0", "1.1", "1.2"] {
        assert!(
            admitted.contains(&format!("prim equal language \"{language}\"")),
            "the owner values admit language {language}"
        );
    }

    // the pinned LexLean returns this code, for this reason
    let diagnostics = read(
        "crates/negative-fixtures/lexlean/package-language-mismatch/expected/diagnostics.json",
    );
    assert!(diagnostics.contains("\"code\":\"LLC0103\""));
    assert!(
        diagnostics.contains("lexicon language `1.2` does not match project language `1.1`"),
        "the fixture states a lexicon language that differs from the project's"
    );
    let register = read("crates/negative-fixtures/register.toml");
    let at = register
        .find("name = \"package-language-mismatch\"")
        .expect("the fixture is registered");
    let row = &register[at..register[at..]
        .find("[[fixture]]")
        .map_or(register.len(), |n| at + n)];
    assert!(row.contains("codes = [\"LLC0103\"]"), "{row}");
    assert!(read("crates/quality-map/lexlean/errors.toml").contains("LLC0103"));
}

/// PF-07: the Atlas data is the pinned LexLean's.
#[test]
fn the_atlas_data_is_the_pinned_lexleans_pf_07() {
    let model = repo_model::Model::load_from_repo_root().expect("the model loads");
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-LEXLEAN-ATLAS")
        .expect("model/authorities.toml names A-LEXLEAN-ATLAS");
    assert_eq!(
        digest_of(&["entries.tsv", "lexicon.toml", "modules.txt"]),
        authority.checksum,
        "the Atlas extracts differ from the pinned LexLean: a copy is edited here, or the authority moved"
    );
    assert!(authority
        .citation
        .contains("e0b74b81e6f49c83596a957d232c8383a5db1a25"));
    assert_eq!(
        std::fs::read_to_string(root().join(".github/lexlean.rev"))
            .unwrap()
            .lines()
            .find_map(|line| line.strip_prefix("rev = "))
            .unwrap()
            .trim(),
        "e0b74b81e6f49c83596a957d232c8383a5db1a25",
        "the authority is the revision this repository pins"
    );

    let source = profiles();
    let lexicon = read("crates/model/tests/atlas/lexicon.toml");
    for (key, definition) in [
        ("package", "atlasPackage"),
        ("version", "atlasVersion"),
        ("language", "atlasLanguage"),
    ] {
        let line = lexicon
            .lines()
            .find(|line| line.starts_with(&format!("{key} = ")))
            .unwrap_or_else(|| panic!("lexicon.toml has no {key}"));
        assert_eq!(
            line.split_once(" = ").unwrap().1,
            body(&source, definition),
            "the model's {definition} is the lexicon's {key}"
        );
    }
    assert_eq!(body(&source, "atlasLanguage"), "\"1.0\"");

    let entries = read("crates/model/tests/atlas/entries.tsv");
    let rows: Vec<Vec<&str>> = entries
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 293);
    assert_eq!(body(&source, "atlasEntryCount"), "293");
    let ids: BTreeSet<&str> = rows.iter().map(|row| row[0]).collect();
    assert_eq!(ids.len(), 293, "the entry ids are distinct");
    for row in &rows {
        assert_eq!(row.len(), 5);
        assert_eq!(row[2], "document", "{} is a document denotation", row[0]);
        assert_eq!(row[3], "Atlas", "{} denotes the module Atlas", row[0]);
    }

    let modules_text = read("crates/model/tests/atlas/modules.txt");
    let modules: Vec<&str> = modules_text.lines().collect();
    assert_eq!(modules.len(), 86);
    assert_eq!(body(&source, "atlasModuleCount"), "86");
    assert_eq!(
        modules.iter().filter(|m| m.starts_with("Prelude.")).count(),
        6
    );
    assert_eq!(
        modules
            .iter()
            .filter(|m| !m.starts_with("Prelude."))
            .count(),
        80
    );
    assert!(modules.contains(&"Atlas"));
    let distinct: BTreeSet<&&str> = modules.iter().collect();
    assert_eq!(distinct.len(), 86);

    // the model's data, chunk by chunk, is the copy, in order
    let mut model_ids = Vec::new();
    let mut model_modules = Vec::new();
    for chunk in 0..=12 {
        let name = format!("atlasEntries{chunk:02}");
        if !source.contains(&format!("definition{{name: {name},")) {
            break;
        }
        let text = body(&source, &name);
        for part in text.split("{id := \"").skip(1) {
            let (id, rest) = part.split_once("\", denotes := \"").unwrap();
            let (denotes, _) = rest.split_once('"').unwrap();
            assert_eq!(denotes, "Atlas");
            model_ids.push(id.to_string());
        }
    }
    for chunk in 0..=4 {
        let name = format!("atlasModules{chunk:02}");
        if !source.contains(&format!("definition{{name: {name},")) {
            break;
        }
        for part in body(&source, &name).split("(cons \"").skip(1) {
            model_modules.push(part.split('"').next().unwrap().to_string());
        }
    }
    let copy_ids: Vec<String> = rows.iter().map(|row| row[0].to_string()).collect();
    assert_eq!(model_ids, copy_ids, "the model's entry ids are the copy's");
    let copy_modules: Vec<String> = modules.iter().map(|m| m.to_string()).collect();
    assert_eq!(
        model_modules, copy_modules,
        "the model's modules are the copy's"
    );
}

/// PF-08: an Atlas profile serves all 293 entries.
#[test]
fn an_atlas_profile_serves_all_293_entries_pf_08() {
    let source = profiles();
    theorems_exist(
        &source,
        &[
            "theAtlasProfileIsLanguageOneZero",
            "theAtlasProfileRefusesALanguageOnePackage",
            "theAtlasProfileHasTheIssuesCounts",
            "everyAtlasEntryDenotesTheAtlasModule",
            "theAtlasModuleIsInTheProjectsModules",
            "anAtlasProfileServesAll293Entries",
            "withoutTheAtlasModuleNoEntryIsServed",
            "anEntryIntoAMissingModuleIsNotServed",
            "aMissingModuleIsDetected",
            "profilesModelHoldsTheorem",
        ],
    );
    assert!(
        declaration(&source, "theorem", "anAtlasProfileServesAll293Entries")
            .contains("((servedCount (atlasModules) (atlasEntries)) = 293)")
    );
    assert!(
        declaration(&source, "theorem", "withoutTheAtlasModuleNoEntryIsServed").contains("= 0)")
    );
    assert_eq!(body(&source, "atlasModuleName"), "\"Atlas\"");

    // the owner values put the Atlas profile at language 1.0
    let owner = tight(&read("src/OwnerValues.lex.tex"));
    assert!(owner.contains("(record ProfileSpec {name := \"uor-atlas\", language := \"1.0\"})"));
    assert_eq!(body(&source, "atlasProfileName"), "\"uor-atlas\"");

    // the entry into a missing module is the one entry that is not served
    assert!(body(&source, "unknownEntry").contains("denotes := \"Missing\""));
    let modules = read("crates/model/tests/atlas/modules.txt");
    assert!(!modules.lines().any(|m| m == "Missing"));

    // the profile set of the issue's definition is served as one
    let model = declaration(&source, "definition", "profilesModelHolds");
    assert!(model.contains("(servesAll (atlasModules) (atlasEntries))"));
    assert!(model.contains("(languageTable)"));
}

/// PF-09: every law has a recorded planted defect.
#[test]
fn every_law_has_a_recorded_planted_defect_pf_09() {
    let doc = read("PROFILES.md");
    let rows: Vec<Vec<String>> = doc
        .lines()
        .filter(|line| line.starts_with("| P") && !line.starts_with("| PF-"))
        .map(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect()
        })
        .collect();
    assert_eq!(rows.len(), 4, "four planted defects are recorded");
    let source = profiles();
    for row in &rows {
        assert_eq!(row.len(), 5, "a row has five cells: {row:?}");
        for cell in row {
            assert!(!cell.is_empty(), "an empty cell in {row:?}");
        }
        let law = row[1].trim_matches('`');
        declaration(&source, "theorem", law);
        assert!(
            row[3].contains("lexlean verify") || row[3].contains("LLV"),
            "the diagnostic of {} comes from lexlean verify",
            row[0]
        );
    }
    let laws: BTreeSet<&str> = rows.iter().map(|row| row[1].trim_matches('`')).collect();
    assert_eq!(laws.len(), 4, "the four defects break four different laws");
}
