use std::collections::BTreeMap;
use std::env::var;
use std::fs::{File, write};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use chrono::Local;
use dotenvy::dotenv;
use rand::{RngExt, rng};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use crate::clients::{clients_file_path, read_clients};

#[derive(Serialize, Deserialize, Debug)]
struct Timer {
    entry_date: String,
    exit_date: String,
}

impl Timer {
    fn new(entry_date: String, exit_date: String) -> Self {
        Self {
            entry_date,
            exit_date,
        }
    }
}

fn timer() -> Duration {
    let mut rng = rng();
    let hours = rng.random_range(1..=72);
    Duration::from_hours(hours)
}

fn timers_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir = var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?;
    Ok(PathBuf::from(data_dir).join("timers.json"))
}

pub fn add_timer_to_clients() -> Result<(), anyhow::Error> {
    let clients_file = clients_file_path()?;
    let clients = read_clients(&clients_file)?;
    let clients_id: Vec<usize> = clients.keys().copied().collect();

    let timers_file = timers_file_path()?;
    if !timers_file.exists() {
        File::create(&timers_file).context("Could not create Timers file")?;
    }

    /*
     *
     * TODO: add entry date when client is generated, then run the timer!
     *
     */

    let mut timers: BTreeMap<usize, Timer> = BTreeMap::new();
    let entry_date = Local::now();
    for client in clients_id {
        let exit_date = entry_date + timer();
        let timer = Timer::new(entry_date.to_string(), exit_date.to_string());
        timers.insert(client, timer);
    }

    let json = to_string_pretty(&timers)?;
    write(timers_file, &json)?;
    Ok(())
}
