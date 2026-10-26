//! EF-01 to EF-08: the closed host effect set of issue #7.
//!
//! The effect set, its codec and the theorems are LexLean source
//! (`src/Effects.lex.tex`), and `lexlean verify` checks the kernel side. These
//! tests read the same source as text and hold it against issue #7 (the ten
//! effects it names), against `EFFECTS.md` (the wire, the vectors, the mapping to
//! PrismPM#216 and the planted defects), and against the gateway-facing rules
//! (Q6). Nothing here decodes a wire: the codec has one source, the model.

use std::collections::BTreeSet;
use std::path::PathBuf;

/// E1 to E10 as issue #7 names them: constructor, wire name.
const ISSUE: [(&str, &str); 10] = [
    ("StoreGet", "store.get"),
    ("StorePut", "store.put"),
    ("EngineCheck", "engine.check"),
    ("EngineSnapshot", "engine.snapshot"),
    ("EngineFormat", "engine.format"),
    ("EngineBuild", "engine.build"),
    ("VerifyStart", "verify.start"),
    ("VerifyControl", "verify.control"),
    ("LexiconRead", "lexicon.read"),
    ("AttestationLookup", "attestation.lookup"),
];

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The source with runs of white space collapsed and brackets and braces tightened,
/// so the formatter's one-line and multi-line spellings of a declaration read the same.
fn tight(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("[ ", "[")
        .replace(" ]", "]")
}

fn source() -> String {
    tight(&read("src/Effects.lex.tex"))
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

/// The body of the declaration `kind{name: <name>, ...}`, without its braces.
fn declaration(source: &str, kind: &str, name: &str) -> String {
    let marker = format!("{kind}{{name: {name}, ");
    let at = source
        .find(&marker)
        .unwrap_or_else(|| panic!("src/Effects.lex.tex has no {kind} {name}"));
    let chunk = &source[at + kind.len() + 1..];
    chunk[..closing_brace(chunk)].to_string()
}

fn has_theorem(source: &str, name: &str) -> bool {
    source.contains(&format!("theorem{{name: {name}, "))
}

/// The statement and proof kind of a theorem.
fn theorem(source: &str, name: &str) -> (String, String) {
    let body = declaration(source, "theorem", name);
    let (_, rest) = body
        .split_once("statement: ")
        .unwrap_or_else(|| panic!("theorem {name} has no statement"));
    let (statement, proof) = rest
        .split_once(", proof: ")
        .unwrap_or_else(|| panic!("theorem {name} has no proof"));
    let kind = proof
        .split('{')
        .next()
        .expect("a proof has a kind")
        .to_string();
    (statement.to_string(), kind)
}

/// The constructor names of an inductive.
fn constructors(source: &str, name: &str) -> Vec<String> {
    let body = declaration(source, "inductive", name);
    let (_, list) = body
        .split_once("constructors: [")
        .unwrap_or_else(|| panic!("inductive {name} has no constructors"));
    list.split("{name: ")
        .skip(1)
        .map(|part| part.split(',').next().expect("a name").to_string())
        .collect()
}

fn definition_body(source: &str, name: &str) -> String {
    let body = declaration(source, "definition", name);
    let (_, rest) = body
        .split_once(", body: ")
        .unwrap_or_else(|| panic!("definition {name} has no body"));
    rest.split(", recursive_argument: ")
        .next()
        .expect("a body")
        .to_string()
}

/// The effects of `allEffects`, in order.
fn listed_effects(source: &str) -> Vec<String> {
    let body = definition_body(source, "allEffects");
    body.split("(ctor Effect.")
        .skip(1)
        .map(|part| part.split(')').next().expect("a name").to_string())
        .collect()
}

/// The text of the string literals of a body, in order.
fn strings(body: &str) -> Vec<String> {
    body.split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, text)| text.to_string())
        .collect()
}

/// `effectName`: constructor to wire name, as the source writes it.
fn names(source: &str) -> Vec<(String, String)> {
    let body = definition_body(source, "effectName");
    body.split("| Effect.")
        .skip(1)
        .map(|branch| {
            let (constructor, rest) = branch.split_once(" => ").expect("a branch");
            (constructor.to_string(), strings(rest).remove(0))
        })
        .collect()
}

