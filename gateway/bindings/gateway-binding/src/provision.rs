//! Provisioning of a profile (J5): the operator's commands, as functions.
//!
//! A profile is a LexLean project that the operator made with `lexlean init` and locked with
//! `lexlean lock`, plus the review records of its path and Git packages. `install` takes such a
//! project, refuses it with a typed `Refusal` when the compiler or this module would not serve it,
//! and otherwise writes the directory `profiles/<name>/` of the store:
//!
//! | entry | what |
//! | --- | --- |
//! | `profile` | line 1 the profile ID in hexadecimal, then one source root per line |
//! | `vocabulary` | the served vocabulary: a JSON object the lexicon tools read |
//! | `reviews/<sha256>` | a review record, named by the digest of its bytes |
//! | the closure | `lexlean.toml`, the lock, the Lake workspace files, the source roots, the path packages, the artifact sources and the Git cache |
//!
//! Nothing is written before every refusal has been decided, and the directory is renamed into
//! place from a hidden staging directory, so a refused install leaves the store as it was.
//! The language rule is the compiler's: `lock --check` refuses a package of another language with
//! `LLC0103` and a stale lock with `LLC0102`, and this module reports the code it was given.
//! The identity is `identity::profile_id`, held to the model.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use camino::Utf8Path;
use lexlean::config::LexiconSource;
use lexlean::lexicon::entry::{Channel, Denotation};
use lexlean::project::Project;
use lexlean::{Diagnostic, Engine, LexLeanError, LockRequest};
use sha2::{Digest, Sha256};

use crate::identity;
use crate::lexicon::ENTRY_MAXIMUM;
use crate::materialize::RECORD;
use crate::store;

/// The area of the store that holds the catalogue of the profiles.
pub const CATALOGUE_AREA: &str = "index";

/// The key of the catalogue in its area.
pub const CATALOGUE: &str = "profiles";

/// The entry of a profile that holds the review records.
pub const REVIEWS: &str = "reviews";

/// The entry of a profile that holds its served vocabulary.
pub const VOCABULARY: &str = "vocabulary";

/// The most bytes of a vocabulary: the bound of a `lexicon.read` of the core.
pub const VOCABULARY_LIMIT: usize = ENTRY_MAXIMUM;

/// The first line of a review record.
const REVIEW_HEAD: &str = "lexlean-mcp-review-v1";

static STAGED: AtomicU64 = AtomicU64::new(0);

/// Why a project was not provisioned. Every one has a code, and the code of a refusal of the
/// compiler is the compiler's own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// The name is not a key segment of at most 64 characters.
    NameInvalid,
    /// A profile of that name is installed.
    ProfileExists,
    /// The project or a review file cannot be read as a directory with a configuration.
    ProjectUnreadable,
    /// The compiler refused, with this diagnostic code (`LLC0102`, `LLC0103`, ...).
    Compiler(String),
    /// The project holds a symbolic link, or an entry that is neither a file nor a directory.
    LinkInBundle,
    /// The project has an entry named like one the profile directory reserves.
    ReservedEntry,
    /// The project has source packages, which a profile does not carry.
    SourcePackagesUnsupported,
    /// A path or Git package has no review record.
    ReviewMissing,
    /// A review record is not four exact lines.
    ReviewMalformed,
    /// A review record names no locked package at that digest.
    ReviewStale,
    /// Two review records are the same bytes.
    ReviewDuplicate,
    /// The vocabulary is over `VOCABULARY_LIMIT`.
    VocabularyTooLarge,
    /// The identity refused the files or the review records.
    IdentityRefused,
    /// The store cannot be written.
    StoreUnwritable,
}

