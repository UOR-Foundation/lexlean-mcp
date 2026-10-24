//! ST-01 to ST-09: the store, persistence and lifecycle model of issue #13.
//!
//! The model is LexLean source (`src/Store.lex.tex`) and `lexlean verify` checks
//! the kernel side. These tests read the same source as text and hold it against
//! what the issue states (the seven directories, the backup list, the order of the
//! drift checks) and against `STORE.md`, so that a law dropped from the model, a
//! name changed in one place, or a planted defect left unrecorded fails here and
//! names the law.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
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

fn store() -> String {
    tight(&read("src/Store.lex.tex"))
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
        .unwrap_or_else(|| panic!("src/Store.lex.tex has no {kind} {name}"));
    let chunk = &source[at + kind.len() + 1..];
    chunk[..closing_brace(chunk)].to_string()
}

/// The field `field: ...` of a declaration, up to the next top-level `, <word>:`.
fn after<'a>(text: &'a str, marker: &str) -> &'a str {
    let at = text
        .find(marker)
        .unwrap_or_else(|| panic!("`{marker}` is not in `{text}`"));
    &text[at + marker.len()..]
}

/// The constructors, in order, of `inductive{name: <name>, ...}`.
fn constructors(source: &str, name: &str) -> Vec<String> {
    let inductive = declaration(source, "inductive", name);
    let list = after(&inductive, "constructors: [");
    let mut found = Vec::new();
    let mut rest = list;
    while let Some(at) = rest.find("{name: ") {
        let tail = &rest[at + "{name: ".len()..];
        let end = tail.find(',').expect("a constructor has fields");
        found.push(tail[..end].to_string());
        rest = &tail[end..];
    }
    found
}

/// The field names, in order, of `structure{name: <name>, ...}`.
fn fields(source: &str, name: &str) -> Vec<String> {
    let structure = declaration(source, "structure", name);
    let list = after(&structure, "fields: [");
    let mut found = Vec::new();
    let mut rest = list;
    while let Some(at) = rest.find("{name: ") {
        let tail = &rest[at + "{name: ".len()..];
        let end = tail.find(',').expect("a field has a type");
        found.push(tail[..end].to_string());
        rest = &tail[end..];
    }
    found
}

/// The arms `| <prefix>X => <value>` of a definition's body, as (X, value).
fn arms(source: &str, definition: &str, prefix: &str) -> Vec<(String, String)> {
    let body = declaration(source, "definition", definition);
    let body = after(&body, "body: ");
    let marker = format!("| {prefix}.");
    let mut found = Vec::new();
    let mut rest = body;
    while let Some(at) = rest.find(&marker) {
        let tail = &rest[at + marker.len()..];
        let (constructor, tail) = tail.split_once(" => ").expect("an arm has a value");
        let end = tail.find(" | ").unwrap_or(tail.len());
        found.push((
            constructor.to_string(),
            tail[..end].trim_end_matches(')').to_string(),
        ));
        rest = &tail[end..];
    }
    found
}

fn theorems_exist(source: &str, names: &[&str]) {
    for name in names {
        let theorem = declaration(source, "theorem", name);
        assert!(
            theorem.contains("proof: "),
            "theorem {name} has no proof in src/Store.lex.tex"
        );
        assert!(
            !theorem.contains("sorry"),
            "theorem {name} is not proved: it mentions sorry"
        );
    }
}

/// The list of `(ctor Verdict.X)` of a statement, in order.
fn verdicts(statement: &str) -> Vec<String> {
    statement
        .split("(ctor Verdict.")
        .skip(1)
        .map(|tail| tail[..tail.find(')').expect("a constructor closes")].to_string())
        .collect()
}

/// ST-01: the layout is the issue's seven directories.
#[test]
fn the_layout_is_the_seven_directories_of_the_issue_st_01() {
    let source = store();
    let issue = [
        "profiles",
        "drafts",
        "builds",
        "verifications",
        "index",
        "exports",
        "keys",
    ];
    let order = constructors(&source, "Dir");
    assert_eq!(order.len(), 7, "the layout has seven directories");
    let names = arms(&source, "dirName", "Dir");
    assert_eq!(
        names.iter().map(|(c, _)| c.clone()).collect::<Vec<_>>(),
        order,
        "dirName names every directory once, in the order of the type"
    );
    let wire: Vec<String> = names
        .iter()
        .map(|(_, value)| value.trim_matches('"').to_string())
        .collect();
    assert_eq!(
        wire, issue,
        "the wire names are those of issue #13, in its order"
    );
    assert_eq!(
        wire.iter().collect::<BTreeSet<_>>().len(),
        7,
        "the wire names are distinct"
    );
    assert!(
        declaration(&source, "definition", "storeRoot").contains("\"lexlean-mcp/store/1\""),
        "the store root is lexlean-mcp/store/1"
    );
    theorems_exist(
        &source,
        &[
            "sevenDirectories",
            "wireNamesAreTheIssuesSeven",
            "wireNamesAreDistinct",
            "everyDirectoryIsListed",
            "everyWireNameParsesBack",
            "anUnknownNameIsRefused",
        ],
    );
}