/// `effectTag`: constructor to tag.
fn tags(source: &str) -> Vec<(String, u32)> {
    let body = definition_body(source, "effectTag");
    body.split("| Effect.")
        .skip(1)
        .map(|branch| {
            let (constructor, rest) = branch.split_once(" => ").expect("a branch");
            let number: String = rest.chars().take_while(char::is_ascii_digit).collect();
            (constructor.to_string(), number.parse().expect("a numeral"))
        })
        .collect()
}

/// A vector of the model: label, wire in words, expected in words.
#[derive(Debug, PartialEq, Eq)]
struct Vector {
    label: String,
    wire: String,
    expected: String,
}

fn effect_words(constructor: &str) -> String {
    let (index, (_, name)) = ISSUE
        .iter()
        .enumerate()
        .find(|(_, (c, _))| *c == constructor)
        .unwrap_or_else(|| panic!("{constructor} is not an effect of issue #7"));
    format!("E{} {name}", index + 1)
}

fn vectors(source: &str) -> Vec<Vector> {
    let body = definition_body(source, "acceptanceVectors");
    body.split("(record Vector {")
        .skip(1)
        .map(|chunk| {
            let label = strings(chunk).remove(0);
            let after_wire = chunk.split_once("wire := ").expect("a wire").1;
            let (wire, after) = after_wire
                .split_once(", expected := ")
                .expect("an expectation");
            let wire = if let Some(frame) = wire.strip_prefix("(ctor Wire.Frame ") {
                // An octet value above 255 is spelled in decimal, so no hex pair hides it.
                let big: Vec<String> = frame
                    .split("(cons ")
                    .skip(1)
                    .map(|part| {
                        part.chars()
                            .take_while(char::is_ascii_digit)
                            .collect::<String>()
                    })
                    .map(|number| {
                        let value: u32 = number.parse().expect("a numeral");
                        if value > 255 {
                            format!("{value}")
                        } else {
                            format!("{value:02x}")
                        }
                    })
                    .collect();
                if big.is_empty() {
                    "frame ()".to_string()
                } else {
                    format!("frame {}", big.join(" "))
                }
            } else {
                format!("name \"{}\"", strings(wire).remove(0))
            };
            let expected = if let Some(found) =
                after.strip_prefix("(ctor Result.ok [Effect, Refusal] (ctor Effect.")
            {
                effect_words(found.split(')').next().expect("a name"))
            } else {
                let refusal = after
                    .split_once("(ctor Refusal.")
                    .expect("a refusal")
                    .1
                    .split(')')
                    .next()
                    .expect("a name");
                format!("refused {refusal}")
            };
            Vector {
                label,
                wire,
                expected,
            }
        })
        .collect()
}

/// The rows of a table of `EFFECTS.md` under the heading that starts with `heading`.
fn rows(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut inside = false;
    let mut found = Vec::new();
    for line in document.lines() {
        if line.starts_with("## ") {
            inside = line.starts_with(heading);
            continue;
        }
        if inside && line.starts_with("| ") && !line.starts_with("| ---") {
            found.push(
                line.trim()
                    .trim_matches('|')
                    .split('|')
                    .map(|cell| cell.trim().trim_matches('`').to_string())
                    .collect::<Vec<_>>(),
            );
        }
    }
    if !found.is_empty() {
        found.remove(0); // the header row
    }
    found
}

fn section(document: &str, heading: &str) -> String {
    let mut inside = false;
    let mut text = String::new();
    for line in document.lines() {
        if line.starts_with("## ") {
            inside = line.starts_with(heading);
            continue;
        }
        if inside {
            text.push_str(line);
            text.push('\n');
        }
    }
    assert!(
        !text.trim().is_empty(),
        "EFFECTS.md has no section {heading}"
    );
    text
}

