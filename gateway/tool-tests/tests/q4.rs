//! TL-08, Q4: check, snapshot, format and build start no child process.
//!
//! The observation is the process accounting of the host process, which the kernel keeps
//! and the library cannot edit: the minor and major faults and the user and system time of the
//! children this process has waited for (`/proc/self/stat`, fields 11, 13, 16 and 17), and the
//! children that are alive in any thread (`/proc/self/task/*/children`). A child that was
//! started and waited for moves the first; a child that is alive appears in the second.
//!
//! This file is a test binary of its own and holds one test, so nothing else in the process
//! spawns while the tools run; the control at its end spawns a child on purpose and the same
//! instrument must see it, or the instrument sees nothing and the test proves nothing.

use std::process::Command;

use gateway_tool_tests::{tiny, Gateway, TOOLS};
use serde_json::json;

/// The accounting of the children this process has waited for.
fn waited_for() -> [u64; 4] {
    let stat = std::fs::read_to_string("/proc/self/stat").expect("/proc/self/stat");
    // The command name is in parentheses and may hold spaces: the fields start after the last
    // `)`, at field 3.
    let fields: Vec<&str> = stat
        .rsplit_once(") ")
        .expect("a command name")
        .1
        .split(' ')
        .collect();
    let at = |field: usize| -> u64 { fields[field - 3].parse().expect("a number") };
    [at(11), at(13), at(16), at(17)]
}

/// The pids of the children that are alive, or `None` when the kernel does not list them.
fn alive() -> Option<Vec<u32>> {
    let mut pids = Vec::new();
    let mut listed = false;
    for task in std::fs::read_dir("/proc/self/task").expect("/proc/self/task").flatten() {
        if let Ok(children) = std::fs::read_to_string(task.path().join("children")) {
            listed = true;
            pids.extend(children.split_whitespace().filter_map(|pid| pid.parse::<u32>().ok()));
        }
    }
    listed.then_some(pids)
}

#[test]
fn no_child_process_is_started_by_the_four_tools_tl_08() {
    let mut gateway = Gateway::new();
    gateway.install("tiny", &tiny());
    let before = (waited_for(), alive());
    for tool in TOOLS {
        let reply = gateway.call(tool, json!({"project": "tiny"}));
        assert_eq!(reply.effects.len(), 1, "{tool}: {}", reply.body);
        assert!(
            reply.result().get("isError").is_none_or(|flag| flag == false),
            "{tool} succeeds, so it ran the whole pipeline: {}",
            reply.body
        );
        assert_eq!(
            (waited_for(), alive()),
            before,
            "{tool} started a child process"
        );
    }
    // Every selection and the check-only format are paths of the same calls.
    for (tool, arguments) in [
        ("lexlean_check", json!({"project": "tiny", "all": true})),
        ("lexlean_snapshot", json!({"project": "tiny", "files": ["src/Tiny.lex.tex"]})),
        ("lexlean_format", json!({"project": "tiny", "check_only": true})),
    ] {
        gateway.call(tool, arguments);
        assert_eq!((waited_for(), alive()), before, "{tool} started a child process");
    }

    // The control: a child that is started and waited for moves the accounting, and a child
    // that is alive is listed (when the kernel lists children at all).
    let mut child = Command::new("true").spawn().expect("a child");
    child.wait().expect("the child ends");
    assert_ne!(waited_for(), before.0, "the accounting sees a child that was waited for");
    if before.1.is_some() {
        let mut sleeper = Command::new("sleep").arg("30").spawn().expect("a child");
        let listed = alive().expect("children are listed");
        assert!(
            listed.contains(&sleeper.id()),
            "the accounting sees a live child: {listed:?}"
        );
        sleeper.kill().expect("the child ends");
        sleeper.wait().expect("the child is waited for");
    }
}