impl Refusal {
    /// The code of the refusal.
    pub fn code(&self) -> String {
        match self {
            Refusal::NameInvalid => "NameInvalid".to_string(),
            Refusal::ProfileExists => "ProfileExists".to_string(),
            Refusal::ProjectUnreadable => "ProjectUnreadable".to_string(),
            Refusal::Compiler(code) => code.clone(),
            Refusal::LinkInBundle => "LinkInBundle".to_string(),
            Refusal::ReservedEntry => "ReservedEntry".to_string(),
            Refusal::SourcePackagesUnsupported => "SourcePackagesUnsupported".to_string(),
            Refusal::ReviewMissing => "ReviewMissing".to_string(),
            Refusal::ReviewMalformed => "ReviewMalformed".to_string(),
            Refusal::ReviewStale => "ReviewStale".to_string(),
            Refusal::ReviewDuplicate => "ReviewDuplicate".to_string(),
            Refusal::VocabularyTooLarge => "VocabularyTooLarge".to_string(),
            Refusal::IdentityRefused => "IdentityRefused".to_string(),
            Refusal::StoreUnwritable => "StoreUnwritable".to_string(),
        }
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "refused {}", self.code())
    }
}

impl std::error::Error for Refusal {}

impl From<identity::Refused> for Refusal {
    fn from(_: identity::Refused) -> Refusal {
        Refusal::IdentityRefused
    }
}

fn compiler(diagnostics: &[Diagnostic]) -> Refusal {
    Refusal::Compiler(
        diagnostics
            .first()
            .map_or("LLC0001", |diagnostic| diagnostic.code.as_str())
            .to_string(),
    )
}

fn compiler_error(error: &LexLeanError) -> Refusal {
    compiler(&error.diagnostics)
}

/// Whether a package of this lock kind needs a review record: every kind but `builtin`, so a kind
/// the host does not know needs one too.
pub fn needs_review(kind: &str) -> bool {
    kind != "builtin"
}

/// The toolchain of the profile identity, from the header of a lock: the digest of the compiler's
/// semantics and the Lean toolchain, as the lines `compiler_semantics <hex>` and
/// `lean_toolchain <name>`. `None` when the lock has no such header.
pub fn toolchain(lock: &[u8]) -> Option<Vec<u8>> {
    let text = std::str::from_utf8(lock).ok()?;
    let mut compiler_semantics = None;
    let mut lean_toolchain = None;
    for line in text.lines() {
        if line.starts_with('[') {
            break;
        }
        let Some((key, value)) = line.split_once(" = ") else {
            continue;
        };
        let value = value.strip_prefix('"')?.strip_suffix('"')?;
        match key {
            "compiler_semantics" => compiler_semantics = Some(value),
            "lean_toolchain" => lean_toolchain = Some(value),
            _ => {}
        }
    }
    let digest = compiler_semantics?;
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
    {
        return None;
    }
    Some(
        format!(
            "compiler_semantics {digest}\nlean_toolchain {}\n",
            lean_toolchain?
        )
        .into_bytes(),
    )
}

/// Whether `path` is under `root` as `Profiles.underRoot` says: the root, one `/` and more, so
/// whole components (`srcs/Main.lex.tex` is not under `src`).
fn under_root(root: &str, path: &str) -> bool {
    path.strip_prefix(root)
        .is_some_and(|rest| rest.len() > 1 && rest.starts_with('/'))
}

fn utf8(path: &Path) -> Result<&Utf8Path, Refusal> {
    Utf8Path::from_path(path).ok_or(Refusal::ProjectUnreadable)
}

/// The first diagnostic code in the text a command printed.
fn printed_code(text: &str) -> String {
    let bytes = text.as_bytes();
    for start in 0..bytes.len().saturating_sub(6) {
        let window = &bytes[start..start + 7];
        if window.starts_with(b"LL")
            && window[2].is_ascii_uppercase()
            && window[3..].iter().all(u8::is_ascii_digit)
        {
            return String::from_utf8_lossy(window).into_owned();
        }
    }
    "LLC0001".to_string()
}

