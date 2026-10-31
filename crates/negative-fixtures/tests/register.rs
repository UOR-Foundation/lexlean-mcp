//! FX-01 to FX-08: the AC-07 fixture register of issue #10.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use negative_fixtures::{
    check_classes, check_derived, check_exclusions, check_pin, root_tree, Counts, Register,
    Vendored, EXCLUDED, F4, F7, ISSUE_EXCLUSIONS, PIN_COMMIT, PIN_TREE, VERIFY,
};
use repo_model::{AuthorityRow, Model};

fn crate_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn root() -> PathBuf {
    crate_dir()
        .ancestors()
        .nth(2)
        .expect("crates/negative-fixtures is two below the root")
        .to_path_buf()
}

fn register() -> Register {
    static REGISTER: OnceLock<Register> = OnceLock::new();
    REGISTER
        .get_or_init(|| {
            Register::load(&crate_dir().join("register.toml")).expect("register.toml loads")
        })
        .clone()
}

fn vendored() -> Vendored {
    static VENDORED: OnceLock<Vendored> = OnceLock::new();
    VENDORED
        .get_or_init(|| Vendored::load(&crate_dir().join("lexlean")).expect("lexlean/ loads"))
        .clone()
}

fn model() -> Model {
    Model::load(&root().join("model")).expect("the model loads")
}

fn statement(model: &Model, id: &str) -> String {
    model
        .ids
        .get(id)
        .unwrap_or_else(|| panic!("{id} is registered"))
        .statement
        .clone()
}

fn authority(model: &Model) -> AuthorityRow {
    model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-LEXLEAN-NEGATIVE")
        .expect("A-LEXLEAN-NEGATIVE is a cited authority")
        .clone()
}

fn by_command(register: &Register, command: &str) -> usize {
    register
        .fixture
        .iter()
        .filter(|f| f.command == command)
        .count()
}

fn by_reason(register: &Register, reason: &str) -> usize {
    register
        .fixture
        .iter()
        .filter(|f| f.reason == reason)
        .count()
}

/// The sentence FX-01 states, from the register.
fn commands_sentence(register: &Register) -> String {
    format!(
        "{} fixtures, of which {} run the command check, {} build, {} migrate, {} verify, {} lock and {} production",
        register.fixture.len(),
        by_command(register, "check"),
        by_command(register, "build"),
        by_command(register, "migrate"),
        by_command(register, "verify"),
        by_command(register, "lock"),
        by_command(register, "production"),
    )
}

/// The sentence FX-06 states, from the counts.
fn counts_sentence(counts: &Counts) -> String {
    format!(
        "{} fixtures: {} for F4, {} for F7, {} for #11 and {} excluded, so {} reach a tool",
        counts.total,
        counts.f4,
        counts.f7,
        counts.verify,
        counts.excluded,
        counts.in_scope()
    )
}

/// The sentence FX-07 states, from the register.
fn reasons_sentence(register: &Register) -> String {
    format!(
        "{} for CLI argument parsing, {} for symlinks, {} for setup and {} for being outside the product",
        by_reason(register, "CLI argument parsing"),
        by_reason(register, "symlinks"),
        by_reason(register, "setup"),
        by_reason(register, "outside the product"),
    )
}

/// Change the first hexadecimal digit of a digest, and nothing else.
fn flip_first_digit(digest: &mut String) {
    let first = if digest.starts_with('0') { "1" } else { "0" };
    digest.replace_range(0..1, first);
}

fn position(register: &Register, name: &str) -> usize {
    register
        .fixture
        .iter()
        .position(|f| f.name == name)
        .expect("the fixture is listed")
}

fn assert_clean(what: &str, violations: &[String]) {
    assert!(violations.is_empty(), "{what}:\n{}", violations.join("\n"));
}

fn assert_rejects(what: &str, violations: &[String], expected: &str) {
    assert!(
        violations.iter().any(|v| v.contains(expected)),
        "{what}: no violation names `{expected}`; got\n{}",
        violations.join("\n")
    );
}

