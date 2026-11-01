//! VO-01 to VO-16: the gateway side of issue #8, profiles, vocabulary and provisioning.
//!
//! The behavior is tested where the host and the core run: `gateway/bindings/gateway-binding`
//! tests the identity and the operator's commands (`tests/provision.rs`), and
//! `gateway/tool-tests` calls the tools, the resource template and completion through the
//! production core with the real host (`tests/vocabulary.rs`). These tests read the sources as
//! text and hold them against each other: the core's language rule against the model's, the
//! tools against the closed effect set, the capabilities against what is served, the tests
//! against the rows they answer, and the planted defects against `PROFILES.md`. Nothing here
//! runs LexLean.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn tight(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| {
                ["target", ".prism", ".lexlean", "registry"]
                    .iter()
                    .any(|skipped| name == *skipped)
            }) {
                continue;
            }
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// The names of the `#[test]` functions of the Rust files beneath `relative`.
fn test_names(relative: &str) -> BTreeSet<String> {
    let mut files = Vec::new();
    walk(&root().join(relative), &mut files);
    let mut names = BTreeSet::new();
    for file in files
        .iter()
        .filter(|file| file.extension().is_some_and(|e| e == "rs"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let mut in_test = false;
        for line in text.lines() {
            let line = line.trim();
            if line == "#[test]" {
                in_test = true;
            } else if in_test && line.starts_with("fn ") {
                names.insert(line[3..].split('(').next().expect("a name").to_string());
                in_test = false;
            } else if in_test && !line.starts_with("#[") && !line.starts_with("///") {
                in_test = false;
            }
        }
    }
    names
}

fn gateway_tests() -> BTreeSet<String> {
    let mut names = test_names("gateway/tool-tests");
    names.extend(test_names("gateway/bindings/gateway-binding"));
    names
}

/// The tests of the gateway that answer the row `id`: their names end in it.
fn answering(id: &str) -> BTreeSet<String> {
    let suffix = id.to_lowercase().replace('-', "_");
    gateway_tests()
        .into_iter()
        .filter(|name| name.ends_with(&suffix))
        .collect()
}

/// Every test named exists among the tests of the row `id`.
fn needs(id: &str, tests: &[&str]) {
    let names = answering(id);
    for test in tests {
        assert!(
            names.contains(*test),
            "a test of gateway/ has the name {test} for {id}: {names:?}"
        );
    }
}

/// Every test named exists among the tests of the gateway, whatever row it carries.
fn needs_named(tests: &[&str]) {
    let names = gateway_tests();
    for test in tests {
        assert!(
            names.contains(*test),
            "a test of gateway/ has the name {test}"
        );
    }
}

/// The blocks of a data-form module, by name: each block runs from a line that opens one to the
/// line before the next, without the blank lines between.
fn blocks(text: &str) -> Vec<(String, String)> {
    let openers = ["inductive{", "structure{", "definition{", "theorem{"];
    let mut found: Vec<(String, String)> = Vec::new();
    let mut current: Option<String> = None;
    for line in text.lines() {
        let opens = openers.iter().any(|opener| line.starts_with(opener));
        let ends = line.starts_with("\\end{semanticmodule}");
        if opens || ends {
            if let Some(block) = current.take() {
                let block = block.trim_end().to_string();
                let name = block
                    .split("name: ")
                    .nth(1)
                    .and_then(|rest| rest.split(',').next())
                    .expect("a block has a name")
                    .to_string();
                found.push((name, block));
            }
        }
        if opens {
            current = Some(format!("{line}\n"));
        } else if let Some(block) = current.as_mut() {
            block.push_str(line);
            block.push('\n');
        }
    }
    found
}

fn block_named(text: &str, name: &str) -> String {
    blocks(text)
        .into_iter()
        .find(|(found, _)| found == name)
        .unwrap_or_else(|| panic!("a block named {name}"))
        .1
}

// ---- VO-01 -----------------------------------------------------------------------------------

/// VO-01: the host's identity is held to the model's vectors, and Python recomputes it.
#[test]
fn the_host_holds_the_models_profile_vectors_vo_01() {
    let model = tight(&read("src/Identity.lex.tex"));
    for name in [
        "profilePreimageVector",
        "profileIdVector",
        "profileDigestVector",
        "profileFramingReviewsVector",
        "changedFileProfileId",
        "encProfile",
    ] {
        assert!(model.contains(name), "the model has {name}");
    }
    let host = read("gateway/bindings/gateway-binding/src/identity.rs");
    for needle in [
        "b\"lexlean-mcp-profile-v1\"",
        "\"domain\"",
        "\"files\"",
        "\"file\"",
        "\"reviews\"",
        "\"review\"",
        "\"toolchain\"",
        "Sha256",
    ] {
        assert!(host.contains(needle), "the host's encoder has {needle}");
    }
    let python = read("gateway/vectors/generate.py");
    assert!(
        python.contains("def profile_id(") && python.contains("\"profile-id\""),
        "generate.py recomputes the profile ID and has the command profile-id"
    );
    needs(
        "VO-01",
        &[
            "the_host_reproduces_the_models_profile_vectors_vo_01",
            "the_host_reproduces_the_models_review_framing_vo_01",
            "the_toolchain_of_the_id_is_the_compilers_vo_01",
            "the_recomputation_in_python_agrees_with_the_host_vo_01",
        ],
    );
}

// ---- VO-02 -----------------------------------------------------------------------------------

/// VO-02: every profile file is in the identity, and a source file is not.
#[test]
fn every_profile_file_is_in_the_identity_vo_02() {
    let host = read("gateway/bindings/gateway-binding/src/provision.rs");
    for needle in [
        "lexicon_sources",
        "artifact_sources",
        "config.lockfile",
        "under_root",
    ] {
        assert!(
            host.contains(needle),
            "the host gathers the profile files with {needle}: a file kind left out of the identity fails here"
        );
    }
    needs(
        "VO-02",
        &[
            "a_byte_of_each_profile_file_changes_the_id_vo_02",
            "a_renamed_an_added_and_a_removed_file_change_the_id_vo_02",
            "a_review_record_changes_the_id_vo_02",
            "a_source_file_leaves_the_id_alone_vo_02",
            "install_records_the_id_the_host_computes_vo_02",
        ],
    );
}

// ---- VO-03 -----------------------------------------------------------------------------------

const RULE: [&str; 5] = [
    "PackageCheck",
    "PackageDecl",
    "languageMismatchCode",
    "checkPackage",
    "checkPackages",
];

/// VO-03: the language rule of the core is the model's, declaration by declaration.
#[test]
fn the_language_rule_of_the_core_is_the_models_vo_03() {
    let model = read("src/Profiles.lex.tex");
    let copy = read("gateway/src/ProfileLanguage.lex.tex");
    for name in RULE {
        let theirs = block_named(&model, name);
        let ours = block_named(&copy, name);
        assert_eq!(
            theirs, ours,
            "{name} of gateway/src/ProfileLanguage.lex.tex is the declaration of src/Profiles.lex.tex"
        );
    }
    let declared: Vec<String> = blocks(&copy).into_iter().map(|(name, _)| name).collect();
    assert_eq!(
        declared,
        RULE.map(str::to_string).to_vec(),
        "the copy holds these declarations and no other"
    );
    for name in ["languageMismatchCode", "checkPackage", "checkPackages"] {
        assert!(
            block_named(&model, name).contains("executable: true"),
            "the model carries {name} as executable"
        );
    }
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    assert!(
        vocabulary.contains("ProfileLanguage::checkPackages")
            && vocabulary.contains("ProfileLanguage::PackageCheck.Refused"),
        "the tools apply the model's rule"
    );
    let language = read("gateway/src/ProfileLanguage.lex.tex");
    assert!(
        language.starts_with("\\begin{lexlean}{ProfileLanguage}\n"),
        "the module is named for what it holds"
    );
}

// ---- VO-04 -----------------------------------------------------------------------------------

/// VO-04: a package of another language is refused wherever it could enter.
#[test]
fn a_package_of_another_language_is_refused_everywhere_vo_04() {
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    let uses = vocabulary.matches("ProfileLanguage::checkPackages").count();
    assert!(
        uses >= 3,
        "the catalogue, the search and the read each apply the rule: {uses} uses"
    );
    needs(
        "VO-04",
        &[
            "a_package_of_another_language_is_refused_at_install_with_llc0103_vo_04",
            "a_catalogue_with_a_package_of_another_language_lists_the_profile_under_refused_vo_04",
            "search_refuses_a_vocabulary_of_mixed_languages_with_llc0103_vo_04",
        ],
    );
}

// ---- VO-05 -----------------------------------------------------------------------------------

/// VO-05: provisioning is the binding's command and no effect.
#[test]
fn provisioning_is_a_command_and_not_an_effect_vo_05() {
    let effects = read("src/Effects.lex.tex");
    let names: Vec<&str> = effects
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("\"")
                .and_then(|rest| rest.split('"').next())
        })
        .filter(|name| name.contains('.'))
        .collect();
    for forbidden in ["lock", "init", "install", "provision"] {
        assert!(
            !names.iter().any(|name| name.contains(forbidden)),
            "{forbidden} is no effect: {names:?}"
        );
    }
    let manifest = read("gateway/bindings/gateway-binding/Cargo.toml");
    assert!(
        manifest.contains("[[bin]]") && manifest.contains("name = \"gateway-provision\""),
        "the binding has the operator's binary"
    );
    let tools = tight(&read("gateway/src/GatewayTools.lex.tex"));
    for forbidden in [
        "lexlean_lock",
        "lexlean_init",
        "lexlean_install",
        "lexlean_provision",
    ] {
        assert!(!tools.contains(forbidden), "no tool is {forbidden}");
    }
    let main = read("gateway/bindings/gateway-binding/src/bin/gateway-provision.rs");
    assert!(
        main.contains("allow-network") || main.contains("allow_network"),
        "the binary has the flag"
    );
    needs(
        "VO-05",
        &[
            "init_makes_a_project_that_locks_and_installs_vo_05",
            "a_stale_lock_is_llc0102_and_install_does_not_write_it_vo_05",
            "a_missing_lock_is_refused_and_not_made_vo_05",
            "allow_network_is_accepted_by_lock_alone_vo_05",
            "the_binary_provisions_a_profile_vo_05",
        ],
    );
}