/// `lexlean init`: a new project in `destination`, which is absent or empty.
pub fn init(
    destination: &Path,
    name: &str,
    module_prefix: &str,
    language: &str,
) -> Result<(), Refusal> {
    let working = std::env::current_dir().map_err(|_| Refusal::ProjectUnreadable)?;
    let working = Utf8Path::from_path(&working).ok_or(Refusal::ProjectUnreadable)?;
    let arguments: Vec<String> = [
        "lexlean",
        "init",
        utf8(destination)?.as_str(),
        "--name",
        name,
        "--module-prefix",
        module_prefix,
        "--language",
        language,
    ]
    .iter()
    .map(|argument| (*argument).to_string())
    .collect();
    let (mut out, mut err) = (Vec::new(), Vec::new());
    match lexlean::cli::run(&arguments, working, &mut out, &mut err) {
        0 => Ok(()),
        _ => Err(Refusal::Compiler(printed_code(&String::from_utf8_lossy(
            &err,
        )))),
    }
}

fn engine(project: &Path) -> Result<Engine, Refusal> {
    let file = project.join("lexlean.toml");
    Engine::load(utf8(&file)?).map_err(|error| compiler_error(&error))
}

/// `lexlean lock`: write the lock of the project; `allow_network` permits the acquisition of a
/// missing exact Git commit and nothing else.
pub fn lock(project: &Path, allow_network: bool) -> Result<(), Refusal> {
    engine(project)?
        .lock(LockRequest {
            check_only: false,
            allow_network,
        })
        .map(|_| ())
        .map_err(|error| compiler_error(&error))
}

/// `lexlean lock --check`: the lock is exactly the lock of the project, or the compiler refuses.
fn check_lock(project: &Path) -> Result<(), Refusal> {
    engine(project)?
        .lock(LockRequest {
            check_only: true,
            allow_network: false,
        })
        .map(|_| ())
        .map_err(|error| compiler_error(&error))
}

/// A review record: the package and version it names, the tree digest it vouches for and the
/// reviewer.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Review {
    package: String,
    version: String,
    digest: String,
}

/// The four lines of a review record, exactly:
///
/// ```text
/// lexlean-mcp-review-v1
/// package <id>@<version>
/// tree_sha256 <64 lower-case hexadecimal digits>
/// reviewer <name>
/// ```
///
/// each ended by a line feed, with nothing else.
fn parse_review(bytes: &[u8]) -> Result<Review, Refusal> {
    let text = std::str::from_utf8(bytes).map_err(|_| Refusal::ReviewMalformed)?;
    let body = text.strip_suffix('\n').ok_or(Refusal::ReviewMalformed)?;
    let lines: Vec<&str> = body.split('\n').collect();
    let [head, package, digest, reviewer] = lines[..] else {
        return Err(Refusal::ReviewMalformed);
    };
    if head != REVIEW_HEAD || text.contains('\r') {
        return Err(Refusal::ReviewMalformed);
    }
    let named = package
        .strip_prefix("package ")
        .and_then(|rest| rest.split_once('@'))
        .filter(|(id, version)| {
            !id.is_empty()
                && !version.is_empty()
                && !id.contains(char::is_whitespace)
                && !version.contains(char::is_whitespace)
        })
        .ok_or(Refusal::ReviewMalformed)?;
    let digest = digest
        .strip_prefix("tree_sha256 ")
        .filter(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        })
        .ok_or(Refusal::ReviewMalformed)?;
    if reviewer
        .strip_prefix("reviewer ")
        .is_none_or(|name| name.trim().is_empty())
    {
        return Err(Refusal::ReviewMalformed);
    }
    Ok(Review {
        package: named.0.to_string(),
        version: named.1.to_string(),
        digest: digest.to_string(),
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The regular files below `base/relative`, as paths relative to `base`; a link, or an entry that
/// is neither a file nor a directory, is refused, and so is a name that is not UTF-8. A path for
/// which `skip` is true is not entered.
fn walk(
    base: &Path,
    relative: &str,
    skip: &dyn Fn(&str) -> bool,
    out: &mut Vec<String>,
) -> Result<(), Refusal> {
    let directory = if relative.is_empty() {
        base.to_path_buf()
    } else {
        base.join(relative)
    };
    let mut entries: Vec<_> = fs::read_dir(&directory)
        .map_err(|_| Refusal::ProjectUnreadable)?
        .collect::<Result<_, _>>()
        .map_err(|_| Refusal::ProjectUnreadable)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    for entry in entries {
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| Refusal::ProjectUnreadable)?;
        let path = if relative.is_empty() {
            name
        } else {
            format!("{relative}/{name}")
        };
        let kind = fs::symlink_metadata(entry.path())
            .map_err(|_| Refusal::ProjectUnreadable)?
            .file_type();
        if skip(&path) {
            if kind.is_symlink() {
                return Err(Refusal::LinkInBundle);
            }
            continue;
        }
        if kind.is_dir() {
            walk(base, &path, skip, out)?;
        } else if kind.is_file() {
            out.push(path);
        } else {
            return Err(Refusal::LinkInBundle);
        }
    }
    Ok(())
}

/// The file at `base/relative` as bytes, never through a link.
fn read_file(base: &Path, relative: &str) -> Result<Vec<u8>, Refusal> {
    let path = base.join(relative);
    let kind = fs::symlink_metadata(&path)
        .map_err(|_| Refusal::ProjectUnreadable)?
        .file_type();
    if kind.is_symlink() {
        return Err(Refusal::LinkInBundle);
    }
    let mut file = store::open_for_reading(&path).map_err(|_| Refusal::ProjectUnreadable)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| Refusal::ProjectUnreadable)?;
    Ok(bytes)
}

