use std::collections::BTreeMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use anyhow::Context;
use chrono::{Duration, NaiveDateTime};
use rand::{RngExt, rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::clients::read_clients;
use crate::data::data_file;

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    entry_time: NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    exit_time: Option<NaiveDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

fn rng_datetime_for_simulation() -> (i32, i32, i32, i32, i32) {
    let month = rng().random_range(1..13);
    let day = rng().random_range(1..29);
    let hour = rng().random_range(0..=24);
    let minute = rng().random_range(0..60);
    let second = rng().random_range(0..60);
    (month, day, hour, minute, second)
}

pub fn read_timers(timers_file_path: &Path) -> Result<BTreeMap<usize, Timer>, anyhow::Error> {
    if !timers_file_path.exists() {
        return Ok(BTreeMap::new());
    }

    let timers_string = read_to_string(timers_file_path).context("Could not read Timers file!")?;
    if timers_string.trim().is_empty() {
        return Ok(BTreeMap::new());
    }

    let timers = from_str(&timers_string).context("Could not deserialize from Timers string!")?;
    Ok(timers)
}

pub fn add_entry_time() -> Result<(), anyhow::Error> {
    let clients_file = data_file("clients.json").context("Could not get Clients file!")?;
    let clients = read_clients(&clients_file).context("Could not read Clients file!")?;

    let entry_time_format = "%Y-%m-%d %H:%M:%S";
    let mut timers: BTreeMap<usize, Timer> = BTreeMap::new();
    for client_id in clients.keys().copied() {
        let (month, day, hour, minute, second) = rng_datetime_for_simulation();
        let entry_time_string = format!("2026-{month}-{day:02} {hour:02}:{minute:02}:{second:02}");
        let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, entry_time_format)
            .context("Could not get entry NaiveDateTime from string")?;
        let timer = Timer {
            entry_time,
            exit_time: None,
            duration: None,
        };
        timers.insert(client_id, timer);
    }

    let timers_file = data_file("timers.json").context("Could not get Timers file!")?;
    let json = to_string_pretty(&timers).context("Could not serialize Entry Timers data!")?;
    write(timers_file, &json).context("Could not write Entry into Timers file!")?;
    Ok(())
}

pub fn add_exit_time() -> Result<(), anyhow::Error> {
    let timers_file = data_file("timers.json").context("Could not get Timers file!")?;
    let mut timers = read_timers(&timers_file).context("Could not read Timers file!")?;

    for timer in timers.values_mut() {
        if timer.exit_time.is_some() {
            continue;
        }
        let hours: i64 = rng().random_range(1..97);
        timer.duration = Some(hours);
        timer.exit_time = Some(timer.entry_time + Duration::hours(hours));
    }

    let json = to_string_pretty(&timers).context("Could not serialize Exit Timers data!")?;
    write(timers_file, &json).context("Could not write Exit into Timers file!")?;
    Ok(())
}
