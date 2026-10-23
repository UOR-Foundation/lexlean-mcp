//! Shared by the oracle tests.
#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nfc_tables::ucd::{self, Data, Files, Row};
use sha2::{Digest, Sha256};

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/nfc-oracle is two below the root")
        .to_path_buf()
}

/// Lowercase hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of bytes, as hexadecimal.
pub fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The pinned files.
pub fn files() -> Files {
    Files::load(&nfc_tables::ucd_dir()).expect("the pinned files read")
}

/// What the algorithms read from them.
pub fn data() -> Data {
    Data::parse(&files()).expect("the pinned files parse")
}

/// Every row of `NormalizationTest.txt`.
pub fn rows() -> Vec<Row> {
    ucd::rows(&files().normalization_test).expect("NormalizationTest.txt parses")
}

/// The text of a scalar sequence.
pub fn text(scalars: &[u32]) -> String {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or_else(|| panic!("U+{c:04X} is not a scalar value")))
        .collect()
}

/// `U+0041 U+0301`, for a message.
pub fn show(scalars: &[u32]) -> String {
    scalars
        .iter()
        .map(|c| format!("{c:04X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A function of a scalar sequence, remembered, because the file repeats strings.
pub struct Memo<T> {
    seen: HashMap<Vec<u32>, T>,
    calls: usize,
}

impl<T: Clone> Memo<T> {
    /// An empty memo.
    pub fn new() -> Self {
        Memo {
            seen: HashMap::new(),
            calls: 0,
        }
    }

    /// `f(key)`, computed once per distinct key.
    pub fn get(&mut self, key: &[u32], f: impl FnOnce(&[u32]) -> T) -> T {
        if let Some(found) = self.seen.get(key) {
            return found.clone();
        }
        self.calls += 1;
        let value = f(key);
        self.seen.insert(key.to_vec(), value.clone());
        value
    }

    /// How many distinct keys were computed.
    pub fn distinct(&self) -> usize {
        self.calls
    }
}

/// The text of `src/Admission.lex.tex`.
pub fn admission_source() -> String {
    std::fs::read_to_string(root().join("src/Admission.lex.tex"))
        .expect("src/Admission.lex.tex reads")
}

/// The text of the `kind{name: ..., ...}` block of `name`.
pub fn block<'a>(source: &'a str, kind: &str, name: &str) -> &'a str {
    let head = format!("{kind}{{\n  name: {name},");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("no {kind} {name}"));
    let end = source[start..].find("\n}\n").expect("a block ends") + start;
    &source[start..end]
}

/// The constructors of `Refusal`, in the order the module states them.
pub fn refusal_names() -> Vec<String> {
    let source = admission_source();
    let refusal = block(&source, "inductive", "Refusal");
    refusal
        .split("{name: ")
        .skip(1)
        .map(|part| part.split(',').next().expect("a name").to_owned())
        .collect()
}

/// The verdict words of `admitRun` as the constructor text of the module:
/// `Admitted` or `Refused NotNfc`.
pub fn verdict_text(words: &[u64]) -> String {
    match words {
        [0] => "Admitted".to_owned(),
        [1, index] => {
            let names = refusal_names();
            let name = names
                .get(usize::try_from(*index).expect("a small index"))
                .unwrap_or_else(|| panic!("refusal index {index} is past the constructors"));
            format!("Refused {name}")
        }
        other => panic!("unexpected verdict words {other:?}"),
    }
}

/// Run `f` on a thread with a stack of `bytes`, and give back its result.
///
/// # Panics
/// When the thread cannot start, or `f` panics.
pub fn on_stack<T: Send + 'static>(bytes: usize, f: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(bytes)
        .spawn(f)
        .expect("the thread starts")
        .join()
        .expect("the work finishes on that stack without panicking")
}

/// Run `f` on a thread with a stack of `bytes`, and give back its result, or fail the
/// test when it has not finished after `limit`. The thread is left to the process.
///
/// # Panics
/// When the thread cannot start, `f` panics, or `limit` passes.
pub fn on_stack_within<T: Send + 'static>(
    bytes: usize,
    limit: std::time::Duration,
    f: impl FnOnce() -> T + Send + 'static,
) -> (T, std::time::Duration) {
    let (sender, receiver) = std::sync::mpsc::channel();
    let started = std::time::Instant::now();
    std::thread::Builder::new()
        .stack_size(bytes)
        .spawn(move || {
            let value = f();
            let _ = sender.send(value);
        })
        .expect("the thread starts");
    match receiver.recv_timeout(limit) {
        Ok(value) => (value, started.elapsed()),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            panic!("the work had not finished after {limit:?}")
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            panic!("the work finished on that stack without a result: it panicked")
        }
    }
}
