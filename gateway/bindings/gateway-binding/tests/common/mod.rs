//! What the tests of the verifier process share: the program in its two forms (the real one and a
//! script that stands in for it), a configuration with a key, and the calls of the verify effects.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use gateway_binding::{Config, Native, State};
use gateway_host_api::protocol::{verify_control, verify_start, Host, HostError};

/// The fixture project.
pub const TINY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors/fixture/tiny");

/// The seed of the key the tests sign with.
pub const SEED: [u8; 32] = [0x55; 32];

/// Lowercase hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The program that Cargo built beside the tests.
pub fn real_verifier() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_gateway-verifier"))
}

/// The script that stands in for the program (`fake-verifier.sh`), written into `dir` with the
/// mode it needs: the mode is set here and not kept in the repository, where a checkout may drop it.
pub fn fake_verifier(dir: &Path) -> PathBuf {
    let path = dir.join("fake-verifier");
    std::fs::write(&path, include_str!("fake-verifier.sh")).expect("a script");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("a mode");
    path
}

/// A configuration whose store is below `dir`, whose key is a file of `dir` and whose verifier is
/// `program`.
pub fn config(dir: &Path, program: PathBuf) -> Config {
    let key = dir.join("signing.key");
    std::fs::write(&key, format!("{}\n", hex(&SEED))).expect("a key file");
    Config::at(dir.join("store"))
        .with_signing_key(key)
        .with_verifier(program)
}

/// Copy a directory tree, leaving out the build root of LexLean.
pub fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from)
        .expect("a fixture directory")
        .flatten()
    {
        let name = entry.file_name();
        if name == ".lexlean" {
            continue;
        }
        if entry.file_type().expect("a type").is_dir() {
            copy(&entry.path(), &to.join(&name));
        } else {
            std::fs::copy(entry.path(), to.join(&name)).expect("a copy");
        }
    }
}

/// The fixture project, installed as the draft project `tiny`, with `behavior` for the fake.
pub fn install_draft(host: &Native, behavior: Option<&str>) {
    let draft = host.config().store.join("drafts").join("tiny");
    copy(Path::new(TINY), &draft);
    if let Some(behavior) = behavior {
        set_behavior(host, behavior);
    }
}

/// What the fake does for the next task started: it is copied into the task with the project.
pub fn set_behavior(host: &Native, behavior: &str) {
    std::fs::write(
        host.config()
            .store
            .join("drafts")
            .join("tiny")
            .join("fake-behavior"),
        behavior,
    )
    .expect("a behavior");
}

/// The fixture project, installed as the draft project `name`, with `behavior` for the fake. A
/// project that is named is used where it lies, so the tasks of one project share a directory and
/// a behavior: a test that runs tasks beside each other gives each its own project.
pub fn install_named(host: &Native, name: &str, behavior: &str) {
    let draft = host.config().store.join("drafts").join(name);
    copy(Path::new(TINY), &draft);
    std::fs::write(draft.join("fake-behavior"), behavior).expect("a behavior");
}

/// `verify.start` on the draft project `tiny`.
pub fn start(host: &mut Native) -> Result<(String, u32), verify_start::Failure> {
    start_named(host, "tiny")
}

/// `verify.start` on the draft project `name`.
pub fn start_named(host: &mut Native, name: &str) -> Result<(String, u32), verify_start::Failure> {
    host.verify_start(verify_start::Request {
        project: name.into(),
        selection: 0,
        files: String::new(),
    })
    .map(|done| (done.task, done.state))
    .map_err(failure)
}

/// The state of a task.
pub fn status(
    host: &mut Native,
    task: &str,
) -> Result<verify_control::Completed, verify_control::Failure> {
    host.verify_control(verify_control::Request {
        task: task.into(),
        action: 0,
    })
    .map_err(failure)
}

/// Cancel a task.
pub fn cancel(
    host: &mut Native,
    task: &str,
) -> Result<verify_control::Completed, verify_control::Failure> {
    host.verify_control(verify_control::Request {
        task: task.into(),
        action: 1,
    })
    .map_err(failure)
}

/// The failure of an effect; a fault of the host is a bug of the test.
pub fn failure<F>(error: HostError<F>) -> F {
    match error {
        HostError::Failed(failure) => failure,
        HostError::Fault => panic!("the host faulted"),
    }
}

/// Wait until the task is in `state`.
pub fn wait_for(host: &mut Native, task: &str, state: State) -> verify_control::Completed {
    let until = Instant::now() + Duration::from_secs(60);
    loop {
        let report = status(host, task).expect("a known task");
        if report.state == state as u32 {
            return report;
        }
        assert!(
            Instant::now() < until,
            "{task} did not reach {state:?}: it is {}",
            report.state
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

/// Every file below `directory`, relative to it; empty when it does not exist.
pub fn files_below(directory: &Path) -> Vec<String> {
    fn walk(base: &Path, at: &Path, found: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(at) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, found);
            } else {
                found.push(
                    path.strip_prefix(base)
                        .expect("below")
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }
    let mut found = Vec::new();
    walk(directory, directory, &mut found);
    found.sort();
    found
}

/// The files named `name` below the drafts area of the store: one for each task that got as far as
/// writing one.
pub fn found_in_drafts(host: &Native, name: &str) -> Vec<PathBuf> {
    let drafts = host.config().store.join("drafts");
    files_below(&drafts)
        .into_iter()
        .filter(|file| file.rsplit('/').next() == Some(name))
        .map(|file| drafts.join(file))
        .collect()
}

/// Whether the process is running: it is in `/proc` and is not a zombie.
pub fn alive(pid: u32) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|stat| {
            stat.rsplit_once(')')
                .and_then(|(_, rest)| rest.split_whitespace().next().map(|state| state != "Z"))
        })
        .unwrap_or(false)
}

/// Wait until the process is gone, for at most `seconds`.
pub fn gone_within(pid: u32, seconds: u64) -> bool {
    let until = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < until {
        if !alive(pid) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    !alive(pid)
}

/// The processes that are running in the session `session`, other than this one.
pub fn members_of_session(session: u32) -> Vec<u32> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return found;
    };
    for entry in entries.flatten() {
        let Some(pid) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<u32>().ok())
        else {
            continue;
        };
        let Ok(stat) = std::fs::read_to_string(entry.path().join("stat")) else {
            continue;
        };
        let fields: Vec<&str> = stat
            .rsplit_once(')')
            .map_or("", |(_, rest)| rest)
            .split_whitespace()
            .collect();
        // state, parent, group, session
        if fields.get(3).and_then(|field| field.parse::<u32>().ok()) == Some(session)
            && fields.first() != Some(&"Z")
            && pid != std::process::id()
        {
            found.push(pid);
        }
    }
    found
}

/// The number written in a file of the fake, such as `pid` or `orphan.pid`.
pub fn number_in(file: &Path) -> u32 {
    std::fs::read_to_string(file)
        .expect("a file")
        .trim()
        .parse()
        .expect("a number")
}
