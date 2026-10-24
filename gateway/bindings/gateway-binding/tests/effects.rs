//! The ten effects of the core, and the frame effect, each against a store in a temporary
//! directory: its success, and each failure it declares.

use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gateway_binding::{Config, Job, Native, Outcome, State};
use gateway_host_api::protocol::{
    attestation_lookup, engine_build, engine_check, engine_format, engine_snapshot, lexicon_read,
    rate_calls, store_get, store_put, verify_control, verify_start, Host, HostError,
};

const TINY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors/fixture/tiny");

fn host() -> (Native, tempfile::TempDir) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let host = Native::new(Config::at(directory.path().join("store")));
    (host, directory)
}

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a fixture directory").flatten() {
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

/// The fixture project, installed as the draft project `tiny`.
fn with_tiny() -> (Native, tempfile::TempDir) {
    let (host, directory) = host();
    copy(
        Path::new(TINY),
        &host.config().store.join("drafts").join("tiny"),
    );
    (host, directory)
}

fn put(host: &mut Native, key: &str, content: &[u8]) -> Result<u64, store_put::Failure> {
    host.store_put(store_put::Request {
        area: 1,
        key: key.to_string(),
        content: content.to_vec(),
    })
    .map(|done| done.length)
    .map_err(failure)
}

fn failure<F>(error: HostError<F>) -> F {
    match error {
        HostError::Failed(failure) => failure,
        HostError::Fault => panic!("the host faulted"),
    }
}

fn check(host: &mut Native, project: &str, selection: u32, files: &str) -> Result<(u32, String), engine_check::Failure> {
    host.engine_check(engine_check::Request {
        project: project.to_string(),
        selection,
        files: files.to_string(),
    })
    .map(|done| (done.class, String::from_utf8_lossy(&done.result).into_owned()))
    .map_err(failure)
}

// ---- store.get ------------------------------------------------------------------------

#[test]
fn store_get_returns_what_store_put_wrote() {
    let (mut host, _directory) = host();
    assert_eq!(put(&mut host, "a/b.txt", b"hello"), Ok(5));
    let done = host
        .store_get(store_get::Request {
            area: 1,
            key: "a/b.txt".into(),
        })
        .expect("read");
    assert!(done.found);
    assert_eq!(done.content, b"hello");
}

#[test]
fn store_get_reports_a_missing_key_as_not_found() {
    let (mut host, _directory) = host();
    for key in ["nothing", "no/such/dir/file"] {
        let done = host
            .store_get(store_get::Request {
                area: 1,
                key: key.into(),
            })
            .expect("read");
        assert!(!done.found, "{key}");
        assert!(done.content.is_empty());
    }
}

#[test]
fn store_get_refuses_an_area_that_is_not_one_of_the_seven() {
    let (mut host, _directory) = host();
    for area in [7, 8, u32::MAX] {
        let error = host
            .store_get(store_get::Request {
                area,
                key: "k".into(),
            })
            .map(|_| ())
            .map_err(failure);
        assert_eq!(error, Err(store_get::Failure::AreaUnknown), "area {area}");
    }
}

#[test]
fn store_get_refuses_a_key_that_is_not_a_key() {
    let (mut host, _directory) = host();
    let long = "a".repeat(256);
    for key in [
        "", "..", "../x", "a/../b", "/abs", "a//b", ".hidden", "a/.b", "a b", "a\\b", "ü", "a/", &long,
    ] {
        let error = host
            .store_get(store_get::Request {
                area: 1,
                key: key.into(),
            })
            .map(|_| ())
            .map_err(failure);
        assert_eq!(error, Err(store_get::Failure::KeyRefused), "{key:?}");
    }
}

