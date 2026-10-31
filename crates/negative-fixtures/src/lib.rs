//! The AC-07 fixture register of issue #10: the pinned, classified table of
//! LexLean's `tests/negative` fixtures, each with the command result the CLI
//! gives for it.
//!
//! Dev and CI only. Nothing here runs a LexLean command. The crate holds the
//! expected-result files of the fixtures (`lexlean/<name>/case.toml` and
//! `lexlean/<name>/expected/*`, byte-identical to LexLean's), the register
//! (`register.toml`) that says what each fixture is, and the checks that tie
//! the two to the pinned LexLean tree. The tools of F4 to F7 and of #11 start
//! from this table; they do not re-derive it.
//!
//! A git tree hash is a function of the names, modes and contents beneath it.
//! Every fixture directory holds three entries, `case.toml`, `expected/` and
//! `project/`; the first two are vendored here, so only the hash of `project/`
//! is recorded and the hash of the fixture directory, and from them the hash of
//! `tests/negative` itself, is recomputed from bytes in this repository.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};

/// The LexLean commit the fixtures are pinned to: `.github/lexlean.rev`.
pub const PIN_COMMIT: &str = "e0b74b81e6f49c83596a957d232c8383a5db1a25";

/// The git tree of `tests/negative` at [`PIN_COMMIT`].
pub const PIN_TREE: &str = "f5a43991dec0c604c8647c5dd050f183a16832a6";

/// The schema tag of `register.toml`.
pub const SPEC: &str = "lexlean-mcp/fixture-register/1";

/// The class of a fixture the check tool of F4 runs.
pub const F4: &str = "F4";
/// The class of a fixture the build tool of F7 runs.
pub const F7: &str = "F7";
/// The class of a fixture the verify tool of #11 runs.
pub const VERIFY: &str = "#11";
/// The class of a fixture no tool reaches.
pub const EXCLUDED: &str = "excluded";

/// The fixtures issue #10 leaves out of AC-07, each with the reason it gives.
pub const ISSUE_EXCLUSIONS: [(&str, &str); 13] = [
    ("check-unknown-flag", "CLI argument parsing"),
    ("check-duplicate-selection", "CLI argument parsing"),
    ("path-symlink", "symlinks"),
    ("model-artifact-symlink", "symlinks"),
    ("git-lexicon-offline", "setup"),
    ("lexicon-unsafe-surface", "setup"),
    ("package-language-mismatch", "setup"),
    ("stale-lock", "setup"),
    ("production-expectation-stale", "outside the product"),
    ("production-request-malformed", "outside the product"),
    ("production-root-unknown", "outside the product"),
    ("production-target-unregistered", "outside the product"),
    (
        "migrate-module-without-notation",
        "migration, which no tool runs",
    ),
];

/// One row of the register.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixture {
    /// The directory name under `tests/negative`.
    pub name: String,
    /// The LexLean command of the fixture (`check`, `build`, `verify`, `lock`
    /// or `production`), as its `case.toml` states it.
    pub command: String,
    /// The exit status of the command result.
    pub exit: u32,
    /// The diagnostic codes of the command result, in order.
    pub codes: Vec<String>,
    /// SHA-256 of `expected/command.json`, the canonical command result.
    pub result_sha256: String,
    /// SHA-256 of `case.toml`.
    pub case_sha256: String,
    /// `F4`, `F7`, `#11` or `excluded`.
    pub class: String,
    /// For an excluded fixture, the reason issue #10 gives; empty otherwise.
    #[serde(default)]
    pub reason: String,
    /// The git tree of `project/`, which is not vendored.
    pub project_tree: String,
    /// The git trees of the other directories of the fixture (`toolchain`,
    /// `overlay`), which are not vendored; empty for most fixtures.
    #[serde(default)]
    pub extra: BTreeMap<String, String>,
    /// The git tree of the fixture directory.
    pub tree: String,
}

/// `register.toml`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Register {
    /// [`SPEC`].
    pub spec: String,
    /// The LexLean commit.
    pub commit: String,
    /// The git tree of `tests/negative` at that commit.
    pub tree: String,
    /// One row per fixture, sorted by name.
    pub fixture: Vec<Fixture>,
}