/// EF-01: the set is exactly the ten effects of issue #7.
#[test]
fn the_effect_set_is_exactly_e1_to_e10_ef_01() {
    let source = source();
    let issue: Vec<String> = ISSUE.iter().map(|(c, _)| (*c).to_string()).collect();
    assert_eq!(
        constructors(&source, "Effect"),
        issue,
        "the Effect inductive is E1 to E10, in order"
    );
    assert_eq!(
        listed_effects(&source),
        issue,
        "allEffects lists the same ten effects, in order"
    );
    let expected_names: Vec<(String, String)> = ISSUE
        .iter()
        .map(|(c, n)| ((*c).to_string(), (*n).to_string()))
        .collect();
    assert_eq!(
        names(&source),
        expected_names,
        "the wire names are the dotted names of the issue"
    );
    let expected_tags: Vec<(String, u32)> = ISSUE
        .iter()
        .enumerate()
        .map(|(index, (c, _))| ((*c).to_string(), u32::try_from(index).expect("small")))
        .collect();
    assert_eq!(
        tags(&source),
        expected_tags,
        "the wire tag of E<n> is n - 1, its position"
    );
    for name in ["tenEffects", "tagsAreZeroToNine", "namesAreTheIssueNames"] {
        let (_, proof) = theorem(&source, name);
        assert_eq!(proof, "decide", "{name} is decided by the kernel");
    }
    assert!(source
        .contains("tenEffects, parameters: [], statement: ((countEffects (allEffects)) = 10)"));
    let document = read("EFFECTS.md");
    let table: Vec<(String, String, String, String)> = rows(&document, "## The effects")
        .into_iter()
        .map(|cells| {
            (
                cells[0].clone(),
                cells[1].clone(),
                cells[2].clone(),
                cells[3].clone(),
            )
        })
        .collect();
    let expected: Vec<(String, String, String, String)> = ISSUE
        .iter()
        .enumerate()
        .map(|(index, (c, n))| {
            (
                format!("E{}", index + 1),
                (*n).to_string(),
                index.to_string(),
                (*c).to_string(),
            )
        })
        .collect();
    assert_eq!(
        table, expected,
        "EFFECTS.md lists E1 to E10 with the name, tag and constructor of the model"
    );
}

/// EF-02: the codec is total and typed, and decode inverts encode.
#[test]
fn decode_is_total_and_inverts_encode_ef_02() {
    let source = source();
    assert_eq!(
        constructors(&source, "Refusal"),
        ["UnknownId", "UnknownName", "MalformedFrame"],
        "a refusal is an unknown id, an unknown name or a malformed frame"
    );
    assert_eq!(constructors(&source, "Wire"), ["Frame", "Named"]);
    for (name, proof) in [("decodeEncode", "cases"), ("decodeEncodeName", "cases")] {
        let (statement, kind) = theorem(&source, name);
        assert_eq!(kind, proof, "{name} is proved by cases on the effect");
        assert!(
            statement.starts_with("((decode (encode")
                || statement.starts_with("((decode (encodeName"),
            "{name} states the round trip: {statement}"
        );
        assert!(
            statement.ends_with("(ctor Result.ok [Effect, Refusal] effect))"),
            "{name} decodes back to the same effect: {statement}"
        );
    }
    let decode = definition_body(&source, "decode");
    assert!(
        decode.contains("Wire.Frame octets => (decodeFrame octets)")
            && decode.contains("Wire.Named text => (decodeNameIn text (allEffects))"),
        "decode is total over both forms of a wire: {decode}"
    );
    let document = read("EFFECTS.md");
    let wire = section(&document, "## The wire");
    for needle in [
        "four octets",
        "big-endian",
        "position",
        "UnknownId",
        "UnknownName",
        "MalformedFrame",
    ] {
        assert!(
            wire.contains(needle),
            "the wire section of EFFECTS.md states `{needle}`"
        );
    }
}