fn config_of(project: &Path) -> Result<lexlean::config::ProjectConfig, Refusal> {
    let directory = fs::metadata(project).map_err(|_| Refusal::ProjectUnreadable)?;
    if !directory.is_dir() {
        return Err(Refusal::ProjectUnreadable);
    }
    let bytes = read_file(project, "lexlean.toml")?;
    lexlean::config::parse_project("lexlean.toml", &bytes)
        .map_err(|diagnostics| compiler(&diagnostics))
}

/// The files that are the identity of a profile, by project-relative path: the configuration, the
/// lock, every file of a path package and every artifact source. A file under a source root is not
/// one: sources are what a draft changes.
fn identity_paths(
    project: &Path,
    config: &lexlean::config::ProjectConfig,
) -> Result<BTreeSet<String>, Refusal> {
    let mut paths = BTreeSet::new();
    paths.insert("lexlean.toml".to_string());
    paths.insert(config.lockfile.clone());
    for source in &config.lexicon_sources {
        if let LexiconSource::Path { path, .. } = source {
            let mut files = Vec::new();
            walk(project, path, &|_| false, &mut files)?;
            paths.extend(files);
        }
    }
    for artifact in &config.artifact_sources {
        paths.insert(artifact.path.clone());
    }
    Ok(paths)
}

/// What `identify` found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identified {
    /// The profile ID.
    pub id: [u8; 32],
    /// The preimage the ID is the SHA-256 of.
    pub preimage: Vec<u8>,
    /// The project-relative paths of the profile files, in order.
    pub files: Vec<String>,
}

/// The identity of the project in `project` with the review records `reviews` (their bytes).
pub fn identify(project: &Path, reviews: &[Vec<u8>]) -> Result<Identified, Refusal> {
    let config = config_of(project)?;
    let lock_bytes = read_file(project, &config.lockfile)?;
    let toolchain = toolchain(&lock_bytes).ok_or(Refusal::ProjectUnreadable)?;
    let paths = identity_paths(project, &config)?;
    let mut rows = Vec::new();
    for path in &paths {
        rows.push((path.as_bytes().to_vec(), read_file(project, path)?));
    }
    let preimage = identity::preimage(&rows, reviews, &toolchain)?;
    Ok(Identified {
        id: Sha256::digest(&preimage).into(),
        preimage,
        files: paths.into_iter().collect(),
    })
}

/// A package of an installed profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// The package ID.
    pub package: String,
    /// The version.
    pub version: String,
    /// `builtin`, `path` or `git`.
    pub kind: String,
    /// The entries the profile serves.
    pub entries: usize,
}

/// What `install` made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Installed {
    /// The name of the profile.
    pub name: String,
    /// The profile ID.
    pub id: [u8; 32],
    /// The language of the profile.
    pub language: String,
    /// The source roots.
    pub roots: Vec<String>,
    /// The locked packages, by ID and version.
    pub packages: Vec<Package>,
}

