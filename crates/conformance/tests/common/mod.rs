//! Helpers the conformance tests share: the repository root and the names of
//! the tests in the workspace.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// Every `#[test]` function name in the workspace.
///
/// Read from the source rather than from `cargo test -- --list`, because this
/// runs *inside* `cargo test` and a nested invocation blocks on the target
/// directory lock. The scan is exact for the shape the workspace uses: a
/// `#[test]` attribute followed, possibly after further attributes, by the
/// function it annotates.
pub fn workspace_test_names(root: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut stack = vec![root.join("crates"), root.join("xtask")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().is_some_and(|n| n == "target") {
                    continue;
                }
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let Ok(text) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let mut armed = false;
                for line in text.lines() {
                    let line = line.trim();
                    if line == "#[test]" {
                        armed = true;
                    } else if armed {
                        if let Some(rest) = line.strip_prefix("fn ") {
                            let name: String = rest
                                .chars()
                                .take_while(|c| c.is_alphanumeric() || *c == '_')
                                .collect();
                            if !name.is_empty() {
                                names.insert(name);
                            }
                            armed = false;
                        } else if !line.starts_with('#') && !line.is_empty() {
                            armed = false;
                        }
                    }
                }
            }
        }
    }
    names
}
