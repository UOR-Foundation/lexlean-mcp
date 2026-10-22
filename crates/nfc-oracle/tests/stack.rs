//! NF-13: no stage of the exported roots recurses to a depth bounded by the input.
//!
//! The roots `nfcRun` and `admitRun` are exported with the effects `allocation`
//! and `overflow` and without `recursion`: every self call of the module is a tail
//! call, which the export renders as a loop (SPEC section 17.13, RB-14). An output
//! with `recursion` would need a host stack that fits the input; these do not.
//! The runs below give the roots one MiB of scalars on a two MiB thread stack, which
//! a recursion of that depth would overflow, and every run returns a typed value.

mod common;

use std::path::Path;

use common::{on_stack, verdict_text};
use nfc_oracle::{admit, decode, is_nfc, nfc, nfd};
use unicode_normalization::UnicodeNormalization;

const STACK: usize = 2 * 1024 * 1024;
const MIB: usize = 1024 * 1024;

fn effects(package: &str) -> serde_json::Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(package)
        .join("provenance.json");
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).expect("provenance.json is committed"))
            .expect("provenance.json is JSON");
    json["effects"].clone()
}

#[test]
fn neither_root_declares_a_recursion_effect_nf_13() {
    for package in ["lexlean_mcp_nfc", "lexlean_mcp_admission"] {
        assert_eq!(
            effects(package),
            serde_json::json!(["allocation", "overflow"]),
            "{package}: a root that recurses to a depth bounded by its input states `recursion`"
        );
    }
}

fn repeat(unit: &[u32], times: usize) -> Vec<u32> {
    let mut out = Vec::with_capacity(unit.len() * times);
    for _ in 0..times {
        out.extend_from_slice(unit);
    }
    out
}

fn reference_nfc(scalars: &[u32]) -> Vec<u32> {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).expect("a scalar"))
        .nfc()
        .map(u32::from)
        .collect()
}

fn reference_nfd(scalars: &[u32]) -> Vec<u32> {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).expect("a scalar"))
        .nfd()
        .map(u32::from)
        .collect()
}

#[test]
fn one_mib_of_scalars_is_decided_on_a_two_mib_stack_nf_13() {
    let shapes: Vec<(&str, Vec<u32>)> = vec![
        ("ascii", repeat(&[0x61], MIB)),
        ("precomposed", repeat(&[0xE9], MIB)),
        ("decomposed", repeat(&[0x65, 0x301], MIB / 2)),
        ("hangul", repeat(&[0x1100, 0x1161, 0x11A8], MIB / 3)),
        ("deep decomposition", repeat(&[0x1F82], MIB)),
        ("one starter and marks", {
            let mut v = vec![0x61];
            v.extend(repeat(&[0x301], MIB - 1));
            v
        }),
    ];
    for (name, scalars) in shapes {
        assert!(scalars.len() >= MIB - 2, "{name}: {}", scalars.len());
        let expected_nfc = reference_nfc(&scalars);
        let expected_nfd = reference_nfd(&scalars);
        let expected_is = expected_nfc == scalars;
        let (is, composed, decomposed) = on_stack(STACK, {
            let scalars = scalars.clone();
            move || (is_nfc(&scalars), nfc(&scalars), nfd(&scalars))
        });
        assert_eq!(is, expected_is, "{name}: the verdict");
        assert!(composed == expected_nfc, "{name}: the NFC");
        assert!(decomposed == expected_nfd, "{name}: the NFD");
        let decoded = on_stack(STACK, {
            let octets: Vec<u8> = scalars
                .iter()
                .map(|&c| char::from_u32(c).expect("a scalar"))
                .collect::<String>()
                .into_bytes();
            move || decode(&octets)
        });
        assert!(decoded == scalars, "{name}: the reader");
    }
}

fn path_of(prefix: &str, unit: &str, times: usize, suffix: &str) -> Vec<u8> {
    let mut text = String::with_capacity(prefix.len() + unit.len() * times + suffix.len());
    text.push_str(prefix);
    for _ in 0..times {
        text.push_str(unit);
    }
    text.push_str(suffix);
    text.into_bytes()
}

#[test]
fn a_path_of_one_mib_of_scalars_is_admitted_or_refused_on_a_two_mib_stack_nf_13() {
    let mut cases: Vec<(&str, Vec<u8>, &str)> = vec![
        (
            "many components",
            path_of("src/", "a/", MIB / 2, "m.lex.tex"),
            "Admitted",
        ),
        (
            "one long component",
            path_of("src/", "a", MIB, ".lex.tex"),
            "Admitted",
        ),
        (
            "precomposed components",
            path_of("src/", "\u{e9}/", MIB / 2, "m.lex.tex"),
            "Admitted",
        ),
        (
            "dot dot after a long prefix",
            path_of("src/", "a/", MIB / 2, "../m.lex.tex"),
            "Refused DotDotComponent",
        ),
        (
            "empty component after a long prefix",
            path_of("src/", "a/", MIB / 2, "/m.lex.tex"),
            "Refused EmptyComponent",
        ),
        (
            "decomposed letter at the end",
            path_of("src/", "a/", MIB / 2, "e\u{301}.lex.tex"),
            "Refused NotNfc",
        ),
        (
            "decomposed letters throughout",
            path_of("src/", "e\u{301}", MIB / 2, ".lex.tex"),
            "Refused NotNfc",
        ),
        (
            "outside the roots",
            path_of("other/", "a/", MIB / 2, "m.lex.tex"),
            "Refused OutsideRoots",
        ),
        (
            "wrong suffix",
            path_of("src/", "a/", MIB / 2, "m.txt"),
            "Refused NotLexTex",
        ),
    ];
    let mut invalid = path_of("src/", "a/", MIB / 2, "m");
    invalid.extend_from_slice(&[0xFF]);
    invalid.extend_from_slice(b".lex.tex");
    cases.push(("invalid UTF-8 at the end", invalid, "Refused NotUtf8"));
    for (name, path, expected) in cases {
        let size = path.len();
        assert!(size >= MIB, "{name}: {size} octets");
        let got = on_stack(STACK, move || verdict_text(&admit(&[b"src"], &path)));
        assert_eq!(got, expected, "{name} ({size} octets)");
    }
}
