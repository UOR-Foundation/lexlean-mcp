//! TH-02 to TH-04: the entries of the host that the protocol core asks for beside the rate
//! counter: the time, a fresh identifier and the table of tasks.
//!
//! The expected values are made here: the instants are known dates of the calendar, the
//! identifiers are judged by their shape and by how often they repeat, and the table by the
//! bytes that were given.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use gateway_binding::{iso, Config, Native};
use gateway_host_api::protocol::{clock_now, task_id, task_store, task_table, Host, HostError};

fn host() -> (Native, tempfile::TempDir) {
    let directory = tempfile::tempdir().expect("a directory");
    let host = Native::new(Config::at(directory.path().join("store")));
    (host, directory)
}

fn answered<T, F>(result: Result<T, HostError<F>>) -> T {
    match result {
        Ok(done) => done,
        Err(HostError::Failed(_)) => panic!("the host refused"),
        Err(HostError::Fault) => panic!("the host faulted"),
    }
}

/// Days from 1970-01-01 to a civil date, written again here from Howard Hinnant's algorithm so
/// that the text of the host is read back by code that is not its own.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let yoe = year.rem_euclid(400);
    let doy = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The milliseconds since the epoch that `YYYY-MM-DDTHH:MM:SS.mmmZ` names.
fn epoch_ms(text: &str) -> i64 {
    let number = |range: std::ops::Range<usize>| -> i64 { text[range].parse().expect("digits") };
    assert_eq!(text.len(), 24, "{text}");
    assert_eq!(&text[4..5], "-");
    assert_eq!(&text[7..8], "-");
    assert_eq!(&text[10..11], "T");
    assert_eq!(&text[13..14], ":");
    assert_eq!(&text[16..17], ":");
    assert_eq!(&text[19..20], ".");
    assert_eq!(&text[23..24], "Z");
    let days = days_from_civil(number(0..4), number(5..7), number(8..10));
    ((days * 24 + number(11..13)) * 60 + number(14..16)) * 60_000
        + number(17..19) * 1_000
        + number(20..23)
}

// TH-02

#[test]
fn the_text_of_an_instant_is_iso_8601_in_utc_with_milliseconds_th_02() {
    for (ms, text) in [
        (0, "1970-01-01T00:00:00.000Z"),
        (1_700_000_000_123, "2023-11-14T22:13:20.123Z"),
        (951_782_400_000, "2000-02-29T00:00:00.000Z"),
        (1_709_164_799_999, "2024-02-28T23:59:59.999Z"),
        (1_709_164_800_000, "2024-02-29T00:00:00.000Z"),
        (4_102_444_799_999, "2099-12-31T23:59:59.999Z"),
    ] {
        assert_eq!(iso(ms), text, "{ms} ms");
        assert_eq!(
            epoch_ms(text),
            i64::try_from(ms).expect("a count"),
            "{text} read back"
        );
    }
}

#[test]
fn the_counter_never_goes_back_and_the_text_agrees_with_it_th_02() {
    let (host, _directory) = host();
    let start = Instant::now();
    let mut earlier: Option<(u64, String)> = None;
    for milliseconds in [0u64, 1, 1, 250, 1_500, 86_400_000, 86_400_001] {
        let (ms, text) = host.clock(start + Duration::from_millis(milliseconds));
        if let Some((before, before_text)) = &earlier {
            assert!(ms >= *before, "{ms} after {before}");
            assert_eq!(
                epoch_ms(&text) - epoch_ms(before_text),
                i64::try_from(ms - before).expect("a count"),
                "the text agrees with the counter over the interval"
            );
        }
        earlier = Some((ms, text));
    }
}