#[test]
fn store_get_fails_when_the_key_names_a_directory() {
    let (mut host, _directory) = host();
    put(&mut host, "d/file", b"x").expect("put");
    let error = host
        .store_get(store_get::Request {
            area: 1,
            key: "d".into(),
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(store_get::Failure::Unreadable));
}

#[cfg(unix)]
#[test]
fn store_get_fails_when_the_path_crosses_a_link() {
    let (mut host, directory) = host();
    let outside = directory.path().join("outside");
    std::fs::create_dir_all(&outside).expect("a directory");
    std::fs::write(outside.join("secret"), b"secret").expect("a file");
    let drafts = host.config().store.join("drafts");
    std::fs::create_dir_all(&drafts).expect("a directory");
    std::os::unix::fs::symlink(&outside, drafts.join("link")).expect("a link");
    std::os::unix::fs::symlink(outside.join("secret"), drafts.join("file")).expect("a link");
    for key in ["link/secret", "file"] {
        let error = host
            .store_get(store_get::Request {
                area: 1,
                key: key.into(),
            })
            .map(|_| ())
            .map_err(failure);
        assert_eq!(error, Err(store_get::Failure::Unreadable), "{key}");
    }
}

#[test]
fn store_get_fails_when_the_file_is_over_the_draft_limit() {
    let directory = tempfile::tempdir().expect("a directory");
    let mut config = Config::at(directory.path().join("store"));
    config.draft_max_bytes = 4;
    let mut host = Native::new(config);
    std::fs::create_dir_all(host.config().store.join("drafts")).expect("a directory");
    std::fs::write(host.config().store.join("drafts").join("big"), b"12345").expect("a file");
    let error = host
        .store_get(store_get::Request {
            area: 1,
            key: "big".into(),
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(store_get::Failure::Unreadable));
}

// ---- store.put ------------------------------------------------------------------------

#[test]
fn store_put_writes_the_file_and_returns_its_length() {
    let (mut host, _directory) = host();
    assert_eq!(put(&mut host, "x/y/z.txt", b"twelve bytes"), Ok(12));
    let written = host.config().store.join("drafts/x/y/z.txt");
    assert_eq!(std::fs::read(written).expect("the file"), b"twelve bytes");
    assert_eq!(put(&mut host, "empty", b""), Ok(0));
}

#[test]
fn store_put_replaces_a_file_whole_and_leaves_no_temporary() {
    let (mut host, _directory) = host();
    put(&mut host, "f", b"a long first content").expect("put");
    put(&mut host, "f", b"short").expect("put");
    let drafts = host.config().store.join("drafts");
    assert_eq!(std::fs::read(drafts.join("f")).expect("the file"), b"short");
    let names: Vec<_> = std::fs::read_dir(&drafts)
        .expect("a directory")
        .flatten()
        .map(|entry| entry.file_name())
        .collect();
    assert_eq!(names, ["f"], "no temporary file is left");
}

#[test]
fn store_put_writes_every_one_of_the_seven_areas() {
    let (mut host, _directory) = host();
    for (area, name) in [
        "profiles",
        "drafts",
        "builds",
        "verifications",
        "index",
        "exports",
        "keys",
    ]
    .iter()
    .enumerate()
    {
        let area = u32::try_from(area).expect("a number");
        host.store_put(store_put::Request {
            area,
            key: "k".into(),
            content: vec![1, 2, 3],
        })
        .expect("put");
        assert!(host.config().store.join(name).join("k").is_file(), "{name}");
    }
}

#[test]
fn store_put_refuses_an_area_that_is_not_one_of_the_seven() {
    let (mut host, _directory) = host();
    let error = host
        .store_put(store_put::Request {
            area: 7,
            key: "k".into(),
            content: vec![],
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(store_put::Failure::AreaUnknown));
}

#[test]
fn store_put_refuses_a_key_that_is_not_a_key() {
    let (mut host, _directory) = host();
    for key in ["", "..", "../x", "a/../b", "/abs", "a//b", ".hidden", "a b"] {
        assert_eq!(put(&mut host, key, b"x"), Err(store_put::Failure::KeyRefused), "{key:?}");
    }
    assert!(!host.config().store.join("x").exists());
}

#[test]
fn store_put_fails_when_the_key_names_a_directory_or_a_file_is_in_the_way() {
    let (mut host, _directory) = host();
    put(&mut host, "d/file", b"x").expect("put");
    assert_eq!(put(&mut host, "d", b"x"), Err(store_put::Failure::Unwritable));
    assert_eq!(put(&mut host, "d/file/under", b"x"), Err(store_put::Failure::Unwritable));
}

#[cfg(unix)]
#[test]
fn store_put_fails_when_the_path_crosses_a_link() {
    let (mut host, directory) = host();
    let outside = directory.path().join("outside");
    std::fs::create_dir_all(&outside).expect("a directory");
    let drafts = host.config().store.join("drafts");
    std::fs::create_dir_all(&drafts).expect("a directory");
    std::os::unix::fs::symlink(&outside, drafts.join("link")).expect("a link");
    assert_eq!(put(&mut host, "link/file", b"x"), Err(store_put::Failure::Unwritable));
    assert!(!outside.join("file").exists(), "nothing was written through the link");
    std::os::unix::fs::symlink(outside.join("target"), drafts.join("dangling")).expect("a link");
    assert_eq!(put(&mut host, "dangling", b"x"), Err(store_put::Failure::Unwritable));
    assert!(!outside.join("target").exists());
}

#[test]
fn store_put_fails_when_the_content_is_over_the_draft_limit() {
    let directory = tempfile::tempdir().expect("a directory");
    let mut config = Config::at(directory.path().join("store"));
    config.draft_max_bytes = 4;
    let mut host = Native::new(config);
    assert_eq!(put(&mut host, "ok", b"1234"), Ok(4));
    assert_eq!(put(&mut host, "big", b"12345"), Err(store_put::Failure::Unwritable));
    assert!(!host.config().store.join("drafts/big").exists());
}

// ---- engine.check ---------------------------------------------------------------------

#[test]
fn engine_check_checks_a_draft_project() {
    let (mut host, _directory) = with_tiny();
    let (class, result) = check(&mut host, "tiny", 0, "").expect("check");
    assert_eq!(class, 0, "{result}");
    let document: serde_json::Value = serde_json::from_str(&result).expect("JSON");
    assert_eq!(document["modules"]["Tiny"]["lean_module"], "Tiny.Tiny");
    assert_eq!(document["source_id"].as_str().map(str::len), Some(64));
}

#[test]
fn engine_check_selects_every_module_or_the_files_named() {
    let (mut host, _directory) = with_tiny();
    assert_eq!(check(&mut host, "tiny", 1, "").expect("check").0, 0);
    assert_eq!(check(&mut host, "tiny", 2, "src/Tiny.lex.tex").expect("check").0, 0);
}

#[test]
fn engine_check_answers_class_one_for_a_bad_draft() {
    let (mut host, _directory) = with_tiny();
    put(&mut host, "tiny/src/Tiny.lex.tex", b"\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n").expect("put");
    let (class, result) = check(&mut host, "tiny", 0, "").expect("check");
    assert_eq!(class, 1, "{result}");
    let document: serde_json::Value = serde_json::from_str(&result).expect("JSON");
    assert!(document["diagnostics"][0]["code"].as_str().is_some_and(|code| code.starts_with("LL")));
}

#[test]
fn engine_check_answers_class_two_for_a_selection_that_is_not_one() {
    let (mut host, _directory) = with_tiny();
    for (selection, files) in [(7, ""), (2, ""), (2, "../x"), (2, "src/Tiny.lex.tex\n"), (2, "src/Nope.lex.tex")] {
        let (class, result) = check(&mut host, "tiny", selection, files).expect("check");
        assert_eq!(class, 2, "{selection} {files:?}: {result}");
    }
}

#[test]
fn engine_check_refuses_a_project_that_is_not_a_draft_project() {
    let (mut host, _directory) = with_tiny();
    std::fs::create_dir_all(host.config().store.join("drafts/empty")).expect("a directory");
    for project in ["absent", "empty", "", "..", "tiny/src", "a b", ".lexlean"] {
        assert_eq!(
            check(&mut host, project, 0, "").map(|_| ()),
            Err(engine_check::Failure::ProjectRefused),
            "{project:?}"
        );
    }
}

#[test]
fn engine_check_is_unavailable_when_the_engine_cannot_run() {
    let (mut host, _directory) = with_tiny();
    let mut config = String::from_utf8(std::fs::read(Path::new(TINY).join("lexlean.toml")).expect("a file")).expect("text");
    config = config.replace("lean_workspace = \".\"", "lean_workspace = \"no-such-workspace\"");
    put(&mut host, "tiny/lexlean.toml", config.as_bytes()).expect("put");
    let outcome = check(&mut host, "tiny", 0, "");
    assert!(
        matches!(outcome, Err(engine_check::Failure::EngineUnavailable)),
        "a missing Lake workspace is an environment error: {outcome:?}"
    );
}

// ---- engine.snapshot ------------------------------------------------------------------

#[test]
fn engine_snapshot_returns_the_semantic_snapshot() {
    let (mut host, _directory) = with_tiny();
    let done = host
        .engine_snapshot(engine_snapshot::Request {
            project: "tiny".into(),
            selection: 0,
            files: String::new(),
        })
        .expect("snapshot");
    assert_eq!(done.class, 0);
    let document: serde_json::Value = serde_json::from_slice(&done.result).expect("JSON");
    assert!(document.is_object() && !document.as_object().expect("an object").is_empty());
}

#[test]
fn engine_snapshot_answers_class_two_for_a_selection_that_is_not_one() {
    let (mut host, _directory) = with_tiny();
    let done = host
        .engine_snapshot(engine_snapshot::Request {
            project: "tiny".into(),
            selection: 9,
            files: String::new(),
        })
        .expect("snapshot");
    assert_eq!(done.class, 2);
}

#[test]
fn engine_snapshot_refuses_a_project_that_is_not_a_draft_project() {
    let (mut host, _directory) = with_tiny();
    let error = host
        .engine_snapshot(engine_snapshot::Request {
            project: "absent".into(),
            selection: 0,
            files: String::new(),
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(engine_snapshot::Failure::ProjectRefused));
}

#[test]
fn engine_snapshot_is_unavailable_when_the_engine_cannot_run() {
    let (mut host, _directory) = with_tiny();
    let config = std::fs::read_to_string(Path::new(TINY).join("lexlean.toml"))
        .expect("a file")
        .replace("lean_workspace = \".\"", "lean_workspace = \"no-such-workspace\"");
    put(&mut host, "tiny/lexlean.toml", config.as_bytes()).expect("put");
    let error = host
        .engine_snapshot(engine_snapshot::Request {
            project: "tiny".into(),
            selection: 0,
            files: String::new(),
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(engine_snapshot::Failure::EngineUnavailable));
}

// ---- engine.format --------------------------------------------------------------------

fn format(host: &mut Native, project: &str, check_only: bool) -> Result<(u32, serde_json::Value), engine_format::Failure> {
    host.engine_format(engine_format::Request {
        project: project.into(),
        selection: 0,
        files: String::new(),
        check_only,
    })
    .map(|done| (done.class, serde_json::from_slice(&done.result).expect("JSON")))
    .map_err(failure)
}

#[test]
fn engine_format_checks_without_writing() {
    let (mut host, _directory) = with_tiny();
    let (class, document) = format(&mut host, "tiny", true).expect("format");
    assert_eq!(class, 0);
    assert_eq!(document["canonical"]["Tiny"], true);
    let drafts = host.config().store.join("drafts/tiny/src/Tiny.lex.tex");
    let mut untidy = std::fs::read_to_string(&drafts).expect("a file");
    untidy.push_str("\n\n\n");
    std::fs::write(&drafts, &untidy).expect("a file");
    let (class, document) = format(&mut host, "tiny", true).expect("format");
    assert_eq!(class, 0);
    assert_eq!(document["canonical"]["Tiny"], false);
    assert_eq!(std::fs::read_to_string(&drafts).expect("a file"), untidy, "check_only writes nothing");
}

#[test]
fn engine_format_rewrites_a_module_to_its_canonical_form() {
    let (mut host, _directory) = with_tiny();
    let drafts = host.config().store.join("drafts/tiny/src/Tiny.lex.tex");
    let canonical = std::fs::read_to_string(&drafts).expect("a file");
    std::fs::write(&drafts, format!("{canonical}\n\n\n")).expect("a file");
    let (class, _) = format(&mut host, "tiny", false).expect("format");
    assert_eq!(class, 0);
    assert_eq!(std::fs::read_to_string(&drafts).expect("a file"), canonical);
}

#[test]
fn engine_format_refuses_a_project_that_is_not_a_draft_project() {
    let (mut host, _directory) = with_tiny();
    assert_eq!(
        format(&mut host, "absent", true).map(|_| ()),
        Err(engine_format::Failure::ProjectRefused)
    );
}

#[test]
fn engine_format_is_unavailable_when_the_engine_cannot_run() {
    let (mut host, _directory) = with_tiny();
    let config = std::fs::read_to_string(Path::new(TINY).join("lexlean.toml"))
        .expect("a file")
        .replace("lean_workspace = \".\"", "lean_workspace = \"no-such-workspace\"");
    put(&mut host, "tiny/lexlean.toml", config.as_bytes()).expect("put");
    assert_eq!(
        format(&mut host, "tiny", true).map(|_| ()),
        Err(engine_format::Failure::EngineUnavailable)
    );
}

// ---- engine.build ---------------------------------------------------------------------

fn build(host: &mut Native, project: &str) -> Result<(u32, serde_json::Value), engine_build::Failure> {
    host.engine_build(engine_build::Request {
        project: project.into(),
        selection: 0,
        files: String::new(),
    })
    .map(|done| (done.class, serde_json::from_slice(&done.result).expect("JSON")))
    .map_err(failure)
}

#[test]
fn engine_build_publishes_a_build_of_the_project() {
    let (mut host, _directory) = with_tiny();
    let (class, document) = build(&mut host, "tiny").expect("build");
    assert_eq!(class, 0, "{document}");
    let id = document["build_id"].as_str().expect("a build id");
    assert_eq!(id.len(), 64);
    let published = host.config().store.join("drafts/tiny/.lexlean/build").join(id);
    assert!(published.is_dir(), "{published:?}");
    let again = build(&mut host, "tiny").expect("build").1;
    assert_eq!(again["build_id"], document["build_id"], "the build is content-addressed");
}

#[test]
fn engine_build_answers_class_one_for_a_bad_draft() {
    let (mut host, _directory) = with_tiny();
    put(&mut host, "tiny/src/Tiny.lex.tex", b"\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n").expect("put");
    assert_eq!(build(&mut host, "tiny").expect("build").0, 1);
}

#[test]
fn engine_build_refuses_a_project_that_is_not_a_draft_project() {
    let (mut host, _directory) = with_tiny();
    assert_eq!(
        build(&mut host, "absent").map(|_| ()),
        Err(engine_build::Failure::ProjectRefused)
    );
}

#[test]
fn engine_build_is_unavailable_when_the_engine_cannot_run() {
    let (mut host, _directory) = with_tiny();
    let config = std::fs::read_to_string(Path::new(TINY).join("lexlean.toml"))
        .expect("a file")
        .replace("lean_workspace = \".\"", "lean_workspace = \"no-such-workspace\"");
    put(&mut host, "tiny/lexlean.toml", config.as_bytes()).expect("put");
    assert_eq!(
        build(&mut host, "tiny").map(|_| ()),
        Err(engine_build::Failure::EngineUnavailable)
    );
}

// ---- verify.start and verify.control --------------------------------------------------

/// A runner that waits for a signal, and counts how many run at once.
struct Gate {
    release: Mutex<mpsc::Receiver<()>>,
    running: Mutex<(usize, usize)>,
}

fn gated_host() -> (Native, tempfile::TempDir, mpsc::Sender<()>, Arc<Gate>) {
    let directory = tempfile::tempdir().expect("a directory");
    let (send, receive) = mpsc::channel();
    let gate = Arc::new(Gate {
        release: Mutex::new(receive),
        running: Mutex::new((0, 0)),
    });
    let runner_gate = Arc::clone(&gate);
    let host = Native::with_runner(
        Config::at(directory.path().join("store")),
        Arc::new(move |_job: &Job| {
            {
                let mut running = runner_gate.running.lock().expect("a lock");
                running.0 += 1;
                running.1 = running.1.max(running.0);
            }
            let _ = runner_gate
                .release
                .lock()
                .expect("a lock")
                .recv_timeout(Duration::from_secs(30));
            runner_gate.running.lock().expect("a lock").0 -= 1;
            Outcome {
                class: 0,
                result: b"{\"verified\":true}".to_vec(),
            }
        }),
    );
    copy(
        Path::new(TINY),
        &host.config().store.join("drafts").join("tiny"),
    );
    (host, directory, send, gate)
}

fn start(host: &mut Native, project: &str) -> Result<(String, u32), verify_start::Failure> {
    host.verify_start(verify_start::Request {
        project: project.into(),
        selection: 0,
        files: String::new(),
    })
    .map(|done| (done.task, done.state))
    .map_err(failure)
}

fn status(host: &mut Native, task: &str) -> Result<verify_control::Completed, verify_control::Failure> {
    host.verify_control(verify_control::Request {
        task: task.into(),
        action: 0,
    })
    .map_err(failure)
}

fn wait_for(host: &mut Native, task: &str, state: State) -> verify_control::Completed {
    let until = Instant::now() + Duration::from_secs(20);
    loop {
        let report = status(host, task).expect("a known task");
        if report.state == state as u32 {
            return report;
        }
        assert!(Instant::now() < until, "{task} did not reach {state:?}: {}", report.state);
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn verify_start_runs_a_task_to_its_end() {
    let (mut host, _directory, release, _gate) = gated_host();
    let (task, state) = start(&mut host, "tiny").expect("start");
    assert_eq!(task, "task-1");
    assert_eq!(state, State::Running as u32);
    assert_eq!(status(&mut host, &task).expect("status").state, State::Running as u32);
    release.send(()).expect("a signal");
    let done = wait_for(&mut host, &task, State::Verified);
    assert_eq!(done.class, 0);
    assert_eq!(done.result, b"{\"verified\":true}");
}

#[test]
fn verify_start_runs_one_task_at_a_time_and_none_fails_for_waiting() {
    let (mut host, _directory, release, gate) = gated_host();
    let (first, state) = start(&mut host, "tiny").expect("start");
    assert_eq!(state, State::Running as u32);
    let (second, state) = start(&mut host, "tiny").expect("start");
    let (third, third_state) = start(&mut host, "tiny").expect("start");
    assert_eq!(state, State::Queued as u32);
    assert_eq!(third_state, State::Queued as u32);
    assert_eq!((first.as_str(), second.as_str(), third.as_str()), ("task-1", "task-2", "task-3"));
    for task in [&first, &second, &third] {
        release.send(()).expect("a signal");
        wait_for(&mut host, task, State::Verified);
    }
    assert_eq!(gate.running.lock().expect("a lock").1, 1, "never two at once");
}

#[test]
fn verify_start_refuses_a_project_that_is_not_a_draft_project() {
    let (mut host, _directory, _release, _gate) = gated_host();
    for project in ["absent", "..", ""] {
        assert_eq!(start(&mut host, project), Err(verify_start::Failure::ProjectRefused), "{project:?}");
    }
    let error = host
        .verify_start(verify_start::Request {
            project: "tiny".into(),
            selection: 5,
            files: String::new(),
        })
        .map(|_| ())
        .map_err(failure);
    assert_eq!(error, Err(verify_start::Failure::ProjectRefused), "a selection that is not one");
}

#[test]
fn verify_start_records_the_failure_of_a_bad_draft() {
    let directory = tempfile::tempdir().expect("a directory");
    let mut host = Native::new(Config::at(directory.path().join("store")));
    copy(
        Path::new(TINY),
        &host.config().store.join("drafts").join("tiny"),
    );
    put(&mut host, "tiny/src/Tiny.lex.tex", b"\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n").expect("put");
    let (task, _) = start(&mut host, "tiny").expect("start");
    let done = wait_for(&mut host, &task, State::Failed);
    assert_eq!(done.class, 1, "a language error");
    assert!(String::from_utf8_lossy(&done.result).contains("diagnostics"));
}

#[test]
#[ignore = "runs a real verification, which needs the Lean toolchain: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn verify_start_runs_a_real_verification_and_leaves_a_record() {
    let (mut host, _directory) = with_tiny();
    let (task, _) = start(&mut host, "tiny").expect("start");
    let until = Instant::now() + Duration::from_secs(900);
    let done = loop {
        let report = status(&mut host, &task).expect("status");
        if report.state == State::Verified as u32 || report.state == State::Failed as u32 {
            break report;
        }
        assert!(Instant::now() < until, "the verification did not end");
        std::thread::sleep(Duration::from_millis(500));
    };
    assert_eq!(done.state, State::Verified as u32, "{}", String::from_utf8_lossy(&done.result));
    let record: serde_json::Value = serde_json::from_slice(&done.result).expect("JSON");
    let attestation = record["attestation_id"].as_str().expect("an id").to_string();
    // The record is found by its ID, and by the digest of a source file it verified.
    let by_id = host
        .attestation_lookup(attestation_lookup::Request { by: 0, value: attestation.clone() })
        .expect("lookup");
    assert!(by_id.found);
    let source = std::fs::read(Path::new(TINY).join("src/Tiny.lex.tex")).expect("a file");
    let digest = lexlean::Sha256Digest::of(&source).to_hex();
    let by_file = host
        .attestation_lookup(attestation_lookup::Request { by: 1, value: digest })
        .expect("lookup");
    assert!(by_file.found);
    assert_eq!(by_file.envelope, by_id.envelope);
}

#[test]
fn verify_control_reports_the_status_of_a_task() {
    let (mut host, _directory, release, _gate) = gated_host();
    let (task, _) = start(&mut host, "tiny").expect("start");
    let running = status(&mut host, &task).expect("status");
    assert_eq!((running.state, running.class), (State::Running as u32, 0));
    assert!(running.result.is_empty());
    assert_eq!((running.groups_ended, running.groups_failed), (0, 0));
    release.send(()).expect("a signal");
    wait_for(&mut host, &task, State::Verified);
}

#[test]
fn verify_control_cancels_a_queued_task_and_it_never_runs() {
    let (mut host, _directory, release, gate) = gated_host();
    let (first, _) = start(&mut host, "tiny").expect("start");
    let (second, state) = start(&mut host, "tiny").expect("start");
    assert_eq!(state, State::Queued as u32);
    let cancelled = host
        .verify_control(verify_control::Request { task: second.clone(), action: 1 })
        .expect("cancel");
    assert_eq!(cancelled.state, State::Cancelled as u32);
    assert_eq!((cancelled.groups_ended, cancelled.groups_failed), (0, 0));
    release.send(()).expect("a signal");
    wait_for(&mut host, &first, State::Verified);
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(status(&mut host, &second).expect("status").state, State::Cancelled as u32);
    assert_eq!(gate.running.lock().expect("a lock").1, 1);
}

#[test]
fn verify_control_cancels_a_running_task() {
    let (mut host, _directory, release, _gate) = gated_host();
    let (task, _) = start(&mut host, "tiny").expect("start");
    let cancelled = host
        .verify_control(verify_control::Request { task: task.clone(), action: 1 })
        .expect("cancel");
    assert_eq!(cancelled.state, State::Cancelled as u32);
    release.send(()).expect("a signal");
    std::thread::sleep(Duration::from_millis(100));
    let after = status(&mut host, &task).expect("status");
    assert_eq!(after.state, State::Cancelled as u32, "the end of the run does not undo the cancellation");
    // A task started after the cancellation runs: children are allowed again.
    let (next, _) = start(&mut host, "tiny").expect("start");
    release.send(()).expect("a signal");
    wait_for(&mut host, &next, State::Verified);
}

#[test]
fn verify_control_leaves_a_finished_task_as_it_ended() {
    let (mut host, _directory, release, _gate) = gated_host();
    let (task, _) = start(&mut host, "tiny").expect("start");
    release.send(()).expect("a signal");
    wait_for(&mut host, &task, State::Verified);
    let after = host
        .verify_control(verify_control::Request { task, action: 1 })
        .expect("cancel");
    assert_eq!(after.state, State::Verified as u32);
}

#[test]
fn verify_control_refuses_a_task_that_is_not_known() {
    let (mut host, _directory) = host();
    for action in [0, 1] {
        let error = host
            .verify_control(verify_control::Request { task: "task-9".into(), action })
            .map(|_| ())
            .map_err(failure);
        assert_eq!(error, Err(verify_control::Failure::UnknownTask), "action {action}");
    }
}

// ---- lexicon.read ---------------------------------------------------------------------

fn lexicon(host: &mut Native, profile: &str, entry: &str) -> Result<(bool, Vec<u8>), lexicon_read::Failure> {
    host.lexicon_read(lexicon_read::Request {
        profile: profile.into(),
        entry: entry.into(),
    })
    .map(|done| (done.found, done.content))
    .map_err(failure)
}

#[test]
fn lexicon_read_returns_an_entry_of_a_profile() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 0, key: "p1/entries/a.toml".into(), content: b"phrase".to_vec() })
        .expect("put");
    assert_eq!(lexicon(&mut host, "p1", "entries/a.toml"), Ok((true, b"phrase".to_vec())));
}

#[test]
fn lexicon_read_reports_a_missing_entry_as_not_found() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 0, key: "p1/lexicon.toml".into(), content: b"x".to_vec() })
        .expect("put");
    assert_eq!(lexicon(&mut host, "p1", "entries/none.toml"), Ok((false, Vec::new())));
}

