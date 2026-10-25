//! Where the host keeps its state, and the owner values that bound it.

use std::path::PathBuf;
use std::time::Duration;

/// The largest draft the store holds, in bytes: the owner value `draft.max_bytes`.
pub const DRAFT_MAX_BYTES: usize = 8_388_608;

/// The most bytes the framing of a draft adds to its files: `draft.max_files` frames of 36 octets,
/// the paths (which `draft.max_bytes` does not count) and the three heads of the preimage. A draft
/// of `draft.max_bytes` of content is stored whole.
pub const DRAFT_BLOB_HEADROOM: usize = 1_048_576;

/// The window of the rate counter: the owner value `rate.window_ms`.
pub const RATE_WINDOW: Duration = Duration::from_millis(60_000);

/// The most bytes of a verification log that a status carries.
pub const LOG_TAIL_BYTES: usize = 65_536;

/// The host's configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// The root of the store: one directory for each of the seven areas.
    pub store: PathBuf,
    /// The largest value the store holds in one key, in bytes: a draft is stored as its preimage,
    /// the `draft.max_bytes` of content and `DRAFT_BLOB_HEADROOM` for its framing.
    pub blob_max_bytes: usize,
    /// The length of the rate window.
    pub rate_window: Duration,
}

impl Config {
    /// A configuration whose store is `store` and whose bounds are the owner values.
    pub fn at(store: impl Into<PathBuf>) -> Config {
        Config {
            store: store.into(),
            blob_max_bytes: DRAFT_MAX_BYTES + DRAFT_BLOB_HEADROOM,
            rate_window: RATE_WINDOW,
        }
    }

    /// The configuration of a process: the store is `LEXLEAN_MCP_STORE`, else
    /// `lexlean-mcp/store` below `XDG_DATA_HOME` or `~/.local/share`, else a directory of
    /// the working directory.
    pub fn from_environment() -> Config {
        let variable = |name: &str| std::env::var_os(name).filter(|value| !value.is_empty());
        let store = if let Some(path) = variable("LEXLEAN_MCP_STORE") {
            PathBuf::from(path)
        } else if let Some(data) = variable("XDG_DATA_HOME") {
            PathBuf::from(data).join("lexlean-mcp").join("store")
        } else if let Some(home) = variable("HOME") {
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("lexlean-mcp")
                .join("store")
        } else {
            PathBuf::from("lexlean-mcp-store")
        };
        Config::at(store)
    }
}
