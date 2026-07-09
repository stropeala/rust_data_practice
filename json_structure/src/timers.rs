use std::collections::BTreeMap;
use std::env::var;
use std::fs::{DirBuilder, File, read_to_string, write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::Context;
use chrono::NaiveDateTime;
use dotenvy::dotenv;
use rand::{RngExt, rng};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::clients::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    entry_time: NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    exit_time: Option<NaiveDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}

fn rng_datetime_for_simulation() -> (i32, i32, i32, i32, i32) {
    let month = rng().random_range(1..13);
    let day = rng().random_range(1..29);
    let hour = rng().random_range(1..24);
    let minute = rng().random_range(1..60);
    let second = rng().random_range(1..60);
    (month, day, hour, minute, second)
}

pub fn timers_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join("timers.json"))
}

pub fn read_timers(timers_file_path: &Path) -> Result<BTreeMap<usize, Timer>, anyhow::Error> {
    let timers_string = read_to_string(timers_file_path).context("Could not read Timers file!")?;
    if timers_string.trim().is_empty() {
        return Ok(BTreeMap::new());
    }

    let timers = from_str(&timers_string).context("Could not deserialize from Timers string!")?;
    Ok(timers)
}

pub fn add_entry_time(clients: BTreeMap<usize, Client>) -> Result<(), anyhow::Error> {
    let timers_file = timers_file_path().context("Could not get Timers file!")?;
    if !timers_file.exists() {
        File::create(&timers_file).context("Could not create Timers file!")?;
    }

    let mut timers: BTreeMap<usize, Timer> = BTreeMap::new();
    for client_id in clients.keys().copied() {
        let (month, day, hour, minute, second) = rng_datetime_for_simulation();
        let entry_time_string = format!("2026-{month}-{day} {hour}:{minute}:{second:02}");
        let entry_time_format = "%Y-%m-%d %H:%M:%S";
        let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, entry_time_format)
            .context("Could not get entry NaiveDateTime from string")?;
        let timer = Timer {
            entry_time,
            exit_time: None,
            duration: None,
        };
        timers.insert(client_id, timer);
    }

    let json = to_string_pretty(&timers).context("Could not serialize Entry Timers data!")?;
    write(timers_file, &json).context("Could not write Entry into Timers file!")?;
    Ok(())
}

pub fn add_exit_time() -> Result<(), anyhow::Error> {
    let timers_file = timers_file_path().context("Could not get Timers file!")?;
    let mut timers = read_timers(&timers_file).context("Could not read Timers file!")?;

    for timer in timers.values_mut() {
        if timer.exit_time.is_some() {
            continue;
        }
        let rng_hours_for_simulation = rng().random_range(1..97);
        timer.duration = Some(format!("{rng_hours_for_simulation} hours"));
        timer.exit_time = Some(timer.entry_time + Duration::from_hours(rng_hours_for_simulation));
    }

    let json = to_string_pretty(&timers).context("Could not serialize Exit Timers data!")?;
    write(timers_file, &json).context("Could not write Exit into Timers file!")?;
    Ok(())
}