/// EF-03: the negative theorem is stated over every wire.
#[test]
fn no_wire_outside_the_ten_decodes_ef_03() {
    let source = source();
    let (statement, kind) = theorem(&source, "decodeSound");
    assert_eq!(statement, "((soundDecode wire (decode wire)) = true)");
    assert_eq!(
        kind, "cases",
        "decodeSound splits the wire into its two forms"
    );
    let parameters = declaration(&source, "theorem", "decodeSound");
    assert!(
        parameters.contains("parameters: [wire : Wire]"),
        "decodeSound quantifies over every wire, not a list of examples: {parameters}"
    );
    for name in [
        "decodeWordSound",
        "decodeFrameSound",
        "decodeNameInSound",
        "decodeNameSound",
    ] {
        assert!(has_theorem(&source, name), "the soundness chain has {name}");
        assert_ne!(
            theorem(&source, name).1,
            "decide",
            "{name} is proved for every input, not decided on examples"
        );
    }
    let sound = definition_body(&source, "soundDecode");
    assert_eq!(
        sound,
        "(match result | Result.ok effect => (accepts wire effect) | Result.error refusal => true)",
        "a wire that decodes is a wire an effect encodes to"
    );
    let accepts = definition_body(&source, "accepts");
    assert!(
        accepts.contains("octetsEq octets (frameOf effect)") && accepts.contains("equal (effectName effect) text"),
        "`accepts` compares a wire with the two encodings of the effect, and nothing else: {accepts}"
    );
    let seen: BTreeSet<String> = vectors(&source)
        .into_iter()
        .filter_map(|vector| vector.expected.strip_prefix("refused ").map(str::to_string))
        .collect();
    let all: BTreeSet<String> = ["UnknownId", "UnknownName", "MalformedFrame"]
        .map(str::to_string)
        .into();
    assert_eq!(
        seen, all,
        "a vector refuses with each of the three refusals"
    );
    let (statement, kind) = theorem(&source, "everyRefusalHasAVector");
    assert_eq!(kind, "decide");
    assert!(statement.contains("refusalSeen (acceptanceVectors) 2"));
}

/// EF-04: tags and names are pairwise distinct.
#[test]
fn ids_and_names_are_pairwise_distinct_ef_04() {
    let source = source();
    assert_eq!(
        theorem(&source, "tagsAreDistinct").0,
        "((distinctNats (tagsOf (allEffects))) = true)"
    );
    assert_eq!(
        theorem(&source, "namesAreDistinct").0,
        "((distinctTexts (namesOf (allEffects))) = true)"
    );
    let tags: BTreeSet<u32> = tags(&source).into_iter().map(|(_, tag)| tag).collect();
    assert_eq!(tags.len(), 10, "ten tags");
    let names: BTreeSet<String> = names(&source).into_iter().map(|(_, name)| name).collect();
    assert_eq!(names.len(), 10, "ten names");
}

/// EF-05: the Engine-call effects are the four calls, one to one.
#[test]
fn the_engine_effects_are_the_four_engine_calls_ef_05() {
    let source = source();
    assert_eq!(
        constructors(&source, "EngineCall"),
        ["Check", "Snapshot", "Format", "Build"]
    );
    let map = definition_body(&source, "engineEffect");
    for (call, effect) in [
        ("Check", "EngineCheck"),
        ("Snapshot", "EngineSnapshot"),
        ("Format", "EngineFormat"),
        ("Build", "EngineBuild"),
    ] {
        assert!(
            map.contains(&format!("EngineCall.{call} => (ctor Effect.{effect})")),
            "{call} maps to {effect}: {map}"
        );
    }
    assert_eq!(
        theorem(&source, "fourEngineEffects").0,
        "((engineCallCount (allEffects)) = 4)"
    );
    assert!(has_theorem(&source, "engineCallRoundTrip"));
    assert!(has_theorem(&source, "everyEngineEffectIsACall"));
    assert_eq!(theorem(&source, "engineCallRoundTrip").1, "cases");
    assert_eq!(theorem(&source, "everyEngineEffectIsACall").1, "cases");
    let statement = theorem(&source, "engineEffectsAreE3ToE6").0;
    for needle in [
        "Check))) = 3",
        "Snapshot))) = 4",
        "Format))) = 5",
        "Build))) = 6",
    ] {
        assert!(
            statement.contains(needle),
            "the Engine calls are E3 to E6: {statement}"
        );
    }
}