#[test]
fn lexicon_read_refuses_a_profile_that_is_not_known() {
    let (mut host, _directory) = host();
    for profile in ["absent", "", "..", "a/b", ".hidden"] {
        assert_eq!(
            lexicon(&mut host, profile, "entry"),
            Err(lexicon_read::Failure::ProfileUnknown),
            "{profile:?}"
        );
    }
}

#[test]
fn lexicon_read_refuses_an_entry_that_is_not_a_key() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 0, key: "p1/x".into(), content: b"x".to_vec() })
        .expect("put");
    for entry in ["", "..", "../p1/x", "/etc/passwd", "a//b"] {
        assert_eq!(
            lexicon(&mut host, "p1", entry),
            Err(lexicon_read::Failure::KeyRefused),
            "{entry:?}"
        );
    }
}

// ---- attestation.lookup ---------------------------------------------------------------

fn lookup(host: &mut Native, by: u32, value: &str) -> Result<(bool, Vec<u8>), attestation_lookup::Failure> {
    host.attestation_lookup(attestation_lookup::Request { by, value: value.into() })
        .map(|done| (done.found, done.envelope))
        .map_err(failure)
}

const ID: &str = "5ee76e509683c39ef3347d2587071270720c04fd7e270b6f4b1538b4d96426cc";
const DIGEST: &str = "6dc4489074435e5d259cd98c09af76cade23c313e899ab46f12b8ecbd7020d46";

