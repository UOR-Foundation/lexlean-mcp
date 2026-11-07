//! Whether the gateway is serving: the readiness command of the deployment (DEPLOYMENT.md, DP-06).
//!
//! The generated `gateway-host` has no mode that answers this, so this program asks it over HTTP,
//! as a client would. A `GET` is refused by the adapter before the core runs, so the probe calls no
//! tool and asks no effect: any HTTP/1.1 status line is the adapter answering.

#![forbid(unsafe_code)]

use std::fmt;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

/// How long the probe waits to connect, to write and to read.
pub const TIMEOUT: Duration = Duration::from_secs(5);

/// The most bytes of the answer the probe reads: a status line is far shorter.
pub const MAX_STATUS_BYTES: usize = 256;

/// Why the gateway is not ready. `gateway-ready` exits 1 with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotReady {
    /// Nothing accepted a connection at the address, or the connection failed; the text is the
    /// error of the system.
    Unreachable(String),
    /// Something answered, but not with an HTTP/1.1 status line.
    NotHttp,
}

impl fmt::Display for NotReady {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotReady::Unreachable(error) => {
                write!(formatter, "the gateway is unreachable: {error}")
            }
            NotReady::NotHttp => {
                formatter.write_str("the gateway did not answer with an HTTP/1.1 status line")
            }
        }
    }
}

impl std::error::Error for NotReady {}

/// Whether `line` begins with an HTTP/1.1 status line: the version, a space and three digits,
/// then a space or the end of the line.
pub fn is_status_line(line: &[u8]) -> bool {
    line.len() >= 12
        && line.starts_with(b"HTTP/1.1 ")
        && line[9..12].iter().all(u8::is_ascii_digit)
        && (line.len() == 12 || line[12] == b' ' || line[12] == b'\r')
}

/// Ask the server at `address` for its status line.
pub fn probe(address: SocketAddr, timeout: Duration) -> Result<(), NotReady> {
    let failed = |error: std::io::Error| NotReady::Unreachable(error.to_string());
    let mut stream = TcpStream::connect_timeout(&address, timeout).map_err(failed)?;
    stream.set_read_timeout(Some(timeout)).map_err(failed)?;
    stream.set_write_timeout(Some(timeout)).map_err(failed)?;
    let request = format!("GET / HTTP/1.1\r\nHost: {address}\r\nConnection: close\r\n\r\n");
    stream.write_all(request.as_bytes()).map_err(failed)?;
    stream.flush().map_err(failed)?;
    let mut answer = Vec::new();
    let mut buffer = [0u8; MAX_STATUS_BYTES];
    while answer.len() < MAX_STATUS_BYTES && !answer.contains(&b'\n') {
        match stream.read(&mut buffer[..MAX_STATUS_BYTES - answer.len()]) {
            Ok(0) => break,
            Ok(count) => answer.extend_from_slice(&buffer[..count]),
            Err(error) if answer.is_empty() => return Err(failed(error)),
            Err(_) => break,
        }
    }
    if is_status_line(&answer) {
        Ok(())
    } else {
        Err(NotReady::NotHttp)
    }
}