// ---- VO-06 -----------------------------------------------------------------------------------

/// VO-06: the review records, and the measurement of risk R2 that `Security` executes.
#[test]
fn the_review_records_are_required_and_the_measurement_is_executed_vo_06() {
    let security = read("src/Security.lex.tex");
    let measurements = block_named(&security, "measurements");
    let row = measurements
        .split("(record Measurement {risk := 1,")
        .nth(1)
        .expect("the measurement of R2")
        .split("(record Measurement {risk := 2,")
        .next()
        .expect("up to the next");
    assert!(
        row.contains("ctor MeasureStatus.Executed") && row.contains("VO-06"),
        "the measurement of R2 is Executed and names VO-06: {row}"
    );
    assert!(
        read("SECURITY.md").contains("| R2 |"),
        "SECURITY.md records the executed measurement of R2"
    );
    let host = read("gateway/bindings/gateway-binding/src/provision.rs");
    for needle in [
        "lexlean-mcp-review-v1",
        "ReviewMissing",
        "ReviewMalformed",
        "ReviewStale",
        "ReviewDuplicate",
    ] {
        assert!(host.contains(needle), "install enforces {needle}");
    }
    needs(
        "VO-06",
        &[
            "a_path_package_without_a_review_is_refused_vo_06",
            "a_valid_review_installs_and_is_stored_by_its_digest_vo_06",
            "a_review_of_another_digest_is_stale_vo_06",
            "a_review_that_names_no_locked_package_is_stale_vo_06",
            "a_malformed_review_is_refused_vo_06",
            "a_repeated_review_is_refused_vo_06",
            "a_builtin_package_needs_no_review_vo_06",
        ],
    );
}