/// The vendored files, keyed by their path below `lexlean/`.
#[derive(Clone, Debug, Default)]
pub struct Vendored {
    /// Path (`<name>/case.toml`, `<name>/expected/command.json`) to bytes.
    pub files: BTreeMap<String, Vec<u8>>,
}

/// The sums of a register.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Counts {
    /// Every fixture.
    pub total: usize,
    /// Fixtures of class [`F4`].
    pub f4: usize,
    /// Fixtures of class [`F7`].
    pub f7: usize,
    /// Fixtures of class [`VERIFY`].
    pub verify: usize,
    /// Fixtures of class [`EXCLUDED`].
    pub excluded: usize,
}

impl Counts {
    /// The fixtures a tool reaches: every one that is not excluded.
    pub fn in_scope(&self) -> usize {
        self.f4 + self.f7 + self.verify
    }
}

/// Lowercase hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(text.get(i..i + 2)?, 16).ok())
        .collect()
}

/// SHA-256 of bytes.
pub fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The git object id of a blob.
pub fn blob_id(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("blob {}\0", bytes.len()).as_bytes());
    hasher.update(bytes);
    hex(&hasher.finalize())
}

/// An entry of a git tree: `(is a tree, name, object id in hexadecimal)`.
pub type Entry = (bool, String, String);

/// The git object id of a tree. Git orders entries by name with a directory
/// compared as if its name ended in a slash.
pub fn tree_id(entries: &[Entry]) -> Result<String, String> {
    let mut sorted: Vec<&Entry> = entries.iter().collect();
    sorted.sort_by_key(|(is_tree, name, _)| format!("{name}{}", if *is_tree { "/" } else { "" }));
    let mut body = Vec::new();
    for (is_tree, name, id) in sorted {
        body.extend_from_slice(if *is_tree { b"40000 " } else { b"100644 " });
        body.extend_from_slice(name.as_bytes());
        body.push(0);
        body.extend_from_slice(&unhex(id).filter(|raw| raw.len() == 20).ok_or_else(|| {
            format!("`{id}` of `{name}` is not a 40 digit hexadecimal object id")
        })?);
    }
    let mut hasher = Sha1::new();
    hasher.update(format!("tree {}\0", body.len()).as_bytes());
    hasher.update(&body);
    Ok(hex(&hasher.finalize()))
}

