//! The executable measures of Q2 and Q8 of issue #15.
//!
//! Q2 asks that each in-scope function of LexLean's public API be reached by
//! exactly one tool. Q8 asks that every diagnostic code resolve through
//! `lexlean_explain`. Neither can be run end to end before the tools exist
//! (#9 to #12), but both rest on facts of the pinned LexLean that can be
//! reproduced and checked now:
//!
//! * the public functions of the stable surface (SPEC 24.1, 24.6, 24.7 and
//!   24.8), recomputed from the vendored source files of the pinned commit and
//!   tied to the tool that exposes each, or to the reason none does;
//! * the closed diagnostic registry (SPEC 26), its generated `ERRORS.md`, and
//!   the codes the negative fixtures of AC-07 return.
//!
//! Dev and CI only. Nothing here runs a LexLean command. Every check returns
//! its violations as a list so that a test can hand it a copy with one defect
//! planted and require the defect to be reported (R3, `QUALITY.md`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use negative_fixtures::{blob_id, sha256};
use serde::Deserialize;

pub use negative_fixtures::PIN_COMMIT;

/// The schema tag of `sources.toml`.
pub const SOURCES_SPEC: &str = "lexlean-mcp/quality-sources/1";

/// The schema tag of `map.toml`.
pub const MAP_SPEC: &str = "lexlean-mcp/api-map/1";

/// What a vendored file is for.
pub const SURFACE: &str = "surface";
/// The registry of diagnostic codes.
pub const REGISTRY: &str = "registry";
/// The generated entries of `lexlean explain`.
pub const ENTRIES: &str = "entries";

/// One vendored file of the pinned LexLean.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    /// The path in the LexLean repository.
    pub path: String,
    /// The path under `lexlean/` of this crate. Source files carry the suffix
    /// `.txt` so that no tool of this workspace reads them as code.
    pub vendored: String,
    /// The git blob id of the file at the pinned commit.
    pub git_blob: String,
    /// SHA-256 of the file.
    pub sha256: String,
    /// The length of the file in bytes.
    pub bytes: u64,
    /// `surface`, `registry` or `entries`.
    pub role: String,
    /// For a surface file, the module the functions are public in.
    #[serde(default)]
    pub module: String,
    /// For a surface file, the only functions that are part of the stable
    /// surface (the file also holds functions of a hidden module).
    #[serde(default)]
    pub only: Vec<String>,
}

/// `sources.toml`: the vendored files and what pins them.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sources {
    /// The schema tag.
    pub spec: String,
    /// The pinned LexLean commit.
    pub commit: String,
    /// One row per vendored file, in order of path.
    pub source: Vec<Source>,
}

/// A tool of the product, as an issue defines it.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    /// The tool name.
    pub name: String,
    /// The issue that defines it.
    pub issue: u32,
}

/// One public function and what becomes of it.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Function {
    /// `Engine::check` for a method, `sdk::assemble` for a function of a module.
    pub path: String,
    /// The signature as the pinned source states it, on one line.
    pub signature: String,
    /// The one tool that exposes the function.
    #[serde(default)]
    pub tool: Option<String>,
    /// For a function no tool exposes, why.
    #[serde(default)]
    pub reason: Option<String>,
    /// The issue the reason is taken from, or the pinned SPEC section when the
    /// issues are silent (`0`).
    #[serde(default)]
    pub issue: Option<u32>,
}

/// `map.toml`: the API-to-tool map.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Map {
    /// The schema tag.
    pub spec: String,
    /// The pinned LexLean commit.
    pub commit: String,
    /// The tools the issues define.
    pub tool: Vec<Tool>,
    /// One row per public function.
    pub function: Vec<Function>,
}

/// A public function found in a source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    /// `Engine::check` or `sdk::assemble`.
    pub path: String,
    /// The signature on one line.
    pub signature: String,
}

impl Sources {
    /// Read `sources.toml`.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Render as `sources.toml`.
    pub fn render(&self) -> String {
        let mut out = format!(
            "# crates/quality-map/sources.toml --- the files of LexLean that Q2 and Q8 are measured against.\n\
             #\n\
             # Written by `cargo run -p quality-map --example import`; every field is\n\
             # recomputed by the tests (QM-01, QM-04) from the vendored bytes.\n\n\
             spec = \"{}\"\ncommit = \"{}\"\n",
            self.spec, self.commit
        );
        for s in &self.source {
            out.push_str(&format!(
                "\n[[source]]\npath = \"{}\"\nvendored = \"{}\"\ngit_blob = \"{}\"\nsha256 = \"{}\"\nbytes = {}\nrole = \"{}\"\n",
                s.path, s.vendored, s.git_blob, s.sha256, s.bytes, s.role
            ));
            if !s.module.is_empty() {
                out.push_str(&format!("module = \"{}\"\n", s.module));
            }
            if !s.only.is_empty() {
                let only: Vec<String> = s.only.iter().map(|n| format!("\"{n}\"")).collect();
                out.push_str(&format!("only = [{}]\n", only.join(", ")));
            }
        }
        out
    }
}

