//! What a lost verifier leaves behind, and its end.
//!
//! LexLean starts every child of a verification (`lake`, `lean`) in a process group of its own
//! and with an empty environment, so a verifier that is killed (by the OOM killer, say) leaves
//! its children running, and nothing in their environment says whose they are. What they keep is
//! their session: the verifier makes a session of its own when it starts, a session survives the
//! death of its leader and is inherited by every descendant, and the kernel lists it in
//! `/proc/<pid>/stat`. The host kills what is in the session of a verifier that has ended.

use std::fs;
use std::time::Duration;

use rustix::process::{kill_process, Pid, Signal};

/// The state and the session in the text of `/proc/<pid>/stat`. The name of the process is in
/// parentheses and may hold spaces and parentheses, so the fields start after the last `)`.
fn state_and_session(stat: &str) -> Option<(char, u32)> {
    let rest = &stat[stat.rfind(')')? + 1..];
    let mut fields = rest.split_whitespace();
    let state = fields.next()?.chars().next()?;
    // state, parent, group, session
    let session = fields.nth(2)?.parse().ok()?;
    Some((state, session))
}

/// The processes in `session`, other than this one and the dead ones that wait to be reaped.
fn members(session: u32) -> Vec<u32> {
    let own = std::process::id();
    let Ok(entries) = fs::read_dir("/proc") else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let pid: u32 = entry.file_name().to_str()?.parse().ok()?;
            if pid == own {
                return None;
            }
            let stat = fs::read_to_string(entry.path().join("stat")).ok()?;
            let (state, found) = state_and_session(&stat)?;
            (found == session && state != 'Z').then_some(pid)
        })
        .collect()
}

/// Kill every process in `session`, as often as it takes (a process may start another before it
/// dies), and return how many were killed. The session of the host itself is never touched.
pub fn kill_session(session: u32) -> u32 {
    let own = fs::read_to_string("/proc/self/stat")
        .ok()
        .and_then(|stat| state_and_session(&stat))
        .map(|(_, own)| own);
    if own == Some(session) {
        return 0;
    }
    let mut killed = 0;
    for _ in 0..5 {
        let found = members(session);
        if found.is_empty() {
            break;
        }
        for pid in found {
            let target = i32::try_from(pid).ok().and_then(Pid::from_raw);
            if target.is_some_and(|target| kill_process(target, Signal::KILL).is_ok()) {
                killed += 1;
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    killed
}

/// Kill the process `pid`.
pub fn kill(pid: u32) {
    if let Some(target) = i32::try_from(pid).ok().and_then(Pid::from_raw) {
        let _ = kill_process(target, Signal::KILL);
    }
}
