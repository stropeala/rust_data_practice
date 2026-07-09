use std::collections::BTreeMap;
use std::env::var;
use std::fs::{DirBuilder, File, write};
use std::path::PathBuf;

use anyhow::Context;
use dotenvy::dotenv;
use serde_json::to_string_pretty;

use crate::clients::{clients_file_path, read_clients};
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

fn change_pariah(id: &usize) -> Result<(), anyhow::Error> {
    let clients_file = clients_file_path().context("Could not get Clients file!")?;
    let mut clients = read_clients(&clients_file).context("Could not read Clients file!")?;

    for (client_id, client) in &mut clients {
        if client_id == id {
            client.pariah = true
        }
    }

    let json = to_string_pretty(&clients).context("Could not serialize Pariah data!")?;
    write(clients_file, &json).context("Could not write Pariah into Clients file!")?;
    Ok(())
}

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
        File::create(&under_two_hours_file).context("Could not create Under Two Hours file!")?;
    }

    let mut in_between: BTreeMap<usize, Timer> = BTreeMap::new();
    let in_between_file = in_between_file_path().context("Could not get In Between file!")?;
    if !in_between_file.exists() {
        File::create(&in_between_file).context("Could not create In Between file!")?;
    }

    let mut above_three_days: BTreeMap<usize, Timer> = BTreeMap::new();
    let above_three_days_file =
        above_three_days_file_path().context("Could not get Above Three Days file!")?;
    if !above_three_days_file.exists() {
        File::create(&above_three_days_file).context("Could not create Above Three Days")?;
    }

    for (client_id, timer) in timers {
        if get_int(&timer.duration.to_owned().unwrap()).context("Could not parse Duration!")? <= 2 {
            under_two_hours.insert(client_id, timer);
        } else if get_int(&timer.duration.to_owned().unwrap())
            .context("Could not parse Duration!")?
            >= 72
        {
            above_three_days.insert(client_id, timer);
            change_pariah(&client_id).context("Could not change Pariah for Clients file!")?;
        } else {
            in_between.insert(client_id, timer);
        }
    }

    let under_two_hours_json = to_string_pretty(&under_two_hours)
        .context("Could not serialize Under Two Hours file data!")?;
    let in_between_json =
        to_string_pretty(&in_between).context("Could not serialize In Between data!")?;
    let above_three_days_json = to_string_pretty(&above_three_days)
        .context("Could not serialize Above Three Days data!")?;
    write(under_two_hours_file, &under_two_hours_json)
        .context("Could not write Under Two Hours file file!")?;
    write(in_between_file, &in_between_json).context("Could not write In Between file!")?;
    write(above_three_days_file, &above_three_days_json)
        .context("Could not write Above Three Days file!")?;
    Ok(())
}