// ---- VO-07 -----------------------------------------------------------------------------------

/// VO-07: the layout of a profile, and the three entries that materialization leaves out.
#[test]
fn the_layout_of_a_profile_and_what_materialization_skips_vo_07() {
    let materialize = read("gateway/bindings/gateway-binding/src/materialize.rs");
    let provision = read("gateway/bindings/gateway-binding/src/provision.rs");
    for name in ["\"profile\"", "\"vocabulary\"", "\"reviews\""] {
        assert!(
            materialize.contains(name) && provision.contains(name),
            "materialization and install both name the reserved entry {name}"
        );
    }
    needs(
        "VO-07",
        &[
            "install_lays_out_the_profile_vo_07",
            "a_name_that_is_not_a_key_segment_is_refused_vo_07",
            "a_name_already_installed_is_refused_and_the_store_is_unchanged_vo_07",
            "a_link_in_the_project_is_refused_vo_07",
            "a_reserved_entry_is_refused_vo_07",
            "a_project_with_source_packages_is_refused_vo_07",
            "a_project_that_is_not_a_directory_or_has_no_configuration_is_refused_vo_07",
            "a_refused_install_leaves_the_store_as_it_was_vo_07",
            "materialization_copies_the_bundle_without_the_derived_entries_vo_07",
        ],
    );
}

