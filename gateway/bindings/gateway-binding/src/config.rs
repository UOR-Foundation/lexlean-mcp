//! Where the host keeps its state, and the owner values that bound it.

use std::ffi::OsString;
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

pub use gateway_signer::service::{SIGNER_VARIABLE, SIGNING_KEY_VARIABLE};

/// The tasks that run at once: the owner value `verify.concurrency`.
pub const VERIFY_CONCURRENCY: usize = 1;

/// The width of one verification: the owner value `verify.width`.
pub const VERIFY_WIDTH: u32 = 1;

/// The memory budget of one verification, in KiB: the owner value `verify.memory_kib`.
pub const VERIFY_MEMORY_KIB: u64 = 12_582_912;

/// How long a process may take to answer a cancel before it is killed.
pub const CANCEL_GRACE: Duration = Duration::from_secs(30);

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
    /// The socket of `gateway-signer`, named by `LEXLEAN_MCP_SIGNER`. A verification is not started
    /// without a signer whose key loads, because its record would be unsigned. There is no setting
    /// for the key: this process does not hold it.
    pub signer: Option<PathBuf>,
    /// The program that verifies, one process for each task: `LEXLEAN_MCP_VERIFIER`, else
    /// `gateway-verifier` beside the running executable. A verification is not started without it.
    pub verifier_program: Option<PathBuf>,
    /// The tasks that run at once.
    pub verify_concurrency: usize,
    /// The width of one verification, given to its process in `LEXLEAN_VERIFY_WIDTH`.
    pub verify_width: u32,
    /// The memory budget of one verification, in KiB, given in `LEXLEAN_VERIFY_MEMORY_KIB`.
    pub verify_memory_kib: u64,
    /// How long a process may take to answer a cancel before it is killed.
    pub cancel_grace: Duration,
}

impl Config {
    /// A configuration whose store is `store` and whose bounds are the owner values.
    pub fn at(store: impl Into<PathBuf>) -> Config {
        Config {
            store: store.into(),
            blob_max_bytes: DRAFT_MAX_BYTES + DRAFT_BLOB_HEADROOM,
            rate_window: RATE_WINDOW,
            signer: None,
            verifier_program: None,
            verify_concurrency: VERIFY_CONCURRENCY,
            verify_width: VERIFY_WIDTH,
            verify_memory_kib: VERIFY_MEMORY_KIB,
            cancel_grace: CANCEL_GRACE,
        }
    }

    /// This configuration with the verifier program `program`.
    pub fn with_verifier(mut self, program: impl Into<PathBuf>) -> Config {
        self.verifier_program = Some(program.into());
        self
    }

    /// This configuration with `slots` tasks at once.
    pub fn with_verify_concurrency(mut self, slots: usize) -> Config {
        self.verify_concurrency = slots;
        self
    }

    /// This configuration with `grace` for a process to answer a cancel.
    pub fn with_cancel_grace(mut self, grace: Duration) -> Config {
        self.cancel_grace = grace;
        self
    }

    /// This configuration with the signer at the socket `socket`.
    pub fn with_signer(mut self, socket: impl Into<PathBuf>) -> Config {
        self.signer = Some(socket.into());
        self
    }

    /// The key history: `keys/history` below the store.
    pub fn history_path(&self) -> PathBuf {
        self.store.join("keys").join("history")
    }

    /// The configuration of a process: the store is `LEXLEAN_MCP_STORE`, else
    /// `lexlean-mcp/store` below `XDG_DATA_HOME` or `~/.local/share`, else a directory of
    /// the working directory; the socket of the signer is `LEXLEAN_MCP_SIGNER`.
    pub fn from_environment() -> Config {
        let mut config = Config::from_variables(|name| std::env::var_os(name));
        if config.verifier_program.is_none() {
            config.verifier_program = std::env::current_exe()
                .ok()
                .and_then(|exe| {
                    exe.parent()
                        .map(|directory| directory.join("gateway-verifier"))
                })
                .filter(|program| program.is_file());
        }
        config
    }

    /// The configuration that the variables `lookup` gives, as `from_environment` reads them;
    /// a variable that is empty is not set.
    pub fn from_variables(lookup: impl Fn(&str) -> Option<OsString>) -> Config {
        let variable = |name: &str| lookup(name).filter(|value| !value.is_empty());
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
        let mut config = Config::at(store);
        config.signer = variable(SIGNER_VARIABLE).map(PathBuf::from);
        config.verifier_program = variable("LEXLEAN_MCP_VERIFIER").map(PathBuf::from);
        config
    }
}
