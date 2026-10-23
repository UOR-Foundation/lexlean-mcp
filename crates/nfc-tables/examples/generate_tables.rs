//! Writes `artifacts/nfc-tables-low.bin`, `artifacts/nfc-tables-high.bin` and `src/NfcTables.lex.tex` from the pinned
//! Unicode Character Database files:
//! `cargo run -p nfc-tables --example generate_tables`.

use std::path::Path;

fn main() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let root = manifest.join("../..");
    let files = nfc_tables::ucd::Files::load(&manifest.join("ucd")).expect("the pinned files read");
    let data = nfc_tables::ucd::Data::parse(&files).expect("the pinned files parse");
    let packed = nfc_tables::tables::pack(&data).expect("the tables pack");
    std::fs::create_dir_all(root.join("artifacts")).expect("the artifact directory exists");
    let (low, high) = nfc_tables::tables::halves(&packed.bytes);
    std::fs::write(root.join("artifacts/nfc-tables-low.bin"), low)
        .expect("the low half is written");
    std::fs::write(root.join("artifacts/nfc-tables-high.bin"), high)
        .expect("the high half is written");
    let sample = nfc_tables::tables::sample(&data, &nfc_tables::tables::sample_seeds())
        .expect("the sample packs");
    std::fs::write(root.join("artifacts/nfc-sample.bin"), &sample.bytes)
        .expect("the sample is written");
    std::fs::write(
        root.join("src/NfcTables.lex.tex"),
        nfc_tables::tables::module(&packed, &sample),
    )
    .expect("the module is written");
    println!(
        "{} octets, sha256 {}, depth {}, fast bound U+{:04X}; sample {} octets",
        packed.bytes.len(),
        nfc_tables::tables::sha256_hex(&packed.bytes),
        packed.depth,
        packed.fast_bound,
        sample.bytes.len()
    );
}