#[test]
fn attestation_lookup_finds_a_record_by_its_id() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 3, key: format!("{ID}.json"), content: b"{\"a\":1}".to_vec() })
        .expect("put");
    assert_eq!(lookup(&mut host, 0, ID), Ok((true, b"{\"a\":1}".to_vec())));
}

#[test]
fn attestation_lookup_finds_a_record_by_the_digest_of_a_verified_file() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 3, key: format!("{ID}.json"), content: b"{\"a\":1}".to_vec() })
        .expect("put");
    host.store_put(store_put::Request { area: 4, key: format!("by-file/{DIGEST}"), content: ID.as_bytes().to_vec() })
        .expect("put");
    assert_eq!(lookup(&mut host, 1, DIGEST), Ok((true, b"{\"a\":1}".to_vec())));
}

#[test]
fn attestation_lookup_reports_an_id_that_has_no_record_as_not_found() {
    let (mut host, _directory) = host();
    assert_eq!(lookup(&mut host, 0, ID), Ok((false, Vec::new())));
    assert_eq!(lookup(&mut host, 1, DIGEST), Ok((false, Vec::new())));
}

#[test]
fn attestation_lookup_refuses_a_value_that_is_not_a_digest() {
    let (mut host, _directory) = host();
    let upper = ID.to_uppercase();
    let short = &ID[1..];
    for value in ["", "abc", short, upper.as_str(), "../../x", &format!("{ID}0")] {
        assert_eq!(
            lookup(&mut host, 0, value),
            Err(attestation_lookup::Failure::ValueRefused),
            "{value:?}"
        );
    }
    assert_eq!(lookup(&mut host, 2, ID), Err(attestation_lookup::Failure::ValueRefused), "a way to look up that is not one");
}