// ---- VO-08 -----------------------------------------------------------------------------------

/// VO-08: the vocabulary is what the compiler resolved, within the bound of `lexicon.read`.
#[test]
fn the_vocabulary_is_what_the_compiler_resolved_vo_08() {
    let provision = read("gateway/bindings/gateway-binding/src/provision.rs");
    assert!(
        provision.contains("resolve_packages"),
        "the packages are the compiler's own resolution"
    );
    assert!(
        provision.contains("VOCABULARY_LIMIT: usize = ENTRY_MAXIMUM"),
        "the bound is the 1 MiB of lexicon.read"
    );
    let lexicon = read("gateway/bindings/gateway-binding/src/lexicon.rs");
    assert!(
        lexicon.contains("ENTRY_MAXIMUM: usize = 1 << 20"),
        "lexicon.read returns at most 1 MiB and 64 octets"
    );
    needs(
        "VO-08",
        &[
            "the_vocabulary_lists_every_locked_package_and_the_served_entries_vo_08",
            "a_vocabulary_over_the_bound_is_refused_vo_08",
        ],
    );
}

// ---- VO-09, VO-10 ----------------------------------------------------------------------------

/// VO-09: `lexlean_profiles` asks one store.get and has a closed schema.
#[test]
fn lexlean_profiles_is_one_store_get_vo_09() {
    let tools = tight(&read("gateway/src/GatewayTools.lex.tex"));
    assert!(
        tools.contains("effectTool \"lexlean_profiles\""),
        "the tool is in the catalogue"
    );
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    for needle in [
        "catalogueArea",
        "cataloguePayload",
        "CatalogueMalformed",
        "ProfileUnknown",
    ] {
        assert!(vocabulary.contains(needle), "the core has {needle}");
    }
    needs(
        "VO-09",
        &[
            "lexlean_profiles_lists_the_installed_profiles_vo_09",
            "a_named_profile_is_listed_alone_and_an_unknown_one_is_refused_vo_09",
            "an_empty_store_has_no_profiles_vo_09",
            "a_catalogue_that_is_not_the_layout_is_refused_vo_09",
            "the_schema_of_lexlean_profiles_is_closed_vo_09",
        ],
    );
}

