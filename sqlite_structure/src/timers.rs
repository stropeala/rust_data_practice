use std::time::Duration;

use anyhow::Context;
use chrono::NaiveDateTime;
use rand::{RngExt, rng};

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn rng_datetime_for_simulation() -> (u32, u32, u32, u32, u32) {
    let month = rng().random_range(1..=12);
    let day = rng().random_range(1..=28);
    let hour = rng().random_range(0..=23);
    let minute = rng().random_range(0..=59);
    let second = rng().random_range(0..=59);
    (month, day, hour, minute, second)
}

pub fn entry_timer() -> anyhow::Result<NaiveDateTime> {
    let (month, day, hour, minute, second) = rng_datetime_for_simulation();
    let entry_time_string = format!("2026-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}");
    let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, DATETIME_FORMAT)
        .context("Failed parsing entry time from string")?;
    Ok(entry_time)
}

pub fn exit_timer(entry_time: NaiveDateTime) -> anyhow::Result<NaiveDateTime> {
    let duration = Duration::from_hours(rng().random_range(1..=96));
    Ok(entry_time + duration)
}