impl Vendored {
    /// Read `dir` (`lexlean/`): every file below `<name>/`.
    pub fn load(dir: &Path) -> Result<Self, String> {
        let mut files = BTreeMap::new();
        let mut stack = vec![(dir.to_path_buf(), String::new())];
        while let Some((path, prefix)) = stack.pop() {
            let entries =
                std::fs::read_dir(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("{}: {e}", path.display()))?;
                let name = entry.file_name().to_string_lossy().into_owned();
                let key = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}/{name}")
                };
                if entry.path().is_dir() {
                    stack.push((entry.path(), key));
                } else {
                    let bytes = std::fs::read(entry.path())
                        .map_err(|e| format!("{}: {e}", entry.path().display()))?;
                    files.insert(key, bytes);
                }
            }
        }
        Ok(Self { files })
    }

    /// The fixture names, sorted.
    pub fn names(&self) -> Vec<String> {
        // A set, because the path order of the files puts `a-b/` before `a/`.
        let names: std::collections::BTreeSet<String> = self
            .files
            .keys()
            .filter_map(|k| k.split('/').next().map(str::to_owned))
            .collect();
        names.into_iter().collect()
    }

    fn file(&self, name: &str, rest: &str) -> Result<&[u8], String> {
        self.files
            .get(&format!("{name}/{rest}"))
            .map(Vec::as_slice)
            .ok_or_else(|| format!("{name}: the vendored file `{rest}` is missing"))
    }

    /// The SHA-256 over the lines `<sha256>  <path>` of every vendored file in
    /// byte order of path: the checksum of the authority.
    pub fn manifest_sha256(&self) -> String {
        let mut text = String::new();
        for (path, bytes) in &self.files {
            text.push_str(&format!("{}  {path}\n", sha256(bytes)));
        }
        sha256(text.as_bytes())
    }

    /// The git tree of `<name>/expected`, from its vendored files.
    fn expected_tree(&self, name: &str) -> Result<String, String> {
        let prefix = format!("{name}/expected/");
        let entries: Vec<Entry> = self
            .files
            .iter()
            .filter_map(|(path, bytes)| {
                path.strip_prefix(&prefix)
                    .map(|file| (false, file.to_owned(), blob_id(bytes)))
            })
            .collect();
        if entries.iter().any(|(_, file, _)| file.contains('/')) {
            return Err(format!("{name}: `expected` holds a directory"));
        }
        tree_id(&entries)
    }

    /// The git tree of the fixture directory `<name>`, from its vendored files
    /// and the recorded trees of `project/` and of any other directory.
    pub fn fixture_tree(
        &self,
        name: &str,
        project_tree: &str,
        extra: &BTreeMap<String, String>,
    ) -> Result<String, String> {
        let mut entries = vec![
            (
                false,
                "case.toml".to_owned(),
                blob_id(self.file(name, "case.toml")?),
            ),
            (true, "expected".to_owned(), self.expected_tree(name)?),
            (true, "project".to_owned(), project_tree.to_owned()),
        ];
        for (other, tree) in extra {
            if ["case.toml", "expected", "project"].contains(&other.as_str()) {
                return Err(format!(
                    "{name}: `{other}` is recorded as an extra directory"
                ));
            }
            entries.push((true, other.clone(), tree.clone()));
        }
        let others: Vec<&String> = self
            .files
            .keys()
            .filter(|p| {
                p.strip_prefix(&format!("{name}/"))
                    .is_some_and(|rest| rest != "case.toml" && !rest.starts_with("expected/"))
            })
            .collect();
        if !others.is_empty() {
            return Err(format!("{name}: unexpected vendored files {others:?}"));
        }
        tree_id(&entries)
    }
}

/// The class of a fixture, from its command and the issue's exclusions.
pub fn class_of(name: &str, command: &str) -> Result<(&'static str, &'static str), String> {
    if let Some((_, reason)) = ISSUE_EXCLUSIONS.iter().find(|(n, _)| *n == name) {
        return Ok((EXCLUDED, reason));
    }
    match command {
        "check" => Ok((F4, "")),
        "build" => Ok((F7, "")),
        "verify" => Ok((VERIFY, "")),
        other => Err(format!(
            "{name}: command `{other}` is neither run by a tool nor left out by issue #10"
        )),
    }
}