/// EF-06: no effect name is a tool name or an MCP method name (Q6).
#[test]
fn no_effect_name_is_a_tool_or_method_name_ef_06() {
    let source = source();
    let reserved = strings(&definition_body(&source, "reservedWords"));
    for needle in [
        "lexlean_",
        "tools/",
        "resources/",
        "prompts/",
        "initialize",
        "ping",
    ] {
        assert!(
            reserved.iter().any(|word| word == needle),
            "the reserved words include `{needle}`"
        );
    }
    assert_eq!(
        theorem(&source, "effectNamesCarryNoToolOrMethodName").0,
        "((effectsClean (allEffects)) = true)"
    );
    assert!(has_theorem(&source, "aToolNameIsNotAnEffectName"));
    assert!(has_theorem(&source, "reservedWordsCoverTheTools"));
    // The kernel checks the octets of the names, and decides that they spell the names.
    for name in ["nameBytesSpellTheNames", "reservedBytesSpellTheWords"] {
        assert_eq!(
            theorem(&source, name).1,
            "decide",
            "{name} is decided by the kernel"
        );
    }
    let hex = |text: &str| -> String { text.bytes().map(|b| format!("{b:02x}")).collect() };
    let octets = definition_body(&source, "effectNameBytes");
    for (constructor, name) in names(&source) {
        let marker = format!("Effect.{constructor} => 0x");
        let at = octets
            .find(&marker)
            .unwrap_or_else(|| panic!("effectNameBytes has no {constructor}"));
        let digits: String = octets[at + marker.len()..]
            .chars()
            .take_while(char::is_ascii_hexdigit)
            .collect();
        assert_eq!(
            digits,
            hex(&name),
            "the octets of {constructor} spell {name}"
        );
    }
    let words = definition_body(&source, "reservedWords");
    for word in strings(&words) {
        assert!(
            words.contains(&format!("text := \"{word}\", bytes := 0x{}", hex(&word))),
            "the octets of the reserved word `{word}` spell it"
        );
    }
    // The same check, recomputed on the names the source holds.
    for (_, name) in names(&source) {
        assert!(
            name.bytes().all(|b| b.is_ascii_lowercase() || b == b'.'),
            "{name} is lower case letters and dots"
        );
        assert!(
            !name.starts_with('.') && !name.ends_with('.') && !name.contains(".."),
            "{name} has no empty segment"
        );
        for word in &reserved {
            assert!(!name.contains(word.as_str()), "{name} contains `{word}`");
        }
    }
}

/// EF-07: every effect has a request vector by frame and by name, and the table agrees with the model.
#[test]
fn every_effect_has_an_acceptance_vector_ef_07() {
    let source = source();
    let model = vectors(&source);
    for (index, (_, name)) in ISSUE.iter().enumerate() {
        let expected = format!("E{} {name}", index + 1);
        assert!(
            model
                .iter()
                .any(|v| v.expected == expected && v.wire.starts_with("frame ")),
            "{expected} has a vector by frame"
        );
        assert!(
            model
                .iter()
                .any(|v| v.expected == expected && v.wire.starts_with("name ")),
            "{expected} has a vector by name"
        );
    }
    for name in [
        "acceptanceVectorsPass",
        "everyEffectHasAVectorByFrameAndByName",
        "vectorTotal",
    ] {
        assert_eq!(
            theorem(&source, name).1,
            "decide",
            "{name} is decided by the kernel"
        );
    }
    assert!(
        theorem(&source, "vectorTotal")
            .0
            .ends_with(&format!("= {})", model.len())),
        "the model states its own vector count"
    );
    let document = read("EFFECTS.md");
    let table: Vec<Vector> = rows(&document, "## The acceptance vectors")
        .into_iter()
        .map(|cells| Vector {
            label: cells[0].clone(),
            wire: cells[1].clone(),
            expected: cells[2].clone(),
        })
        .collect();
    assert_eq!(
        table, model,
        "EFFECTS.md lists the vectors of the model, in order, and no other"
    );
}

/// EF-08: each theorem has a planted defect, recorded with its command, diagnostic and restore.
#[test]
fn each_gate_rejects_its_planted_defect_ef_08() {
    let document = read("EFFECTS.md");
    let rows = rows(&document, "## Planted defects");
    assert!(
        rows.len() >= 5,
        "five planted defects at least: {}",
        rows.len()
    );
    for cells in &rows {
        assert_eq!(
            cells.len(),
            5,
            "a row has a defect, a command, a gate, a diagnostic and a restore"
        );
        for cell in cells {
            assert!(!cell.is_empty(), "no cell of {cells:?} is empty");
        }
        assert!(
            cells[1].contains("lexlean verify"),
            "{} runs lexlean verify",
            cells[0]
        );
    }
    let defects: Vec<String> = rows.iter().map(|cells| cells[0].clone()).collect();
    for needle in [
        "eleventh effect",
        "duplicated id",
        "decode that accepts",
        "tool name",
        "Engine call",
    ] {
        assert!(
            defects.iter().any(|defect| defect.contains(needle)),
            "a planted defect is `{needle}`: {defects:?}"
        );
    }
    // The plants are restored: the committed source holds none of them.
    let source = source();
    assert!(
        !source.contains("Rogue"),
        "the eleventh effect is not in the source"
    );
    assert!(
        !source.contains("name: lexlean_"),
        "no effect is named after a tool"
    );
}