/// VO-10: `lexlean_lexicon_search` asks one lexicon.read and pages.
#[test]
fn lexlean_lexicon_search_is_one_lexicon_read_vo_10() {
    let tools = tight(&read("gateway/src/GatewayTools.lex.tex"));
    assert!(
        tools.contains("effectTool \"lexlean_lexicon_search\""),
        "the tool is in the catalogue"
    );
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    for needle in ["VocabularyMissing", "VocabularyMalformed", "searchIn"] {
        assert!(vocabulary.contains(needle), "the core has {needle}");
    }
    needs(
        "VO-10",
        &[
            "search_pages_through_the_vocabulary_in_order_vo_10",
            "search_matches_ids_and_spellings_without_regard_to_case_vo_10",
            "search_filters_by_package_vo_10",
            "search_refusals_are_typed_vo_10",
            "the_schema_of_lexlean_lexicon_search_is_closed_vo_10",
        ],
    );
}

// ---- VO-11 -----------------------------------------------------------------------------------

/// VO-11: the template, its scope and its lifetime.
#[test]
fn the_template_is_public_with_the_owner_ttl_vo_11() {
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    assert!(
        vocabulary.contains("lexlean://profiles/{profile}/lexicon/{package}@{version}"),
        "the template is the issue's"
    );
    let gateway = tight(&read("gateway/src/Gateway.lex.tex"));
    assert!(
        gateway.contains("name: resourceCache")
            && gateway.contains("cacheScope := (ctor Common::CacheScope.public)"),
        "the scope of the lexicon is public"
    );
    let owner = read("OWNER-VALUES.md");
    let ttl = owner
        .lines()
        .find(|line| line.starts_with("| `cache.ttl_ms` |"))
        .and_then(|line| line.split('|').nth(2))
        .map(|cell| cell.trim().trim_matches('`').to_string())
        .expect("the owner value cache.ttl_ms");
    assert!(
        gateway.contains(&format!(
            "ttlMs := {ttl}, cacheScope := (ctor Common::CacheScope.public)"
        )),
        "the lifetime of the lexicon is the owner's cache.ttl_ms {ttl}"
    );
    needs(
        "VO-11",
        &[
            "the_template_is_listed_public_with_the_owner_ttl_vo_11",
            "resources_list_one_resource_per_package_of_every_profile_vo_11",
            "a_resource_is_read_as_the_entries_of_its_package_vo_11",
            "a_resource_that_is_not_found_is_32602_with_its_uri_vo_11",
        ],
    );
}

// ---- VO-12 -----------------------------------------------------------------------------------

/// VO-12: completion of the arguments of the template.
#[test]
fn completion_covers_profile_and_package_vo_12() {
    let vocabulary = tight(&read("gateway/src/Vocabulary.lex.tex"));
    assert!(
        vocabulary.contains("Resources::matching")
            && vocabulary.contains("Resources::answerComplete")
            && vocabulary.contains("Resources::argumentOf"),
        "completion is MCP's own"
    );
    needs(
        "VO-12",
        &[
            "the_profile_argument_completes_by_prefix_vo_12",
            "the_package_argument_completes_within_the_context_profile_vo_12",
            "completion_is_bounded_at_100_values_vo_12",
            "another_reference_or_argument_is_32602_vo_12",
        ],
    );
}

// ---- VO-13 -----------------------------------------------------------------------------------