fn channel(channel: Channel) -> &'static str {
    match channel {
        Channel::Text => "text",
        Channel::Math => "math",
        Channel::Both => "both",
    }
}

fn denotation(denotation: &Denotation) -> serde_json::Value {
    match denotation {
        Denotation::Core { constructor } => {
            serde_json::json!({"kind": "core", "constructor": constructor})
        }
        Denotation::Lean { module, name } => {
            serde_json::json!({"kind": "lean", "module": module, "name": name})
        }
        Denotation::Document { module, component } => {
            serde_json::json!({"kind": "document", "module": module, "component": component})
        }
        Denotation::Defined { text, .. } => serde_json::json!({"kind": "defined", "text": text}),
    }
}

/// The vocabulary of a locked project: every package of the lock, and every entry the project
/// serves. An entry is served when what it denotes is there: a document denotation needs its
/// module among the modules of the source roots, and every other denotation is the compiler's.
fn vocabulary(
    name: &str,
    project: &Path,
) -> Result<
    (
        serde_json::Value,
        Vec<Package>,
        Vec<lexlean::lock::LockPackage>,
    ),
    Refusal,
> {
    let file = project.join("lexlean.toml");
    let loaded = Project::load(utf8(&file)?).map_err(|error| compiler_error(&error))?;
    let resolved = lexlean::lock::resolve_packages(&loaded, false)
        .map_err(|diagnostics| compiler(&diagnostics))?;
    let modules = loaded
        .all_modules()
        .map_err(|diagnostics| compiler(&diagnostics))?;
    let language = loaded.config.language.clone();
    let mut kinds: BTreeMap<(String, String), String> = BTreeMap::new();
    for row in &resolved.rows {
        kinds.insert((row.id.clone(), row.version.clone()), row.kind.clone());
    }
    let mut packages: Vec<_> = resolved.packages.iter().collect();
    packages.sort_by(|a, b| (&a.id, &a.version).cmp(&(&b.id, &b.version)));
    let mut summary = Vec::new();
    let mut listed = Vec::new();
    let mut entries = Vec::new();
    for package in packages {
        let mut served = 0usize;
        for entry in package.entries.values() {
            if let Denotation::Document { module, .. } = &entry.denotation {
                if !modules.contains_key(module) {
                    continue;
                }
            }
            served += 1;
            let forms: Vec<serde_json::Value> = entry
                .forms
                .iter()
                .map(|form| {
                    serde_json::json!({
                        "id": form.id,
                        "channel": channel(form.channel),
                        "surface": form.surface,
                        "canonical": form.canonical_source,
                        "features": form.features,
                    })
                })
                .collect();
            entries.push(serde_json::json!({
                "package": package.id,
                "version": package.version,
                "id": entry.id,
                "category": entry.category.as_str(),
                "frame": entry.frame.as_str(),
                "arity": entry.surface_arity,
                "denotes": denotation(&entry.denotation),
                "forms": forms,
            }));
        }
        listed.push(serde_json::json!({
            "package": package.id,
            "version": package.version,
            "language": language,
            "entries": served,
        }));
        summary.push(Package {
            package: package.id.clone(),
            version: package.version.clone(),
            kind: kinds
                .get(&(package.id.clone(), package.version.clone()))
                .cloned()
                .unwrap_or_default(),
            entries: served,
        });
    }
    let document = serde_json::json!({
        "profile": name,
        "language": language,
        "packages": listed,
        "entries": entries,
    });
    Ok((document, summary, resolved.rows))
}