/// Derive the row of `name` from the vendored bytes alone, plus the recorded
/// tree of its `project/`.
pub fn derive(
    name: &str,
    vendored: &Vendored,
    project_tree: &str,
    extra: &BTreeMap<String, String>,
) -> Result<Fixture, String> {
    let case_bytes = vendored.file(name, "case.toml")?;
    let result_bytes = vendored.file(name, "expected/command.json")?;
    let case: toml::Table = std::str::from_utf8(case_bytes)
        .map_err(|e| format!("{name}: case.toml is not UTF-8: {e}"))?
        .parse()
        .map_err(|e| format!("{name}: case.toml does not parse: {e}"))?;
    let command = case
        .get("command")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("{name}: case.toml has no command"))?
        .to_owned();
    let case_exit = case
        .get("expected_exit")
        .and_then(toml::Value::as_integer)
        .ok_or_else(|| format!("{name}: case.toml has no expected_exit"))?;
    let result: serde_json::Value = serde_json::from_slice(result_bytes)
        .map_err(|e| format!("{name}: command.json does not parse: {e}"))?;
    let exit = result["exit_code"]
        .as_i64()
        .ok_or_else(|| format!("{name}: command.json has no exit_code"))?;
    if exit != case_exit {
        return Err(format!(
            "{name}: case.toml expects exit {case_exit} and command.json says {exit}"
        ));
    }
    if result["command"].as_str() != Some(command.as_str()) {
        return Err(format!(
            "{name}: case.toml runs `{command}` and command.json is the result of `{}`",
            result["command"]
        ));
    }
    let codes = result["diagnostics"]
        .as_array()
        .ok_or_else(|| format!("{name}: command.json has no diagnostics"))?
        .iter()
        .map(|d| {
            d["code"]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{name}: a diagnostic has no code"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (class, reason) = class_of(name, &command)?;
    Ok(Fixture {
        name: name.to_owned(),
        command,
        exit: u32::try_from(exit).map_err(|_| format!("{name}: exit {exit} is not a status"))?,
        codes,
        result_sha256: sha256(result_bytes),
        case_sha256: sha256(case_bytes),
        class: class.to_owned(),
        reason: reason.to_owned(),
        project_tree: project_tree.to_owned(),
        extra: extra.clone(),
        tree: vendored.fixture_tree(name, project_tree, extra)?,
    })
}

impl Register {
    /// Read `register.toml`.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The sums.
    pub fn counts(&self) -> Counts {
        let of = |class: &str| self.fixture.iter().filter(|f| f.class == class).count();
        Counts {
            total: self.fixture.len(),
            f4: of(F4),
            f7: of(F7),
            verify: of(VERIFY),
            excluded: of(EXCLUDED),
        }
    }

    /// Render as `register.toml`: one table per fixture, in the order of the
    /// rows.
    pub fn render(&self) -> String {
        let mut out = format!(
            "# crates/negative-fixtures/register.toml --- the AC-07 fixture register of issue #10.\n\
             #\n\
             # One row per directory of tests/negative in LexLean at the pinned commit.\n\
             # Written by `cargo run -p negative-fixtures --example import`; every field but\n\
             # `class`, `reason`, `project_tree` and `extra` is read from the vendored files, and the\n\
             # tests recompute all of them. See FIXTURES.md.\n\n\
             spec = {:?}\ncommit = {:?}\ntree = {:?}\n",
            self.spec, self.commit, self.tree
        );
        for f in &self.fixture {
            let codes: Vec<String> = f.codes.iter().map(|c| format!("{c:?}")).collect();
            out.push_str(&format!(
                "\n[[fixture]]\nname = {:?}\ncommand = {:?}\nexit = {}\ncodes = [{}]\nclass = {:?}\n",
                f.name,
                f.command,
                f.exit,
                codes.join(", "),
                f.class
            ));
            if !f.reason.is_empty() {
                out.push_str(&format!("reason = {:?}\n", f.reason));
            }
            out.push_str(&format!(
                "result_sha256 = {:?}\ncase_sha256 = {:?}\nproject_tree = {:?}\n",
                f.result_sha256, f.case_sha256, f.project_tree
            ));
            if !f.extra.is_empty() {
                let pairs: Vec<String> = f
                    .extra
                    .iter()
                    .map(|(k, v)| format!("{k} = {v:?}"))
                    .collect();
                out.push_str(&format!("extra = {{ {} }}\n", pairs.join(", ")));
            }
            out.push_str(&format!("tree = {:?}\n", f.tree));
        }
        out
    }
}

/// The tree of `tests/negative` from the rows' fixture trees.
pub fn root_tree(register: &Register) -> Result<String, String> {
    let entries: Vec<Entry> = register
        .fixture
        .iter()
        .map(|f| (true, f.name.clone(), f.tree.clone()))
        .collect();
    tree_id(&entries)
}

/// The vendored files are the pinned tree: the register names exactly the
/// vendored fixtures, every digest is the digest of its file, and the trees
/// recompute to the recorded ones and to [`PIN_TREE`]. Returns every violation.
pub fn check_pin(register: &Register, vendored: &Vendored) -> Vec<String> {
    let mut out = Vec::new();
    if register.spec != SPEC {
        out.push(format!("spec is `{}`, not `{SPEC}`", register.spec));
    }
    if register.commit != PIN_COMMIT {
        out.push(format!(
            "commit is {}, not the pinned {PIN_COMMIT}",
            register.commit
        ));
    }
    if register.tree != PIN_TREE {
        out.push(format!(
            "tree is {}, not the pinned {PIN_TREE}",
            register.tree
        ));
    }
    let listed: Vec<&str> = register.fixture.iter().map(|f| f.name.as_str()).collect();
    let vendored_names = vendored.names();
    if listed
        != vendored_names
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    {
        out.push(format!(
            "the register names {} fixtures and {} are vendored, or their names differ",
            listed.len(),
            vendored_names.len()
        ));
    }
    for f in &register.fixture {
        let digest = |rest: &str| vendored.file(&f.name, rest).map(sha256);
        match digest("case.toml") {
            Ok(d) if d == f.case_sha256 => {}
            Ok(d) => out.push(format!(
                "{}: case.toml has SHA-256 {d}, the register says {}",
                f.name, f.case_sha256
            )),
            Err(e) => out.push(e),
        }
        match digest("expected/command.json") {
            Ok(d) if d == f.result_sha256 => {}
            Ok(d) => out.push(format!(
                "{}: expected/command.json has SHA-256 {d}, the register says {}",
                f.name, f.result_sha256
            )),
            Err(e) => out.push(e),
        }
        match vendored.fixture_tree(&f.name, &f.project_tree, &f.extra) {
            Ok(t) if t == f.tree => {}
            Ok(t) => out.push(format!(
                "{}: the vendored files make the tree {t}, the register says {}",
                f.name, f.tree
            )),
            Err(e) => out.push(e),
        }
    }
    match root_tree(register) {
        Ok(t) if t == PIN_TREE => {}
        Ok(t) => out.push(format!(
            "the rows make the tree {t} for tests/negative, the pin is {PIN_TREE}"
        )),
        Err(e) => out.push(e),
    }
    out
}

/// Every row says what the vendored bytes say. Returns every violation.
pub fn check_derived(register: &Register, vendored: &Vendored) -> Vec<String> {
    let mut out = Vec::new();
    for f in &register.fixture {
        match derive(&f.name, vendored, &f.project_tree, &f.extra) {
            Ok(derived) if derived == *f => {}
            Ok(derived) => out.push(format!(
                "{}: the register row is not what the vendored bytes give\n    row:     {f:?}\n    derived: {derived:?}",
                f.name
            )),
            Err(e) => out.push(e),
        }
    }
    out
}

/// Every fixture is classified exactly once, by its command and the issue's
/// exclusions, and an excluded fixture has its reason. Returns every violation.
pub fn check_classes(register: &Register) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for f in &register.fixture {
        if seen.contains(&f.name.as_str()) {
            out.push(format!("{}: listed twice", f.name));
        }
        seen.push(&f.name);
        if ![F4, F7, VERIFY, EXCLUDED].contains(&f.class.as_str()) {
            out.push(format!(
                "{}: class `{}` is not one of the four",
                f.name, f.class
            ));
            continue;
        }
        match class_of(&f.name, &f.command) {
            Ok((class, _)) if class == f.class => {}
            Ok((class, _)) => out.push(format!(
                "{}: class is {}, its command `{}` and the issue's exclusions give {class}",
                f.name, f.class, f.command
            )),
            Err(e) => out.push(e),
        }
        if f.class == EXCLUDED && f.reason.trim().is_empty() {
            out.push(format!("{}: excluded without a reason", f.name));
        }
        if f.class != EXCLUDED && !f.reason.is_empty() {
            out.push(format!("{}: has a reason and is not excluded", f.name));
        }
    }
    out
}

/// The excluded fixtures are the issue's twelve, each with the issue's reason,
/// and no other. Returns every violation.
pub fn check_exclusions(register: &Register) -> Vec<String> {
    let mut out = Vec::new();
    for (name, reason) in ISSUE_EXCLUSIONS {
        match register.fixture.iter().find(|f| f.name == name) {
            None => out.push(format!(
                "{name}: the issue excludes it and the register lacks it"
            )),
            Some(f) if f.class != EXCLUDED => out.push(format!(
                "{name}: the issue excludes it and the register classes it {}",
                f.class
            )),
            Some(f) if f.reason != reason => out.push(format!(
                "{name}: the issue's reason is `{reason}`, the register's is `{}`",
                f.reason
            )),
            Some(_) => {}
        }
    }
    for f in register.fixture.iter().filter(|f| f.class == EXCLUDED) {
        if !ISSUE_EXCLUSIONS.iter().any(|(n, _)| *n == f.name) {
            out.push(format!(
                "{}: excluded, and the issue does not exclude it",
                f.name
            ));
        }
    }
    out
}