#[test]
fn attestation_lookup_faults_when_the_index_names_something_that_is_not_an_id() {
    let (mut host, _directory) = host();
    host.store_put(store_put::Request { area: 4, key: format!("by-file/{DIGEST}"), content: b"not an id".to_vec() })
        .expect("put");
    let error = host
        .attestation_lookup(attestation_lookup::Request { by: 1, value: DIGEST.into() })
        .map(|_| ());
    assert_eq!(error, Err(HostError::Fault));
}

// ---- rate.calls -----------------------------------------------------------------------

#[test]
fn rate_calls_answers_with_the_count_before_the_request() {
    let (mut host, _directory) = host();
    let mut calls = |counted| {
        host.rate_calls(rate_calls::Request { counted })
            .expect("the counter cannot fail")
            .calls
    };
    assert_eq!(calls(true), 0);
    assert_eq!(calls(true), 1);
    assert_eq!(calls(false), 2, "a request that is not counted still sees the count");
    assert_eq!(calls(false), 2, "and does not move it");
    assert_eq!(calls(true), 2);
}

#[test]
fn rate_calls_starts_again_when_the_window_closes() {
    let (mut host, _directory) = host();
    let window = host.config().rate_window;
    let start = Instant::now();
    assert_eq!(host.count(true, start), 0);
    assert_eq!(host.count(true, start + window - Duration::from_millis(1)), 1);
    assert_eq!(host.count(true, start + window), 0, "the window closed at its length");
    assert_eq!(host.count(true, start + window + Duration::from_secs(1)), 1);
}