#[test]
fn the_pinned_tree_holds_the_fixtures_the_authority_cites_fx_01() {
    let model = model();
    let register = register();
    let authority = authority(&model);
    assert!(
        authority.citation.contains(PIN_COMMIT),
        "the citation names the commit"
    );
    assert!(
        authority.citation.contains(PIN_TREE),
        "the citation names the tree"
    );
    let rev =
        std::fs::read_to_string(root().join(".github/lexlean.rev")).expect("lexlean.rev reads");
    assert!(
        rev.lines()
            .any(|l| l.trim() == format!("rev = {PIN_COMMIT}")),
        ".github/lexlean.rev pins another commit than the register"
    );
    assert_eq!(register.commit, PIN_COMMIT);
    assert_eq!(
        root_tree(&register).expect("the rows make a tree"),
        PIN_TREE,
        "the fixture trees of the rows are not the pinned tests/negative"
    );
    assert!(
        statement(&model, "FX-01").contains(&commands_sentence(&register)),
        "FX-01 states other counts than the register has: {}",
        commands_sentence(&register)
    );
}

#[test]
fn the_expected_results_are_command_results_of_the_cli_fx_02() {
    let model = model();
    let vendored = vendored();
    for f in &register().fixture {
        let bytes = vendored
            .files
            .get(&format!("{}/expected/command.json", f.name))
            .unwrap_or_else(|| panic!("{}: command.json is vendored", f.name));
        let result: serde_json::Value = serde_json::from_slice(bytes).expect("command.json parses");
        assert_eq!(result["spec"], "lexlean/command-result/1", "{}", f.name);
        assert_eq!(
            result["success"],
            f.exit == 0,
            "{}: success and exit disagree",
            f.name
        );
        for d in result["diagnostics"].as_array().expect("diagnostics") {
            assert_eq!(d["spec"], "lexlean/diagnostic/1", "{}", f.name);
            assert!(
                d["code"].as_str().is_some_and(|c| c.starts_with("LL")),
                "{}",
                f.name
            );
        }
    }
    let claim = model
        .ledger
        .claim
        .iter()
        .find(|c| c.id == "AUTH-08")
        .expect("AUTH-08 states the reproduced fact");
    assert_eq!(claim.authority.as_deref(), Some("A-LEXLEAN-NEGATIVE"));
}

#[test]
fn the_vendored_files_are_the_pinned_tree_fx_03() {
    let model = model();
    let vendored = vendored();
    assert_clean("the vendored files", &check_pin(&register(), &vendored));
    assert_eq!(
        vendored.manifest_sha256(),
        authority(&model).checksum,
        "the authority records another checksum than the vendored files have"
    );
}

#[test]
fn every_row_is_what_the_vendored_bytes_say_fx_04() {
    assert_clean("the rows", &check_derived(&register(), &vendored()));
}

#[test]
fn every_fixture_is_classified_exactly_once_fx_05() {
    let register = register();
    assert_clean("the classes", &check_classes(&register));
    let mut names: Vec<&str> = register.fixture.iter().map(|f| f.name.as_str()).collect();
    let listed = names.len();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), listed, "a name is listed twice");
    assert_eq!(
        names,
        vendored()
            .names()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "the register and the vendored directories name different fixtures"
    );
    let counts = register.counts();
    assert_eq!(
        counts.f4 + counts.f7 + counts.verify + counts.excluded,
        counts.total,
        "a fixture has none of the four classes"
    );
}

#[test]
fn the_counts_are_recomputed_from_the_register_fx_06() {
    let register = register();
    let counts = register.counts();
    assert_eq!(counts.in_scope(), counts.total - counts.excluded);
    assert_eq!(
        counts.f4,
        register
            .fixture
            .iter()
            .filter(|f| f.command == "check" && f.class == F4)
            .count()
    );
    assert_eq!(counts.f7, by_command(&register, "build"));
    assert_eq!(counts.verify, by_command(&register, "verify"));
    assert!(
        statement(&model(), "FX-06").contains(&counts_sentence(&counts)),
        "FX-06 states other counts than the register has: {}",
        counts_sentence(&counts)
    );
}

#[test]
fn the_exclusions_are_the_twelve_of_the_issue_fx_07() {
    let register = register();
    assert_clean("the exclusions", &check_exclusions(&register));
    assert_eq!(register.counts().excluded, ISSUE_EXCLUSIONS.len());
    assert!(
        statement(&model(), "FX-07").contains(&reasons_sentence(&register)),
        "FX-07 states other reasons than the register has: {}",
        reasons_sentence(&register)
    );
}

