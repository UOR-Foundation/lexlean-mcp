//! The side of the host: the process of a task, through the socket of `gateway-verifier-host`.

use std::io::{BufRead, BufReader, Lines, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use serde_json::{json, Value};

use crate::service::{refusal_named, Refusal, TIMEOUT};

/// Why a task has no process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unavailable {
    /// Nothing answered at the socket, or the answer was not one the service could give.
    Lost,
    /// The service refused, as it said.
    Refused(Refusal),
}

/// One line and its answer, within `TIMEOUT` each way. The answer is read through `reader`, which
/// goes on to read the rest: what it has buffered behind the answer is the first lines of the task.
fn first_answer(
    stream: &UnixStream,
    reader: &mut BufReader<UnixStream>,
    request: &Value,
) -> Result<Value, Unavailable> {
    let lost = |_| Unavailable::Lost;
    stream.set_read_timeout(Some(TIMEOUT)).map_err(lost)?;
    stream.set_write_timeout(Some(TIMEOUT)).map_err(lost)?;
    let mut writer = stream;
    writeln!(writer, "{request}")
        .and_then(|()| writer.flush())
        .map_err(lost)?;
    let mut line = Vec::new();
    reader.read_until(b'\n', &mut line).map_err(lost)?;
    let answer: Value = serde_json::from_slice(&line).map_err(|_| Unavailable::Lost)?;
    match answer.get("refused") {
        Some(name) => Err(name
            .as_str()
            .and_then(refusal_named)
            .map_or(Unavailable::Lost, Unavailable::Refused)),
        None => Ok(answer),
    }
}

/// Whether the service at `socket` answers and has a verifier program to start.
pub fn ready(socket: &Path) -> bool {
    let Ok(stream) = UnixStream::connect(socket) else {
        return false;
    };
    let Ok(reader) = stream.try_clone() else {
        return false;
    };
    first_answer(
        &stream,
        &mut BufReader::new(reader),
        &json!({"op": "ready"}),
    )
    .is_ok_and(|answer| answer["ready"] == true)
}

/// What a task asks of its process.
#[derive(Debug, Clone)]
pub struct Run {
    /// The width of the verification.
    pub width: u32,
    /// Its memory budget, in KiB.
    pub memory_kib: u64,
    /// How long the process has to end, after its verdict or after the host has gone.
    pub grace: Duration,
    /// The job line of the process: its directory and its selection.
    pub job: Value,
}

/// The input of a running task. Letting go of it is the end of the input of the process.
#[derive(Debug)]
pub struct Input {
    stream: UnixStream,
}

impl Input {
    fn say(&mut self, line: &str) {
        let _ = writeln!(self.stream, "{line}").and_then(|()| self.stream.flush());
    }

    /// Ask the process to cancel. It answers with its verdict.
    pub fn cancel(&mut self) {
        self.say("cancel");
    }

    /// Have the service kill the process and everything in its session.
    pub fn kill(&mut self) {
        self.say("kill");
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        let _ = self.stream.shutdown(Shutdown::Write);
    }
}

/// The lines of a running task, as events. The last is `ended`; a stream that stops without it is a
/// service that was lost.
pub struct Events {
    lines: Lines<BufReader<UnixStream>>,
}

impl Iterator for Events {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        loop {
            let line = self.lines.next()?.ok()?;
            if let Ok(event) = serde_json::from_str(&line) {
                return Some(event);
            }
        }
    }
}

/// Start the process of a task at the service at `socket`.
pub fn run(socket: &Path, run: &Run) -> Result<(Input, Events), Unavailable> {
    let lost = |_| Unavailable::Lost;
    let stream = UnixStream::connect(socket).map_err(lost)?;
    let grace = u64::try_from(run.grace.as_millis()).unwrap_or(u64::MAX);
    let mut reader = BufReader::new(stream.try_clone().map_err(lost)?);
    let answer = first_answer(
        &stream,
        &mut reader,
        &json!({
            "op": "run",
            "width": run.width,
            "memory_kib": run.memory_kib,
            "grace_ms": grace,
            "job": run.job,
        }),
    )?;
    if answer["event"] != "accepted" {
        return Err(Unavailable::Lost);
    }
    // A verification takes as long as its limits let it: from here a lost service ends the stream.
    stream.set_read_timeout(None).map_err(lost)?;
    Ok((
        Input { stream },
        Events {
            lines: reader.lines(),
        },
    ))
}