/// VO-13: the gateway declares tools, resources and completions, and not prompts.
#[test]
fn the_declared_capabilities_are_the_served_ones_vo_13() {
    let gateway = tight(&read("gateway/src/Gateway.lex.tex"));
    let capabilities = gateway
        .split("name: gatewayCapabilities")
        .nth(1)
        .and_then(|rest| rest.split("name: gatewayCache").next())
        .expect("gatewayCapabilities");
    assert!(
        capabilities.contains("completions := (ctor Option.some [Json::JsonObject]")
            && capabilities
                .contains("resources := (ctor Option.some [Capabilities::ResourcesCapability]")
            && capabilities
                .contains("tools := (ctor Option.some [Capabilities::ListChangedCapability]"),
        "tools, resources and completions are declared: {capabilities}"
    );
    assert!(
        capabilities
            .contains("prompts := (ctor Option.none [Capabilities::ListChangedCapability])"),
        "prompts are not: nothing offers one"
    );
    for method in [
        "resourcesList",
        "resourcesRead",
        "resourcesTemplatesList",
        "completionComplete",
    ] {
        let arm = gateway
            .split(&format!("Methods::ClientRequest.{method} params =>"))
            .nth(1)
            .and_then(|rest| rest.split('|').next())
            .unwrap_or_else(|| panic!("an arm for {method}"));
        assert!(
            !arm.contains("toolsOnly"),
            "{method} is served, not refused as tools only: {arm}"
        );
    }
    for method in ["promptsList", "promptsGet"] {
        let arm = gateway
            .split(&format!("Methods::ClientRequest.{method} params =>"))
            .nth(1)
            .and_then(|rest| rest.split('|').next())
            .unwrap_or_else(|| panic!("an arm for {method}"));
        assert!(
            arm.contains("toolsOnly"),
            "{method} is not declared and is refused"
        );
    }
    let waiting = read("gateway/waiting.json");
    let table = read("GATEWAY-CONFORMANCE.md");
    let value: serde_json::Value = serde_json::from_str(&waiting).expect("waiting.json");
    for (scenario, entry) in value["waiting"].as_object().expect("waiting") {
        assert!(
            table.contains(&format!("`{scenario}`")),
            "{scenario} is in GATEWAY-CONFORMANCE.md"
        );
        let reason = entry["reason"].as_str().expect("a reason");
        assert!(
            !reason.contains("declares neither the prompts nor the resources capability"),
            "{scenario}: the reason is stale, the gateway declares resources now"
        );
    }
    needs(
        "VO-13",
        &[
            "server_discover_declares_the_served_capabilities_vo_13",
            "a_method_of_every_declared_capability_is_served_vo_13",
            "prompts_and_the_tasks_methods_are_not_declared_and_are_refused_vo_13",
        ],
    );
}

// ---- VO-14 -----------------------------------------------------------------------------------

/// VO-14: the Atlas profile serves 293 entries; the tests need the checkout of LexLean and
/// run in the gateway job with `-- --ignored`, whose count `lib.sh` checks.
#[test]
fn an_atlas_profile_serves_all_293_entries_vo_14() {
    let entries = read("crates/model/tests/atlas/entries.tsv");
    assert_eq!(entries.lines().count(), 293, "the model's Atlas lexicon");
    let model = read("src/Profiles.lex.tex");
    assert!(
        model.contains("name: atlasEntryCount, parameters: [], result: Nat, body: 293"),
        "the model counts 293"
    );
    let tests = read("gateway/tool-tests/tests/vocabulary.rs")
        + &read("gateway/tool-tests/tests/drafts.rs")
        + &read("gateway/tool-tests/tests/equality.rs")
        + &read("gateway/tool-tests/tests/explain.rs")
        + &read("gateway/tool-tests/tests/origin.rs")
        + &read("gateway/tool-tests/tests/q2.rs")
        + &read("gateway/tool-tests/tests/q4.rs")
        + &read("gateway/tool-tests/tests/taskhost.rs")
        + &read("gateway/tool-tests/tests/tools.rs");
    // The verify fixtures have a job of their own (VP-07).
    let ignored =
        tests.matches("#[ignore").count() - tests.matches("#[ignore = \"another job:").count();
    let lib = read(".github/workflows/lexlean.yml");
    assert!(
        lib.contains(&format!(
            "gateway_ignored_tests gateway-tool-tests {ignored} "
        )),
        "the gateway job expects the {ignored} ignored tests of gateway-tool-tests"
    );
    needs(
        "VO-14",
        &[
            "an_atlas_profile_serves_all_293_entries_vo_14",
            "a_profile_without_the_module_atlas_serves_none_of_the_entries_vo_14",
        ],
    );
}