impl Map {
    /// Read `map.toml`.
    pub fn load(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// The functions a tool exposes, by path.
    pub fn exposed(&self) -> BTreeSet<String> {
        self.function
            .iter()
            .filter(|f| f.tool.is_some())
            .map(|f| f.path.clone())
            .collect()
    }
}

/// The vendored files, by their path under `lexlean/`.
pub type Files = BTreeMap<String, Vec<u8>>;

/// Read every file `sources` names from `dir`.
pub fn load_files(dir: &Path, sources: &Sources) -> Result<Files, String> {
    let mut files = Files::new();
    for s in &sources.source {
        let path = dir.join(&s.vendored);
        let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        files.insert(s.vendored.clone(), bytes);
    }
    Ok(files)
}

/// The SHA-256 over the lines `<sha256>  <path>` of the vendored files, in
/// byte order of path: the checksum of the authorities that cite them.
pub fn manifest_sha256(files: &Files) -> String {
    let mut text = String::new();
    for (path, bytes) in files {
        text.push_str(&format!("{}  {path}\n", sha256(bytes)));
    }
    sha256(text.as_bytes())
}

/// The manifest of the files with one of the given roles.
pub fn manifest_of(sources: &Sources, files: &Files, roles: &[&str]) -> String {
    let chosen: Files = sources
        .source
        .iter()
        .filter(|s| roles.contains(&s.role.as_str()))
        .filter_map(|s| {
            files
                .get(&s.vendored)
                .map(|b| (s.vendored.clone(), b.clone()))
        })
        .collect();
    manifest_sha256(&chosen)
}

/// The vendored files are the files of the pinned commit: each has the git blob
/// id, the SHA-256 and the length the table records, the table names the pin
/// that `.github/lexlean.rev` names, and no file is missing or extra.
pub fn check_sources(sources: &Sources, files: &Files, rev_commit: &str) -> Vec<String> {
    let mut out = Vec::new();
    if sources.spec != SOURCES_SPEC {
        out.push(format!(
            "sources.toml has spec {}, not {SOURCES_SPEC}",
            sources.spec
        ));
    }
    if sources.commit != rev_commit {
        out.push(format!(
            "sources.toml pins {} and .github/lexlean.rev pins {rev_commit}: import again",
            sources.commit
        ));
    }
    let mut seen = BTreeSet::new();
    for s in &sources.source {
        if !seen.insert(&s.vendored) {
            out.push(format!("{} is listed twice", s.vendored));
        }
        let Some(bytes) = files.get(&s.vendored) else {
            out.push(format!("{} is not vendored", s.vendored));
            continue;
        };
        if blob_id(bytes) != s.git_blob {
            out.push(format!(
                "{}: the git blob id is {} and the table records {}",
                s.vendored,
                blob_id(bytes),
                s.git_blob
            ));
        }
        if sha256(bytes) != s.sha256 {
            out.push(format!(
                "{}: the SHA-256 is not the recorded one",
                s.vendored
            ));
        }
        if bytes.len() as u64 != s.bytes {
            out.push(format!(
                "{}: the length is not the recorded one",
                s.vendored
            ));
        }
        if ![SURFACE, REGISTRY, ENTRIES].contains(&s.role.as_str()) {
            out.push(format!("{}: unknown role {}", s.vendored, s.role));
        }
        if s.role == SURFACE && s.module.is_empty() {
            out.push(format!("{}: a surface file names its module", s.vendored));
        }
    }
    for name in files.keys() {
        if !sources.source.iter().any(|s| &s.vendored == name) {
            out.push(format!("{name} is vendored and not in sources.toml"));
        }
    }
    out
}

/// The name an inherent `impl` header gives, or `None` for a trait impl.
fn inherent_owner(header: &str) -> Option<String> {
    let rest = header.strip_prefix("impl")?;
    let mut rest = rest.trim_start();
    if rest.starts_with('<') {
        let mut depth = 0usize;
        let mut end = None;
        for (i, c) in rest.char_indices() {
            match c {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(i + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        rest = rest[end?..].trim_start();
    }
    if rest.contains(" for ") {
        return None;
    }
    let name: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then_some(name)
}

/// The name and one-line signature when `line` starts a `pub fn`.
fn public_fn(lines: &[&str], at: usize, indent: usize) -> Option<(String, String)> {
    let line = lines[at];
    let rest = line.get(indent..)?.strip_prefix("pub ")?;
    let mut rest = rest;
    for qualifier in ["const ", "async ", "unsafe "] {
        rest = rest.strip_prefix(qualifier).unwrap_or(rest);
    }
    let after = rest.strip_prefix("fn ")?;
    let name: String = after
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() {
        return None;
    }
    let mut signature = String::new();
    for l in &lines[at..] {
        let t = l.trim();
        signature.push_str(t);
        signature.push(' ');
        if t.ends_with('{') || t.ends_with(';') {
            break;
        }
    }
    let mut signature = signature
        .trim()
        .trim_end_matches('{')
        .trim_end_matches(';')
        .trim()
        .to_string();
    signature = signature
        .replace("( ", "(")
        .replace(" )", ")")
        .replace(",)", ")");
    Some((name, signature))
}

/// The public functions of one source file: a `pub fn` at the left margin is a
/// function of `module`, and a `pub fn` four columns in, inside an inherent
/// `impl` at the left margin, is a method of its type. `pub(crate)` is not
/// public. The scan stops at the unit tests. A non-empty `only` keeps just the
/// functions it names, for a file whose other functions are in a hidden module.
pub fn derive(module: &str, only: &[String], text: &str) -> Vec<Found> {
    let lines: Vec<&str> = text.lines().collect();
    let mut found = Vec::new();
    let mut owner: Option<String> = None;
    for (at, line) in lines.iter().enumerate() {
        if *line == "#[cfg(test)]" {
            break;
        }
        let margin = !line.starts_with(' ') && !line.starts_with('\t');
        if margin {
            if *line == "}" {
                owner = None;
            } else if line.starts_with("impl") && line.ends_with('{') {
                owner = inherent_owner(line);
            } else if let Some((name, signature)) = public_fn(&lines, at, 0) {
                found.push((format!("{module}::{name}"), name, signature));
            }
        } else if let (Some(owner), Some((name, signature))) =
            (owner.as_ref(), public_fn(&lines, at, 4))
        {
            found.push((format!("{owner}::{name}"), name, signature));
        }
    }
    found
        .into_iter()
        .filter(|(_, name, _)| only.is_empty() || only.contains(name))
        .map(|(path, _, signature)| Found { path, signature })
        .collect()
}

/// The public functions of every surface file, in order of file and line.
pub fn derive_surface(sources: &Sources, files: &Files) -> Result<Vec<Found>, String> {
    let mut out = Vec::new();
    for s in sources.source.iter().filter(|s| s.role == SURFACE) {
        let bytes = files
            .get(&s.vendored)
            .ok_or_else(|| format!("{} is not vendored", s.vendored))?;
        let text = std::str::from_utf8(bytes).map_err(|e| format!("{}: {e}", s.vendored))?;
        out.extend(derive(&s.module, &s.only, text));
    }
    Ok(out)
}

/// The map has a row for exactly the functions found, with the signatures found:
/// a function added, removed or re-signed upstream, or dropped from the map, is
/// reported.
pub fn check_surface(map: &Map, found: &[Found]) -> Vec<String> {
    let mut out = Vec::new();
    if map.spec != MAP_SPEC {
        out.push(format!("map.toml has spec {}, not {MAP_SPEC}", map.spec));
    }
    let mut mapped = BTreeMap::new();
    for f in &map.function {
        if mapped.insert(&f.path, &f.signature).is_some() {
            out.push(format!("{} is mapped twice", f.path));
        }
    }
    let mut derived = BTreeMap::new();
    for f in found {
        if derived.insert(&f.path, &f.signature).is_some() {
            out.push(format!("{} is public twice in the pinned source", f.path));
        }
    }
    for (path, signature) in &derived {
        match mapped.get(path) {
            None => out.push(format!(
                "{path} is public in the pinned source and is not in map.toml"
            )),
            Some(recorded) if recorded != signature => out.push(format!(
                "{path}: the pinned source declares `{signature}` and map.toml records `{recorded}`"
            )),
            Some(_) => {}
        }
    }
    for path in mapped.keys() {
        if !derived.contains_key(path) {
            out.push(format!(
                "{path} is in map.toml and is not public in the pinned source"
            ));
        }
    }
    out
}

/// The map is total and single: every public function reaches exactly one tool
/// or states a reason no tool does, every tool is one an issue defines, and the
/// functions that reach a tool are the in-scope ones of the model.
pub fn check_map(map: &Map, in_scope: &BTreeSet<String>) -> Vec<String> {
    let mut out = Vec::new();
    let mut tools = BTreeSet::new();
    for t in &map.tool {
        if !tools.insert(t.name.as_str()) {
            out.push(format!("tool {} is defined twice", t.name));
        }
        if !(8..=12).contains(&t.issue) {
            out.push(format!(
                "tool {} cites issue {}, which defines no tool",
                t.name, t.issue
            ));
        }
        if !t.name.starts_with("lexlean_") {
            out.push(format!(
                "tool {} does not carry the lexlean_ prefix",
                t.name
            ));
        }
    }
    for f in &map.function {
        match (&f.tool, &f.reason) {
            (Some(tool), None) => {
                if !tools.contains(tool.as_str()) {
                    out.push(format!(
                        "{} is mapped to {tool}, which no issue defines",
                        f.path
                    ));
                }
            }
            (None, Some(reason)) => {
                if reason.trim().is_empty() {
                    out.push(format!(
                        "{} is not exposed and gives an empty reason",
                        f.path
                    ));
                }
                if f.issue.is_none() {
                    out.push(format!(
                        "{} is not exposed and cites no source for its reason",
                        f.path
                    ));
                }
            }
            (Some(_), Some(_)) => {
                out.push(format!(
                    "{} is mapped to a tool and also states it is not exposed",
                    f.path
                ));
            }
            (None, None) => out.push(format!("{} is unmapped: no tool and no reason", f.path)),
        }
    }
    let exposed = map.exposed();
    for path in in_scope.difference(&exposed) {
        out.push(format!("{path} is in scope of Q2 and reaches no tool"));
    }
    for path in exposed.difference(in_scope) {
        out.push(format!(
            "{path} reaches a tool and is not in scope of Q2 in the model"
        ));
    }
    out
}

/// The in-scope functions the model of Q2 states:
/// `` `Engine::{check, snapshot}` and `sdk::{assemble}` `` gives `Engine::check`,
/// `Engine::snapshot` and `sdk::assemble`.
pub fn in_scope_of_model(quality: &str) -> Result<BTreeSet<String>, String> {
    let key = "record Description {index := 1, observation := \"";
    let at = quality
        .find(key)
        .ok_or("Quality has no description of Q2")?;
    let rest = &quality[at + key.len()..];
    let observation = &rest[..rest
        .find("\", threshold")
        .ok_or("the description of Q2 is cut")?];
    let mut out = BTreeSet::new();
    for group in observation.split('`').skip(1).step_by(2) {
        let (prefix, names) = group
            .split_once("::{")
            .ok_or_else(|| format!("`{group}` is not of the form Owner::{{names}}"))?;
        let names = names
            .strip_suffix('}')
            .ok_or("a name group is not closed")?;
        for name in names.split(',') {
            out.insert(format!("{prefix}::{}", name.trim()));
        }
    }
    if out.is_empty() {
        return Err("the description of Q2 names no function".to_string());
    }
    Ok(out)
}

/// The count of `record Cardinality {measure := <measure>, count := <n>,` in
/// the model.
pub fn cardinality_of_model(quality: &str, measure: u32) -> Option<u64> {
    let key = format!("record Cardinality {{measure := {measure}, count := ");
    let at = quality.find(&key)? + key.len();
    let digits: String = quality[at..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}

/// One row of the diagnostic registry.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    /// The code.
    pub code: String,
    /// The error class.
    pub class: String,
    /// The CLI exit status of the class.
    pub exit: u32,
    /// The meaning without its terminal period.
    pub title: String,
    /// The meaning.
    pub statement: String,
}

/// `model/errors.toml` of LexLean.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    /// The schema tag.
    pub spec: String,
    /// One row per code, sorted.
    pub error: Vec<Entry>,
}

impl Registry {
    /// Parse the registry.
    pub fn parse(text: &str) -> Result<Self, String> {
        toml::from_str(text).map_err(|e| format!("errors.toml: {e}"))
    }

    /// The codes.
    pub fn codes(&self) -> BTreeSet<String> {
        self.error.iter().map(|e| e.code.clone()).collect()
    }
}

/// The entry `lexlean explain` prints for a code: the text of `explain_text` in
/// `crates/lexlean/src/cli.rs` of the pinned commit.
pub fn explain_text(entry: &Entry) -> String {
    format!(
        "## `{}` --- {}\n\n{}\n\nClass: `{}`. Exit code: {}.\n",
        entry.code, entry.title, entry.statement, entry.class, entry.exit
    )
}

/// The exit status of an error class (SPEC 24.5).
pub fn exit_of_class(class: &str) -> Option<u32> {
    match class {
        "language" => Some(1),
        "cli-configuration" => Some(2),
        "environment" => Some(3),
        "security-limit" => Some(4),
        "internal" => Some(70),
        _ => None,
    }
}

/// The digit a code letter opens its range with (SPEC 26.2).
fn range_digit(letter: char) -> Option<char> {
    "CLPRTFBVSI"
        .find(letter)
        .and_then(|i| char::from_digit(i as u32, 10))
}

/// Whether `code` is `LL`, a range letter and four digits whose first is the
/// digit of the range.
pub fn well_formed_code(code: &str) -> bool {
    let b: Vec<char> = code.chars().collect();
    b.len() == 7
        && b[0] == 'L'
        && b[1] == 'L'
        && b[3..].iter().all(char::is_ascii_digit)
        && range_digit(b[2]) == Some(b[3])
}

/// Every code of the registry is explained by its own row: well formed, once,
/// in sorted order, in a closed class with that class's exit status, and with a
/// title and a statement that say something.
pub fn check_registry(registry: &Registry) -> Vec<String> {
    let mut out = Vec::new();
    if registry.spec != "lexlean/errors/1" {
        out.push(format!("errors.toml has spec {}", registry.spec));
    }
    if registry.error.is_empty() {
        out.push("the registry holds no code".to_string());
    }
    let mut previous: Option<&str> = None;
    for e in &registry.error {
        if !well_formed_code(&e.code) {
            out.push(format!("{} is not a well formed code", e.code));
        }
        if let Some(p) = previous {
            if p >= e.code.as_str() {
                out.push(format!(
                    "{} follows {p}: rows are sorted by code and unique",
                    e.code
                ));
            }
        }
        previous = Some(&e.code);
        match exit_of_class(&e.class) {
            None => out.push(format!("{}: unknown class {}", e.code, e.class)),
            Some(exit) if exit != e.exit => {
                out.push(format!(
                    "{}: class {} exits {exit}, not {}",
                    e.code, e.class, e.exit
                ));
            }
            Some(_) => {}
        }
        if e.title.trim().is_empty() {
            out.push(format!("{} has no title", e.code));
        }
        if e.statement.trim().is_empty() {
            out.push(format!("{} has no statement: it is not explained", e.code));
        }
    }
    out
}

/// The entries of `ERRORS.md`: `(code, text)` for each `## ` heading.
pub fn entries_of(errors_md: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in errors_md.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("## `") {
            let code: String = rest.chars().take_while(|c| *c != '`').collect();
            out.push((code, String::new()));
        }
        if let Some((_, text)) = out.last_mut() {
            text.push_str(line);
        }
    }
    for (_, text) in &mut out {
        while text.ends_with("\n\n") {
            text.pop();
        }
    }
    out
}

/// `lexlean explain` resolves every code: `ERRORS.md` holds one entry per code
/// of the registry, no other, and each entry is what `explain_text` renders.
pub fn check_explained(registry: &Registry, errors_md: &str) -> Vec<String> {
    let mut out = Vec::new();
    let entries = entries_of(errors_md);
    let mut seen = BTreeSet::new();
    for (code, text) in &entries {
        if !seen.insert(code.as_str()) {
            out.push(format!("{code} has two entries in ERRORS.md"));
        }
        match registry.error.iter().find(|e| &e.code == code) {
            None => out.push(format!(
                "ERRORS.md explains {code}, which the registry does not hold"
            )),
            Some(e) if explain_text(e) != *text => {
                out.push(format!(
                    "{code}: the entry of ERRORS.md is not what explain prints"
                ));
            }
            Some(_) => {}
        }
    }
    for e in &registry.error {
        if !seen.contains(e.code.as_str()) {
            out.push(format!(
                "{} is registered and has no entry in ERRORS.md",
                e.code
            ));
        }
    }
    out
}

/// Every code a result can carry resolves: each code in `returned` is a code of
/// the registry.
pub fn check_returned(registry: &Registry, returned: &BTreeSet<String>) -> Vec<String> {
    let codes = registry.codes();
    returned
        .difference(&codes)
        .map(|c| format!("{c} is returned and the registry does not explain it"))
        .collect()
}
