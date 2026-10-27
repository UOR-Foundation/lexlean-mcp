//! `gateway-provision`: the operator's commands that make a profile for the gateway.
//!
//! ```text
//! gateway-provision init DIR --name NAME --module-prefix PREFIX [--language 1.0|1.1|1.2]
//! gateway-provision lock PROJECT [--allow-network]
//! gateway-provision identify PROJECT [--review FILE]...
//! gateway-provision install --store STORE NAME PROJECT [--review FILE]...
//! ```
//!
//! `init` and `lock` are `lexlean init` and `lexlean lock` on the compiler of the gateway, so a
//! profile is locked by the compiler that will check its drafts. `--allow-network` belongs to
//! `lock` alone, because only a Git package can need the network, and no other command reaches it.
//! `identify` and `install` print the profile ID. A refusal prints `refused <code>` and exits 1; a
//! misuse of the command line prints nothing on the standard output and exits 2.

use std::path::PathBuf;
use std::process::ExitCode;

use gateway_binding::provision::{identify, init, install, lock, Refusal};

const USAGE: &str = "usage:\n  gateway-provision init DIR --name NAME --module-prefix PREFIX [--language 1.0|1.1|1.2]\n  gateway-provision lock PROJECT [--allow-network]\n  gateway-provision identify PROJECT [--review FILE]...\n  gateway-provision install --store STORE NAME PROJECT [--review FILE]...";

/// The arguments of a command, split into the values of the flags it takes and its operands.
struct Arguments {
    operands: Vec<String>,
    flags: Vec<(String, String)>,
    switches: Vec<String>,
}

/// Split `arguments` by the flags in `valued` (each followed by a value) and `switches` (alone).
/// `None` when a flag is unknown or lacks its value.
fn split(arguments: &[String], valued: &[&str], switches: &[&str]) -> Option<Arguments> {
    let mut found = Arguments {
        operands: Vec::new(),
        flags: Vec::new(),
        switches: Vec::new(),
    };
    let mut rest = arguments.iter();
    while let Some(argument) = rest.next() {
        if valued.contains(&argument.as_str()) {
            found.flags.push((argument.clone(), rest.next()?.clone()));
        } else if switches.contains(&argument.as_str()) {
            found.switches.push(argument.clone());
        } else if argument.starts_with("--") {
            return None;
        } else {
            found.operands.push(argument.clone());
        }
    }
    Some(found)
}

impl Arguments {
    fn flag(&self, name: &str) -> Option<&str> {
        self.flags
            .iter()
            .find(|(flag, _)| flag == name)
            .map(|(_, value)| value.as_str())
    }

    fn all(&self, name: &str) -> Vec<&str> {
        self.flags
            .iter()
            .filter(|(flag, _)| flag == name)
            .map(|(_, value)| value.as_str())
            .collect()
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::from(2)
}

fn report<T>(result: Result<T, Refusal>, print: impl FnOnce(T)) -> ExitCode {
    match result {
        Ok(value) => {
            print(value);
            ExitCode::SUCCESS
        }
        Err(refused) => {
            eprintln!("{refused}");
            ExitCode::from(1)
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn reviews_of(arguments: &Arguments) -> Vec<PathBuf> {
    arguments
        .all("--review")
        .into_iter()
        .map(PathBuf::from)
        .collect()
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let Some((command, rest)) = arguments.split_first() else {
        return usage();
    };
    match command.as_str() {
        "init" => {
            let Some(found) = split(rest, &["--name", "--module-prefix", "--language"], &[]) else {
                return usage();
            };
            let (Some(name), Some(prefix), [destination]) = (
                found.flag("--name"),
                found.flag("--module-prefix"),
                &found.operands[..],
            ) else {
                return usage();
            };
            let language = found.flag("--language").unwrap_or("1.2");
            report(
                init(&PathBuf::from(destination), name, prefix, language),
                |()| (),
            )
        }
        "lock" => {
            let Some(found) = split(rest, &[], &["--allow-network"]) else {
                return usage();
            };
            let [project] = &found.operands[..] else {
                return usage();
            };
            let network = !found.switches.is_empty();
            report(lock(&PathBuf::from(project), network), |()| ())
        }
        "identify" => {
            let Some(found) = split(rest, &["--review"], &[]) else {
                return usage();
            };
            let [project] = &found.operands[..] else {
                return usage();
            };
            let mut records = Vec::new();
            for path in reviews_of(&found) {
                match std::fs::read(&path) {
                    Ok(bytes) => records.push(bytes),
                    Err(_) => return report::<()>(Err(Refusal::ProjectUnreadable), |()| ()),
                }
            }
            report(identify(&PathBuf::from(project), &records), |identified| {
                println!("{}", hex(&identified.id));
            })
        }
        "install" => {
            let Some(found) = split(rest, &["--store", "--review"], &[]) else {
                return usage();
            };
            let (Some(store), [name, project]) = (found.flag("--store"), &found.operands[..])
            else {
                return usage();
            };
            let reviews = reviews_of(&found);
            let reviews: Vec<&std::path::Path> = reviews.iter().map(PathBuf::as_path).collect();
            report(
                install(
                    &PathBuf::from(store),
                    name,
                    &PathBuf::from(project),
                    &reviews,
                ),
                |installed| println!("{}", hex(&installed.id)),
            )
        }
        _ => usage(),
    }
}