#[test]
fn every_gate_rejects_its_planted_defect_fx_08() {
    let register = register();
    let vendored = vendored();
    let model = model();

    // The control: nothing is planted and every gate is clean.
    assert_clean("pin", &check_pin(&register, &vendored));
    assert_clean("derived", &check_derived(&register, &vendored));
    assert_clean("classes", &check_classes(&register));
    assert_clean("exclusions", &check_exclusions(&register));

    // A fixture reclassified.
    let mut plant = register.clone();
    let i = position(&plant, "application-arity");
    plant.fixture[i].class = VERIFY.to_owned();
    assert_rejects(
        "reclassified",
        &check_classes(&plant),
        "application-arity: class is #11",
    );
    let mut plant = register.clone();
    let i = position(&plant, "stale-lock");
    plant.fixture[i].class = F4.to_owned();
    plant.fixture[i].reason.clear();
    assert_rejects(
        "an exclusion dropped",
        &check_classes(&plant),
        "stale-lock: class is F4",
    );
    assert_rejects(
        "an exclusion dropped",
        &check_exclusions(&plant),
        "stale-lock: the issue excludes it",
    );
    let mut plant = register.clone();
    let i = position(&plant, "lexicon-raw-unicode-render");
    plant.fixture[i].class = F7.to_owned();
    plant.fixture[i].command = "check".to_owned();
    assert_rejects(
        "a command rewritten",
        &check_derived(&plant, &vendored),
        "lexicon-raw-unicode-render",
    );

    // A digest altered.
    let mut plant = register.clone();
    let i = position(&plant, "application-arity");
    flip_first_digit(&mut plant.fixture[i].result_sha256);
    assert_rejects(
        "a result digest",
        &check_pin(&plant, &vendored),
        "application-arity: expected/command.json has SHA-256",
    );
    assert_rejects(
        "a result digest",
        &check_derived(&plant, &vendored),
        "application-arity",
    );
    let mut plant = register.clone();
    flip_first_digit(&mut plant.fixture[i].tree);
    assert_rejects(
        "a fixture tree",
        &check_pin(&plant, &vendored),
        "application-arity: the vendored files make the tree",
    );
    assert_rejects(
        "a fixture tree",
        &check_pin(&plant, &vendored),
        "for tests/negative",
    );
    let mut plant = register.clone();
    let i = position(&plant, "application-arity");
    plant.fixture[i].exit += 1;
    assert_rejects(
        "an exit status",
        &check_derived(&plant, &vendored),
        "application-arity",
    );

    // A vendored byte altered.
    let mut changed = vendored.clone();
    let bytes = changed
        .files
        .get_mut("application-arity/expected/command.json")
        .expect("the file is vendored");
    bytes.push(b'\n');
    assert_rejects(
        "a vendored byte",
        &check_pin(&register, &changed),
        "application-arity: expected/command.json has SHA-256",
    );
    assert_ne!(
        changed.manifest_sha256(),
        authority(&model).checksum,
        "the authority checksum covers every vendored byte"
    );
    let mut missing = vendored.clone();
    missing
        .files
        .remove("application-arity/expected/hashes.toml");
    assert_rejects(
        "a vendored file removed",
        &check_pin(&register, &missing),
        "application-arity: the vendored files make the tree",
    );

    // An exclusion without a reason, and one the issue does not make.
    let mut plant = register.clone();
    let i = position(&plant, "stale-lock");
    plant.fixture[i].reason.clear();
    assert_rejects(
        "no reason",
        &check_classes(&plant),
        "stale-lock: excluded without a reason",
    );
    assert_rejects(
        "no reason",
        &check_exclusions(&plant),
        "stale-lock: the issue's reason is `setup`",
    );
    let mut plant = register.clone();
    let i = position(&plant, "application-arity");
    plant.fixture[i].class = EXCLUDED.to_owned();
    plant.fixture[i].reason = "not reached".to_owned();
    assert_rejects(
        "an extra exclusion",
        &check_exclusions(&plant),
        "application-arity: excluded, and the issue does not exclude it",
    );
    assert_rejects(
        "an extra exclusion",
        &check_classes(&plant),
        "application-arity: class is excluded",
    );

    // A name listed twice, and a fixture missing.
    let mut plant = register.clone();
    plant.fixture.push(plant.fixture[0].clone());
    assert_rejects("a duplicate", &check_classes(&plant), "listed twice");
    let mut plant = register.clone();
    plant.fixture.remove(0);
    assert_rejects(
        "a missing fixture",
        &check_pin(&plant, &vendored),
        "fixtures and",
    );
    assert_ne!(
        counts_sentence(&plant.counts()),
        counts_sentence(&register.counts()),
        "the counts follow the register"
    );
    assert!(
        !statement(&model, "FX-06").contains(&counts_sentence(&plant.counts())),
        "a register with a fixture fewer must contradict FX-06"
    );
}