/// Match the review records to the path and Git packages of the lock: every record names a locked
/// package at its digest, and every such package has one.
fn match_reviews(reviews: &[Review], rows: &[lexlean::lock::LockPackage]) -> Result<(), Refusal> {
    let reviewed: BTreeSet<(&str, &str, String)> = rows
        .iter()
        .filter(|row| needs_review(&row.kind))
        .map(|row| {
            (
                row.id.as_str(),
                row.version.as_str(),
                row.tree_sha256.to_hex(),
            )
        })
        .collect();
    if reviews.iter().any(|review| {
        !reviewed.contains(&(
            review.package.as_str(),
            review.version.as_str(),
            review.digest.clone(),
        ))
    }) {
        return Err(Refusal::ReviewStale);
    }
    for (id, version, digest) in &reviewed {
        let found = reviews.iter().any(|review| {
            review.package == *id && review.version == *version && review.digest == *digest
        });
        if !found {
            return Err(Refusal::ReviewMissing);
        }
    }
    Ok(())
}

fn unwritable<T, E>(result: Result<T, E>) -> Result<T, Refusal> {
    result.map_err(|_| Refusal::StoreUnwritable)
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), Refusal> {
    let mut file = unwritable(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path),
    )?;
    unwritable(file.write_all(bytes))?;
    unwritable(file.sync_all())
}

fn copy_into(project: &Path, stage: &Path, relative: &str) -> Result<(), Refusal> {
    let target = stage.join(relative);
    if let Some(parent) = target.parent() {
        unwritable(fs::create_dir_all(parent))?;
    }
    let mut input =
        store::open_for_reading(&project.join(relative)).map_err(|_| Refusal::LinkInBundle)?;
    if !input
        .metadata()
        .map_err(|_| Refusal::ProjectUnreadable)?
        .is_file()
    {
        return Err(Refusal::LinkInBundle);
    }
    let mut bytes = Vec::new();
    input
        .read_to_end(&mut bytes)
        .map_err(|_| Refusal::ProjectUnreadable)?;
    write_file(&target, &bytes)
}

/// The catalogue of the profiles of the store: the profiles, by name, each with its ID, language,
/// source roots and packages. It is derived from the profile directories and rewritten whenever
/// one is installed.
fn write_catalogue(root: &Path) -> Result<(), Refusal> {
    let profiles = root.join("profiles");
    let mut names: Vec<String> = unwritable(fs::read_dir(&profiles))?
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| store::name_is_valid(name))
        .collect();
    names.sort();
    let mut listed = Vec::new();
    for name in names {
        let directory = profiles.join(&name);
        let (Ok(record), Ok(vocabulary)) = (
            fs::read_to_string(directory.join(RECORD)),
            fs::read(directory.join(VOCABULARY)),
        ) else {
            continue;
        };
        let Ok(vocabulary) = serde_json::from_slice::<serde_json::Value>(&vocabulary) else {
            continue;
        };
        let mut lines = record.lines();
        let Some(id) = lines.next() else { continue };
        let roots: Vec<&str> = lines.collect();
        listed.push(serde_json::json!({
            "name": name,
            "id": id,
            "language": vocabulary["language"],
            "roots": roots,
            "packages": vocabulary["packages"],
        }));
    }
    let bytes = unwritable(serde_json::to_vec(&serde_json::json!({"profiles": listed})))?;
    let index = root.join(CATALOGUE_AREA);
    unwritable(fs::create_dir_all(&index))?;
    let temporary = index.join(format!(
        ".{CATALOGUE}-{}-{}",
        std::process::id(),
        STAGED.fetch_add(1, Ordering::Relaxed)
    ));
    write_file(&temporary, &bytes)?;
    if fs::rename(&temporary, index.join(CATALOGUE)).is_err() {
        let _ = fs::remove_file(&temporary);
        return Err(Refusal::StoreUnwritable);
    }
    Ok(())
}