/// The closed fields of E1 and E2 (issue #9): the type, its constructors with their numbers, and
/// the refusal a number past them is.
/// A closed field: its type, its field name, its constructors with their numbers, its refusal, the
/// effects it belongs to, and its decoder.
type Field = (
    &'static str,
    &'static str,
    &'static [(&'static str, u32)],
    &'static str,
    &'static str,
    &'static str,
);

const FIELDS: [Field; 3] = [
    (
        "View",
        "view",
        &[("Stored", 0), ("Data", 1), ("Spans", 2)],
        "UnknownView",
        "store.get",
        "decodeView",
    ),
    (
        "Form",
        "form",
        &[("Plain", 0), ("Assemble", 1), ("Rewrite", 2)],
        "UnknownForm",
        "store.put",
        "decodeForm",
    ),
    (
        "Class",
        "class",
        &[("Accepted", 0), ("Refused", 1), ("Absent", 2)],
        "UnknownClass",
        "store.get and store.put",
        "decodeClass",
    ),
];

/// EF-09: the fields that E1 and E2 gained are closed types of the model, and no effect was added.
#[test]
fn the_view_form_and_class_of_the_store_effects_are_closed_ef_09() {
    let source = source();
    let issue: Vec<String> = ISSUE.iter().map(|(c, _)| (*c).to_string()).collect();
    assert_eq!(
        constructors(&source, "Effect"),
        issue,
        "the fields add no effect: Effect is still E1 to E10"
    );
    let refusals: Vec<String> = FIELDS.iter().map(|f| f.3.to_string()).collect();
    assert_eq!(
        constructors(&source, "FieldRefusal"),
        refusals,
        "one typed refusal for each closed field, and no other"
    );
    let document = read("EFFECTS.md");
    let table = rows(&document, "## The closed fields");
    assert_eq!(table.len(), FIELDS.len(), "EFFECTS.md has a row for each");
    for (kind, field, values, refusal, effect, decoder) in FIELDS {
        let expected: Vec<String> = values.iter().map(|(name, _)| (*name).to_string()).collect();
        assert_eq!(constructors(&source, kind), expected, "the type {kind}");
        let tag = format!("{}Tag", field);
        let at = format!("{}At", field);
        for name in [tag.as_str(), at.as_str(), decoder] {
            assert!(
                source.contains(&format!("definition{{name: {name}, ")),
                "src/Effects.lex.tex defines {name}"
            );
        }
        let tagged = definition_body(&source, &tag);
        for (name, number) in values {
            assert!(
                tagged.contains(&format!("{kind}.{name} => {number}")),
                "{kind}.{name} has the number {number}: {tagged}"
            );
        }
        let sound = format!("{decoder}Sound");
        let round = format!("{decoder}Tag");
        assert!(has_theorem(&source, &sound), "theorem {sound}");
        assert!(has_theorem(&source, &round), "theorem {round}");
        let (_, proof) = theorem(&source, &sound);
        assert_eq!(
            proof, "cases",
            "{sound} is proved for every number, by cases"
        );
        let (statement, _) = theorem(&source, &format!("{decoder}Past"));
        assert!(
            statement.contains(refusal),
            "{decoder}Past states the number past the last constructor is {refusal}: {statement}"
        );
        let row = table
            .iter()
            .find(|cells| cells[0] == kind)
            .unwrap_or_else(|| panic!("EFFECTS.md has a row for {kind}"));
        assert!(
            row[1].contains(effect),
            "the row of {kind} names {effect}: {row:?}"
        );
        for (name, number) in values {
            assert!(
                row[2].contains(&format!("{name} {number}")),
                "the row of {kind} lists {name} {number}: {row:?}"
            );
        }
        assert!(
            row[3].contains(refusal),
            "the row of {kind} names {refusal}"
        );
    }
    let defects = rows(&document, "## Planted defects");
    assert!(
        defects
            .iter()
            .any(|cells| cells[0].contains("unknown view")),
        "EFFECTS.md records the planted defect of an unknown view admitted"
    );
}