/// ST-02: the backup set is the issue's list and nothing else.
#[test]
fn the_backup_set_is_the_issues_list_st_02() {
    let source = store();
    let backed: Vec<String> = arms(&source, "inBackup", "Item")
        .into_iter()
        .filter(|(_, value)| value == "true")
        .map(|(constructor, _)| constructor)
        .collect();
    assert_eq!(
        backed,
        ["Verifications", "Keys", "DraftReferences"],
        "issue #13 backs up verifications/, keys/ and the drafts records reference"
    );
    assert_eq!(
        fields(&source, "Archive"),
        ["verifications", "keys", "draftReferences"],
        "the archive has exactly the three backed-up fields"
    );
    assert_eq!(
        constructors(&source, "Item").len(),
        8,
        "the seven directories and the drafts references"
    );
    theorems_exist(
        &source,
        &[
            "backupSetHasThreeItems",
            "backupPredicateIsMembershipOfTheSet",
            "backupCoversWhatNothingElseRecovers",
            "backupHoldsNothingRecomputable",
            "theArchiveHoldsExactlyTheBackupSet",
        ],
    );
}

/// ST-03: retention follows the bound owner values.
#[test]
fn retention_follows_the_bound_owner_values_st_03() {
    let owner = tight(&read("src/OwnerValues.lex.tex"));
    assert!(
        declaration(&owner, "definition", "storeDraftRetention")
            .contains("(ctor Retention.Days 30)"),
        "store.draft_retention is 30 days in src/OwnerValues.lex.tex"
    );
    assert!(
        declaration(&owner, "definition", "storeAttestationRetention")
            .contains("(ctor Retention.Forever)"),
        "store.attestation_retention is forever in src/OwnerValues.lex.tex"
    );
    let source = store();
    let of = arms(&source, "retentionOf", "Kind");
    assert_eq!(
        of,
        [
            (
                "Draft".to_string(),
                "(OwnerValues::storeDraftRetention".to_string()
            ),
            (
                "Attestation".to_string(),
                "(OwnerValues::storeAttestationRetention".to_string()
            ),
        ],
        "each kind reads its retention from the owner values"
    );
    let expired = arms(&source, "expired", "OwnerValues::Retention");
    assert!(
        expired.iter().any(|(c, v)| c == "Forever" && v == "false"),
        "a Forever retention never expires"
    );
    theorems_exist(
        &source,
        &[
            "attestationsNeverExpire",
            "draftsExpireByTheBoundRetention",
            "aDraftIsKeptAtThirtyDays",
            "aDraftExpiresAtThirtyOneDays",
            "anAttestationIsKeptAfterTenYears",
        ],
    );
}

/// The verdict the issue's order gives: the lock check, then the profile ID, then the
/// toolchain hashes.
fn expected(lock_drifted: bool, id_matched: bool, hashes_matched: bool) -> &'static str {
    if lock_drifted {
        "LockDrift"
    } else if !id_matched {
        "IdDrift"
    } else if !hashes_matched {
        "ToolchainDrift"
    } else {
        "NoDrift"
    }
}

/// ST-04: the decision order and its table.
#[test]
fn the_drift_check_decides_in_the_issues_order_st_04() {
    let source = store();
    assert_eq!(
        constructors(&source, "Verdict"),
        ["NoDrift", "LockDrift", "IdDrift", "ToolchainDrift"]
    );
    let stage = declaration(&source, "definition", "driftStage");
    let id = stage.find("if idMatched").expect("the ID is tested");
    let hashes = stage
        .find("if hashesMatched")
        .expect("the hashes are tested");
    assert!(
        id < hashes,
        "the profile ID is tested before the toolchain hashes"
    );
    assert!(
        stage
            .find("LockResult.Drifted")
            .expect("the lock is tested")
            < id,
        "the lock check is decided before the profile ID"
    );
    // The table: Clean then Drifted, ID matched then not, hashes matched then not.
    let mut table = Vec::new();
    for lock_drifted in [false, true] {
        for id_matched in [true, false] {
            for hashes_matched in [true, false] {
                table.push(expected(lock_drifted, id_matched, hashes_matched).to_string());
            }
        }
    }
    let theorem = declaration(&source, "theorem", "theDecisionTableIsExact");
    let statement = after(&theorem, "statement: ");
    let stated = verdicts(&statement[statement.find(" = ").expect("an equation")..]);
    assert_eq!(
        stated, table,
        "the table is the one the issue's order gives"
    );
    theorems_exist(
        &source,
        &[
            "lockDriftIsReportedFirst",
            "idDriftIsReportedBeforeToolchainDrift",
            "toolchainDriftNeedsAMatchingId",
            "noDriftNeedsAllThree",
            "aDuplicatePathIsIdDrift",
            "aDuplicateReviewIsIdDrift",
            "aFieldTooLongIsIdDrift",
            "anUnchangedProfileHasNoDrift",
            "aChangedLockIsLockDrift",
            "aChangedIdIsIdDriftEvenWhenTheToolchainChanged",
            "aChangedToolchainHashIsToolchainDrift",
        ],
    );
}