/// Install the project in `project` as the profile `name` of the store at `root`, with the
/// review records in the files `reviews`. The project must be locked: the lock is checked, never
/// written.
pub fn install(
    root: &Path,
    name: &str,
    project: &Path,
    reviews: &[&Path],
) -> Result<Installed, Refusal> {
    if !store::name_is_valid(name) {
        return Err(Refusal::NameInvalid);
    }
    let profiles = root.join("profiles");
    if fs::symlink_metadata(profiles.join(name)).is_ok() {
        return Err(Refusal::ProfileExists);
    }
    let config = config_of(project)?;
    if !config.source_packages.is_empty() {
        return Err(Refusal::SourcePackagesUnsupported);
    }
    for reserved in [RECORD, VOCABULARY, REVIEWS] {
        if fs::symlink_metadata(project.join(reserved)).is_ok() {
            return Err(Refusal::ReservedEntry);
        }
    }
    let build_root = config.build_root.clone();
    let mut files = Vec::new();
    walk(
        project,
        "",
        &|path| path == ".git" || path == ".lake" || *path == build_root,
        &mut files,
    )?;
    let cache = format!("{build_root}/cache/git");
    if fs::symlink_metadata(project.join(&cache)).is_ok() {
        walk(project, &cache, &|_| false, &mut files)?;
    }
    check_lock(project)?;
    let mut records = Vec::new();
    let mut parsed = Vec::new();
    for path in reviews {
        let bytes = fs::read(path).map_err(|_| Refusal::ProjectUnreadable)?;
        parsed.push(parse_review(&bytes)?);
        records.push(bytes);
    }
    let mut distinct = BTreeSet::new();
    if !records.iter().all(|bytes| distinct.insert(bytes.clone())) {
        return Err(Refusal::ReviewDuplicate);
    }
    let (document, packages, rows) = vocabulary(name, project)?;
    match_reviews(&parsed, &rows)?;
    let identified = identify(project, &records)?;
    let lock_bytes = read_file(project, &config.lockfile)?;
    let vocabulary_bytes = unwritable(serde_json::to_vec(&document))?;
    if vocabulary_bytes.len() > VOCABULARY_LIMIT {
        return Err(Refusal::VocabularyTooLarge);
    }
    let lock = lexlean::api::parse_lock_bytes(&config.lockfile, &lock_bytes)
        .map_err(|diagnostics| compiler(&diagnostics))?;
    let in_closure = |path: &str| {
        path == "lexlean.toml"
            || path == config.lockfile
            || lock.workspace_files.iter().any(|(file, _)| file == path)
            || config.source_roots.iter().any(|root| under_root(root, path))
            || config.lexicon_sources.iter().any(
                |source| matches!(source, LexiconSource::Path { path: directory, .. } if under_root(directory, path)),
            )
            || config.artifact_sources.iter().any(|artifact| artifact.path == path)
            || under_root(&cache, path)
    };
    let closure: Vec<&String> = files.iter().filter(|path| in_closure(path)).collect();
    unwritable(fs::create_dir_all(&profiles))?;
    let stage = profiles.join(format!(
        ".stage-{}-{}",
        std::process::id(),
        STAGED.fetch_add(1, Ordering::Relaxed)
    ));
    unwritable(fs::create_dir(&stage))?;
    let staged = (|| -> Result<(), Refusal> {
        for path in &closure {
            copy_into(project, &stage, path)?;
        }
        write_file(&stage.join(VOCABULARY), &vocabulary_bytes)?;
        unwritable(fs::create_dir(stage.join(REVIEWS)))?;
        for bytes in &records {
            write_file(
                &stage.join(REVIEWS).join(hex(&Sha256::digest(bytes))),
                bytes,
            )?;
        }
        let mut record = format!("{}\n", hex(&identified.id));
        for source_root in &config.source_roots {
            record.push_str(source_root);
            record.push('\n');
        }
        write_file(&stage.join(RECORD), record.as_bytes())
    })();
    if let Err(refused) = staged {
        let _ = fs::remove_dir_all(&stage);
        return Err(refused);
    }
    let target: PathBuf = profiles.join(name);
    if fs::rename(&stage, &target).is_err() {
        let _ = fs::remove_dir_all(&stage);
        return Err(if fs::symlink_metadata(&target).is_ok() {
            Refusal::ProfileExists
        } else {
            Refusal::StoreUnwritable
        });
    }
    if let Err(refused) = write_catalogue(root) {
        let _ = fs::remove_dir_all(&target);
        return Err(refused);
    }
    Ok(Installed {
        name: name.to_string(),
        id: identified.id,
        language: config.language,
        roots: config.source_roots,
        packages,
    })
}