// ---- VO-15 -----------------------------------------------------------------------------------

/// VO-15: every refusal has a negative test.
#[test]
fn every_refusal_has_a_negative_test_vo_15() {
    let provision = read("gateway/bindings/gateway-binding/src/provision.rs");
    for name in [
        "NameInvalid",
        "ProfileExists",
        "ProjectUnreadable",
        "Compiler(",
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
    ] {
        assert!(provision.contains(name), "provision::Refusal has {name}");
    }
    let tests = read("gateway/bindings/gateway-binding/tests/provision.rs");
    for name in [
        "Refusal::NameInvalid",
        "Refusal::ProfileExists",
        "Refusal::ProjectUnreadable",
        "Refusal::Compiler(",
        "Refusal::LinkInBundle",
        "Refusal::ReservedEntry",
        "Refusal::SourcePackagesUnsupported",
        "Refusal::ReviewMissing",
        "Refusal::ReviewMalformed",
        "Refusal::ReviewStale",
        "Refusal::ReviewDuplicate",
        "Refusal::VocabularyTooLarge",
        "Refusal::IdentityRefused",
        "Refusal::StoreUnwritable",
    ] {
        assert!(tests.contains(name), "a test names {name}");
    }
    let core = tight(&read("gateway/src/Vocabulary.lex.tex"));
    for name in [
        "ProfileUnknown",
        "CatalogueMalformed",
        "VocabularyMissing",
        "VocabularyMalformed",
        "LanguageMismatch",
    ] {
        assert!(core.contains(name), "the core has the refusal {name}");
        assert!(
            read("gateway/tool-tests/tests/vocabulary.rs").contains(name),
            "a tool test provokes {name}"
        );
    }
    needs(
        "VO-15",
        &[
            "a_duplicate_path_and_a_duplicate_review_are_refused_by_the_identity_vo_15",
            "a_store_that_cannot_be_written_is_refused_vo_15",
            "every_refusal_has_a_code_and_the_compiler_refusals_carry_the_compilers_vo_15",
        ],
    );
    needs_named(&[
        "search_refusals_are_typed_vo_10",
        "a_catalogue_that_is_not_the_layout_is_refused_vo_09",
        "a_resource_that_is_not_found_is_32602_with_its_uri_vo_11",
        "another_reference_or_argument_is_32602_vo_12",
        "the_schema_of_lexlean_profiles_is_closed_vo_09",
        "the_schema_of_lexlean_lexicon_search_is_closed_vo_10",
    ]);
}

// ---- VO-16 -----------------------------------------------------------------------------------

/// VO-16: the planted defects are recorded with their command, diagnostic and restore.
#[test]
fn the_planted_defects_are_recorded_vo_16() {
    let profiles = read("PROFILES.md");
    let section = profiles
        .split("## Planted defects of the gateway side")
        .nth(1)
        .expect("PROFILES.md has the section 'Planted defects of the gateway side'");
    for defect in [
        "a profile file excluded from the identity",
        "a mixed-language package admitted",
        "an Atlas entry missing from the served lexicon",
        "a capability declared that is not served",
        "a profile installed without a review record",
        "a reserved entry copied into a project",
    ] {
        assert!(
            section.to_lowercase().contains(&defect.to_lowercase()),
            "the section records the defect: {defect}"
        );
    }
    for word in ["Command", "Diagnostic", "Restore"] {
        assert!(
            section.matches(word).count() >= 6,
            "each defect records its {word}"
        );
    }
}