/// ST-05: no lifecycle event but a write changes the records.
#[test]
fn the_lifecycle_never_touches_a_record_st_05() {
    let source = store();
    let events = constructors(&source, "Event");
    assert_eq!(
        events,
        ["Rollout", "Rollback", "Upgrade", "Retire", "Write"],
        "rollout, rollback, upgrade and retirement, and the write of a verification"
    );
    for (event, value) in arms(&source, "step", "Event") {
        if event == "Write" {
            assert!(
                value.contains("records := (put (project m records) v)"),
                "a write puts the record"
            );
        } else if event != "Rollback" {
            assert!(
                value.contains("records := (project m records)"),
                "{event} leaves the records as they are"
            );
        }
    }
    theorems_exist(
        &source,
        &[
            "aRolloutKeepsEveryRecord",
            "aRollbackKeepsEveryRecord",
            "anUpgradeKeepsEveryRecord",
            "aRetirementKeepsEveryRecord",
            "rollbackRestoresTheRevision",
            "rollbackRestoresTheServedSet",
            "anUpgradeServesTheNewProfile",
            "aRetiredProfileIsNotServed",
        ],
    );
}

/// ST-06: AC-14 is a claim about preimages, and it is stated over Identity's round trip.
#[test]
fn a_new_revision_changes_the_preimage_and_keeps_the_records_st_06() {
    let identity = tight(&read("src/Identity.lex.tex"));
    declaration(&identity, "theorem", "profileRT");
    let source = store();
    let recovery = declaration(&source, "theorem", "aPreimageDeterminesItsToolchain");
    assert!(
        recovery.contains("Identity::profileRT"),
        "the recovery theorem rests on Identity's round trip of the profile preimage"
    );
    assert!(
        !source.contains("collision"),
        "the model claims nothing about SHA-256 collisions"
    );
    theorems_exist(
        &source,
        &[
            "toolchainOfFoundGate",
            "toolchainOfFoundSome",
            "aPreimageDeterminesItsToolchain",
            "anotherRevisionHasAnotherProfilePreimage",
            "theEarlierRecordResolvesAfterTheUpgrade",
            "theEarlierRecordStillValidatesAfterTheUpgrade",
            "theUpgradeServesTheNewProfileOnly",
            "aWriteNeverInvalidatesAStore",
        ],
    );
}

/// ST-07: AC-15 and AC-17.
#[test]
fn a_rollback_serves_the_previous_revision_and_loses_nothing_st_07() {
    theorems_exist(
        &store(),
        &[
            "rollbackServesThePreviousProfiles",
            "rollbackServesThePreviousRevision",
            "theLaterRecordResolvesAfterTheRollback",
            "rollbackKeepsARecordWrittenAfterTheRollout",
            "aRollbackResolvesWhatItResolved",
            "aReleaseRollbackLeavesBothRecordsResolvable",
            "aWrittenVerificationResolves",
        ],
    );
}

/// ST-08: AC-16.
#[test]
fn a_restore_resolves_every_backed_up_record_st_08() {
    let source = store();
    assert!(
        declaration(&source, "definition", "restore").contains("keys := (project a keys)"),
        "restore returns the keys"
    );
    theorems_exist(
        &source,
        &[
            "restoreReturnsTheVerifications",
            "restoreReturnsTheKeys",
            "restoreReturnsTheDraftReferences",
            "restoreLeavesTheRecomputedDirectoriesToBeRebuilt",
            "everyRestoredRecordResolvesAsBefore",
            "theRestoredStoreResolvesEveryBackedUpRecord",
            "theRestoredStoreKeepsTheKeys",
            "reverifyingARestoredStoreGivesTheSameIds",
            "makeVerificationHolds",
            "reverifyGivesTheStoredId",
            "anotherToolchainGivesAnotherAttestationId",
        ],
    );
    let doc = tight(&read("STORE.md"));
    assert!(
        doc.contains("abstract until #11"),
        "STORE.md states that the attestation signing stays abstract until #11"
    );
}

/// ST-09: each planted defect is recorded with its command, diagnostic and restore.
#[test]
fn every_law_has_a_recorded_planted_defect_st_09() {
    let doc = read("STORE.md");
    let rows: Vec<Vec<String>> = doc
        .lines()
        .filter(|line| line.starts_with("| P"))
        .map(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect()
        })
        .collect();
    assert_eq!(rows.len(), 4, "four planted defects are recorded");
    let source = store();
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
