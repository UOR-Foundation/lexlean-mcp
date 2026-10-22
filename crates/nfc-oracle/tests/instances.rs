//! NF-09 (instances): the kernel-checked instances of `src/Nfc.lex.tex` hold on
//! the full tables as well as on the sample image they are checked on.
//!
//! The instances are read from the module's theorem statements, so a new
//! instance is covered without a test edit. `lexlean verify` checks each on
//! the sample image (the full tables are too slow for the elaborator to
//! evaluate); here the exported decision runs each on both images and must
//! return the stated value on both, which is what ties the kernel-checked
//! statement to the tables admission reads.

mod common;

use common::root;
use nfc_oracle::{decode_on, is_nfc_octets_on, nfc_on, nfd_on, utf8, Image};

#[derive(Debug, PartialEq)]
enum Value {
    Bool(bool),
    List(Vec<u32>),
}

enum Input {
    Scalars(Vec<u32>),
    Octets(Vec<u8>),
}

struct Instance {
    name: String,
    function: String,
    input: Input,
    expected: Value,
}

/// `(cons 1 (cons 2 (nil Nat)))` as `[1, 2]`.
fn parse_list(text: &str) -> Vec<u32> {
    let mut out = Vec::new();
    let mut rest = text.trim();
    while let Some(after) = rest.strip_prefix("(cons ") {
        let (number, tail) = after.split_once(' ').expect("a cons has a tail");
        out.push(number.parse().expect("a number"));
        rest = tail;
    }
    assert!(rest.starts_with("(nil Nat)"), "a list ends in nil: {text}");
    out
}

fn instances() -> Vec<Instance> {
    let source =
        std::fs::read_to_string(root().join("src/Nfc.lex.tex")).expect("src/Nfc.lex.tex reads");
    let mut out = Vec::new();
    for block in source.split("theorem{").skip(1) {
        let name = block
            .lines()
            .find_map(|l| l.trim().strip_prefix("name: "))
            .expect("a theorem has a name")
            .trim_end_matches(',')
            .to_owned();
        let statement = block
            .lines()
            .find_map(|l| l.trim().strip_prefix("statement: "))
            .expect("a theorem has a statement")
            .trim_end_matches(',');
        let Some(inner) = statement
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
        else {
            panic!("{name}: unexpected statement shape {statement}");
        };
        let (left, right) = inner.rsplit_once(" = ").expect("an equation");
        let left = left
            .trim()
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .expect("a call");
        let expected = match right.trim() {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            list => Value::List(parse_list(list)),
        };
        let (function, argument) = left.split_once(' ').expect("a function and its arguments");
        let (function, input) = match function {
            "isNfc" | "toNfc" | "toNfd" => {
                let argument = argument
                    .strip_prefix("(NfcTables::sample) ")
                    .unwrap_or_else(|| panic!("{name}: an instance is stated on the sample image"));
                match argument.strip_prefix("(scalarsOfOctets ") {
                    Some(octets) => {
                        let octets = octets.strip_suffix(')').expect("closed");
                        (
                            function,
                            Input::Octets(
                                parse_list(octets)
                                    .iter()
                                    .map(|&o| u8::try_from(o).expect("octet"))
                                    .collect(),
                            ),
                        )
                    }
                    None => (function, Input::Scalars(parse_list(argument))),
                }
            }
            "scalarsOfOctets" => (
                function,
                Input::Octets(
                    parse_list(argument)
                        .iter()
                        .map(|&o| u8::try_from(o).expect("octet"))
                        .collect(),
                ),
            ),
            other => panic!("{name}: an instance of {other} is not covered"),
        };
        out.push(Instance {
            name,
            function: function.to_owned(),
            input,
            expected,
        });
    }
    out
}

fn run(image: Image, instance: &Instance) -> Value {
    let octets = match &instance.input {
        Input::Scalars(scalars) => utf8(scalars),
        Input::Octets(octets) => octets.clone(),
    };
    match instance.function.as_str() {
        "isNfc" => Value::Bool(is_nfc_octets_on(image, &octets)),
        "toNfc" => Value::List(nfc_on(image, &decode_on(image, &octets))),
        "toNfd" => Value::List(nfd_on(image, &decode_on(image, &octets))),
        "scalarsOfOctets" => Value::List(decode_on(image, &octets)),
        other => panic!("{other}"),
    }
}

#[test]
fn every_kernel_checked_instance_holds_on_the_full_tables_and_on_the_sample_nf_09() {
    let all = instances();
    assert!(all.len() >= 25, "{} instances read", all.len());
    for instance in &all {
        for image in [Image::Sample, Image::Full] {
            assert_eq!(
                run(image, instance),
                instance.expected,
                "{} on the {image:?} image",
                instance.name
            );
        }
    }
}

#[test]
fn the_instances_cover_the_cases_the_issue_names_nf_09() {
    let names: Vec<String> = instances().into_iter().map(|i| i.name).collect();
    for wanted in [
        "decomposedLetterIsNotNfc",
        "precomposedLetterIsNfc",
        "hangulLvComposes",
        "hangulLvtComposes",
        "compositionExclusionIsNotNfc",
        "compositionExclusionStaysDecomposed",
        "marksAreReordered",
    ] {
        assert!(
            names.iter().any(|n| n == wanted),
            "no instance named {wanted}"
        );
    }
}
