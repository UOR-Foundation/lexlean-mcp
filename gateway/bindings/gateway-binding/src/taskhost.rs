//! What the core cannot compute for the tasks it serves: the time, an identifier and the table.
//!
//! The core keeps no state (TASKHOST.md). The adapter asks this host for the clock, an
//! identifier a new task may take and the table of tasks before each request, and puts them in
//! the frame; the core asks for `task.store` when it changed the table. The table is the bytes
//! the core last stored, in memory: a restart empties it, and the handles of the tasks it held
//! are then unknown.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rustix::rand::{getrandom, GetRandomFlags};

/// The clock of a host: a counter of milliseconds that never goes back, and the instant it
/// stands for in the calendar.
#[derive(Debug)]
pub struct Clock {
    started: Instant,
    wall_ms: u64,
}

impl Default for Clock {
    fn default() -> Clock {
        let wall = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_millis());
        Clock {
            started: Instant::now(),
            wall_ms: u64::try_from(wall).unwrap_or(u64::MAX),
        }
    }
}

impl Clock {
    /// The counter at `at` and the same instant as ISO 8601 text.
    pub fn at(&self, at: Instant) -> (u64, String) {
        let ms = u64::try_from(at.saturating_duration_since(self.started).as_millis())
            .unwrap_or(u64::MAX);
        (ms, iso(self.wall_ms.saturating_add(ms)))
    }
}

/// The instant `ms` milliseconds after 1970-01-01T00:00:00Z as `YYYY-MM-DDTHH:MM:SS.mmmZ`.
pub fn iso(ms: u64) -> String {
    let milliseconds = ms % 1_000;
    let seconds = ms / 1_000;
    let days = i64::try_from(seconds / 86_400).unwrap_or(i64::MAX / 2);
    let of_day = seconds % 86_400;
    // Howard Hinnant's civil_from_days: the date of a day count, in the proleptic Gregorian
    // calendar.
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_part = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_part + 2) / 5 + 1;
    let month = if month_part < 10 {
        month_part + 3
    } else {
        month_part - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{milliseconds:03}Z",
        of_day / 3_600,
        of_day % 3_600 / 60,
        of_day % 60
    )
}

/// An identifier: 32 lowercase hexadecimal digits from the random source of the system, or
/// `None` when it gives none.
pub fn fresh_id() -> Option<String> {
    let mut bytes = [0u8; 16];
    let mut filled = 0;
    while filled < bytes.len() {
        match getrandom(&mut bytes[filled..], GetRandomFlags::empty()) {
            Ok(count) if count > 0 => filled += count,
            _ => return None,
        }
    }
    Some(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