#[test]
fn the_effect_answers_the_time_th_02() {
    let (mut host, _directory) = host();
    let mut last = 0;
    for _ in 0..50 {
        let done = answered(host.clock_now(clock_now::Request {}));
        assert!(done.ms >= last, "the counter went back");
        last = done.ms;
        let wall = epoch_ms(&done.iso);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("after the epoch")
            .as_millis();
        assert!(
            (i128::from(wall) - i128::try_from(now).expect("a count")).abs() < 5_000,
            "{} is not about now",
            done.iso
        );
    }
}

// TH-03

#[test]
fn an_identifier_is_32_lowercase_hexadecimal_digits_and_never_repeats_th_03() {
    let (mut host, _directory) = host();
    let mut seen = HashSet::new();
    for _ in 0..1_000 {
        let id = answered(host.task_id(task_id::Request {})).id;
        assert_eq!(id.len(), 32, "{id}");
        assert!(
            id.bytes()
                .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f')),
            "{id}"
        );
        assert!(seen.insert(id.clone()), "{id} came twice");
    }
}

#[test]
fn hosts_do_not_share_identifiers_and_none_is_made_from_the_clock_th_03() {
    let mut seen = HashSet::new();
    let mut hosts = Vec::new();
    for _ in 0..10 {
        hosts.push(host());
    }
    for (host, _directory) in &mut hosts {
        for _ in 0..100 {
            let id = answered(host.task_id(task_id::Request {})).id;
            assert!(seen.insert(id.clone()), "{id} came from two hosts");
            // The counter and the text of the time, as hexadecimal, are not the identifier or a
            // prefix or a suffix of it.
            let now = answered(host.clock_now(clock_now::Request {}));
            let counter = format!("{:x}", now.ms);
            assert!(
                !id.contains(&counter) || counter.len() < 4,
                "{id} holds the counter {counter}"
            );
            assert!(
                !id.starts_with(&format!("{:016x}", now.ms)),
                "{id} begins with the counter"
            );
        }
    }
}

// TH-04

fn table(host: &mut Native) -> Vec<u8> {
    answered(host.task_table(task_table::Request {})).table
}

fn store(host: &mut Native, bytes: &[u8]) {
    answered(host.task_store(task_store::Request {
        table: bytes.to_vec(),
    }));
}

#[test]
fn the_table_is_empty_until_one_is_stored_and_then_it_is_what_was_stored_th_04() {
    let (mut host, _directory) = host();
    assert_eq!(table(&mut host), Vec::<u8>::new());
    store(&mut host, b"[]");
    assert_eq!(table(&mut host), b"[]");
    store(&mut host, br#"[{"task":{"taskId":"a"}}]"#);
    assert_eq!(table(&mut host), br#"[{"task":{"taskId":"a"}}]"#);
}

#[test]
fn a_store_replaces_the_whole_table_and_an_empty_store_empties_it_th_04() {
    let (mut host, _directory) = host();
    store(&mut host, b"[1,2,3]");
    store(&mut host, b"[4]");
    assert_eq!(
        table(&mut host),
        b"[4]",
        "the second store replaced the first"
    );
    store(&mut host, b"");
    assert_eq!(
        table(&mut host),
        Vec::<u8>::new(),
        "an empty store empties it"
    );
}

#[test]
fn bytes_of_every_value_and_the_largest_table_come_back_unchanged_th_04() {
    let (mut host, _directory) = host();
    let every: Vec<u8> = (0..=255u8).cycle().take(1_000).collect();
    store(&mut host, &every);
    assert_eq!(table(&mut host), every);
    // The declaration allows a table of 524296 bytes in either direction.
    let largest: Vec<u8> = (0..524_296u32)
        .map(|at| u8::try_from(at.wrapping_mul(31) % 251).expect("a byte"))
        .collect();
    store(&mut host, &largest);
    assert_eq!(table(&mut host), largest);
}

#[test]
fn two_hosts_do_not_share_a_table_th_04() {
    let (mut one, _first) = host();
    let (mut other, _second) = host();
    store(&mut one, b"[1]");
    assert_eq!(table(&mut other), Vec::<u8>::new());
}
