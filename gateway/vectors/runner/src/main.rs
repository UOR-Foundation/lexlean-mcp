//! The recorder of the vectors of the gateway.
//!
//! `gateway/vectors/generate.py record` runs this program. It reads one request frame for
//! each line of its standard input, in hex, and sends each through the production core
//! with the real host (`gateway_binding::Native`) as `gateway-host run` does: a turn for
//! the core, the effect it asks for, its outcome from the host, and the next turn, until the
//! core answers. For each request it prints one line of JSON, the response frame and the
//! effects performed, each with the payload the core asked with and the outcome the host
//! gave, so that the declaration can state them as the transcript of the vector.
//!
//! The requests share one host, so a request sees what the requests before it stored.
//! The store is the one `LEXLEAN_MCP_STORE` names.

use std::io::{BufRead, Write};

use gateway_host::exchange::{core, step, turn, Step};
use gateway_host::perform::perform;
use gateway_host::protocol::{Request, EFFECTS};
use serde_json::json;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if !text.len().is_multiple_of(2) {
        return Err("an odd number of hex digits".to_string());
    }
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).map_err(|error| error.to_string()))
        .collect()
}

fn exchange(
    host: &mut gateway_binding::Native,
    request: &[u8],
) -> Result<serde_json::Value, String> {
    let mut completed: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut steps = Vec::new();
    loop {
        let input = turn(request, &completed).map_err(|failure| format!("{failure:?}"))?;
        let output = core(input).map_err(|failure| format!("{failure:?}"))?;
        match step(&output).map_err(|failure| format!("{failure:?}"))? {
            Step::Done(response) => {
                return Ok(json!({"response": hex(response), "steps": steps}));
            }
            Step::Perform(tag, payload) => {
                let effect =
                    Request::decode(tag, payload).map_err(|refusal| format!("{refusal:?}"))?;
                let outcome = perform(host, effect).map_err(|failure| format!("{failure:?}"))?;
                let name = usize::try_from(tag)
                    .ok()
                    .and_then(|index| EFFECTS.get(index))
                    .ok_or("an effect that is not declared")?;
                steps.push(json!({
                    "effect": name,
                    "request": hex(payload),
                    "outcome": hex(&outcome),
                }));
                completed.push((tag, outcome));
            }
        }
    }
}

/// The stack of the thread the core runs on: the declaration's `rootStackBytes`, with room.
const STACK_BYTES: usize = 512 << 20;

fn main() -> Result<(), String> {
    std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(record)
        .map_err(|error| error.to_string())?
        .join()
        .map_err(|_| "the recorder panicked".to_string())?
}

fn record() -> Result<(), String> {
    let mut host = gateway_binding::Native::default();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| error.to_string())?;
        if line.trim().is_empty() {
            continue;
        }
        let request = unhex(line.trim())?;
        let answer = exchange(&mut host, &request)?;
        writeln!(stdout, "{answer}").map_err(|error| error.to_string())?;
    }
    Ok(())
}
