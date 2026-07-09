use std::collections::BTreeMap;
use std::env::var;
use std::fs::{DirBuilder, File, write};
use std::path::PathBuf;

use anyhow::Context;
use dotenvy::dotenv;
use serde_json::to_string_pretty;

use crate::timers::{Timer, read_timers, timers_file_path};

fn get_int(input: &str) -> Result<u32, anyhow::Error> {
    let mut output_string = String::new();
    for char in input.chars() {
        if char.is_numeric() {
            output_string.push(char);
        }
    }

    let output = output_string
        .parse::<u32>()
        .context("Could not parse for int!")?;
    Ok(output)
}

// fn change_pariah() {}

fn under_two_hours_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join("under_two_hours.json"))
}

fn in_between_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join("in_between.json"))
}

fn above_three_days_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join("above_three_days.json"))
}

pub fn organizer() -> Result<(), anyhow::Error> {
    let timers_file = timers_file_path().context("Could not get Timers file!")?;
    let timers = read_timers(&timers_file).context("Could not read Timers file!")?;

    let mut under_two_hours: BTreeMap<usize, Timer> = BTreeMap::new();
    let under_two_hours_file =
        under_two_hours_file_path().context("Could not get Under Two Hours file!")?;
    if !under_two_hours_file.exists() {
        File::create(&under_two_hours_file)?;
    }

    let mut in_between: BTreeMap<usize, Timer> = BTreeMap::new();
    let in_between_file = in_between_file_path().context("Could not get In Between file!")?;
    if !in_between_file.exists() {
        File::create(&in_between_file)?;
    }

    let mut above_three_days: BTreeMap<usize, Timer> = BTreeMap::new();
    let above_three_days_file =
        above_three_days_file_path().context("Could not get Above Three Days file!")?;
    if !above_three_days_file.exists() {
        File::create(&above_three_days_file)?;
    }

    for (client_id, timer) in timers {
        if get_int(&timer.duration.to_owned().unwrap()).context("Could not parse Duration!")? <= 2 {
            under_two_hours.insert(client_id, timer);
        } else if get_int(&timer.duration.to_owned().unwrap())
            .context("Could not parse Duration!")?
            >= 72
        {
            above_three_days.insert(client_id, timer);
        } else {
            in_between.insert(client_id, timer);
        }
    }

    let under_two_hours_json = to_string_pretty(&under_two_hours)?;
    let in_between_json = to_string_pretty(&in_between)?;
    let above_three_days_json = to_string_pretty(&above_three_days)?;
    write(under_two_hours_file, &under_two_hours_json)?;
    write(in_between_file, &in_between_json)?;
    write(above_three_days_file, &above_three_days_json)?;

    Ok(())
}
