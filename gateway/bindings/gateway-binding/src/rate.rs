//! The rate counter: how many counted requests the client has made in the current window.
//!
//! The core is a pure function of its request frame, so it cannot count. The adapter asks
//! this host once for every request that reaches the core, with the flag that says whether
//! the request is a counted one, and puts the answer in the frame, where the core compares
//! it with its limit (`rate.tool_calls`). The adapter gives a host no client identity beyond
//! the flag, and the gateway listens on one loopback interface, so there is one window for
//! the process. The window is fixed: it opens at the first counted request and closes
//! `rate.window_ms` later.

use std::time::{Duration, Instant};

/// The counter of one process.
#[derive(Debug, Default)]
pub struct Counter {
    window: Option<(Instant, u32)>,
}

impl Counter {
    /// The number of counted requests made in the window that is open at `now`, before this
    /// request, and then this request counted when `counted` is true.
    pub fn answer(&mut self, counted: bool, now: Instant, window: Duration) -> u32 {
        let (opened, calls) = match self.window {
            Some((opened, calls)) if now.saturating_duration_since(opened) < window => {
                (opened, calls)
            }
            _ => (now, 0),
        };
        self.window = Some((
            opened,
            if counted {
                calls.saturating_add(1)
            } else {
                calls
            },
        ));
        calls
    }
}
