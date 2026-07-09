use std::collections::BTreeMap;
use std::fs::write;

use anyhow::{Context, anyhow};
use serde_json::to_string_pretty;

use crate::clients::read_clients;
use crate::data::data_file;
use crate::timers::{Timer, read_timers};

fn change_pariah(ids: &[usize]) -> Result<(), anyhow::Error> {
    if ids.is_empty() {
        return Ok(());
    }

    let clients_file = data_file("clients.json").context("Could not get Clients file!")?;
    let mut clients = read_clients(&clients_file).context("Could not read Clients file!")?;

    for id in ids {
        if let Some(client) = clients.get_mut(id) {
            client.pariah = true;
        }
    }

    let json = to_string_pretty(&clients).context("Could not serialize Clients data!")?;
    write(clients_file, &json).context("Could not change Pariah for Clients file!")?;
    Ok(())
}

pub fn organizer() -> Result<(), anyhow::Error> {
    let timers_file = data_file("timers.json").context("Could not get Timers file!")?;
    let timers = read_timers(&timers_file).context("Could not read Timers file!")?;

    let mut under_two_hours: BTreeMap<usize, Timer> = BTreeMap::new();
    let under_two_hours_file =
        data_file("under_two_hours.json").context("Could not get Under Two Hours file!")?;

    let mut in_between: BTreeMap<usize, Timer> = BTreeMap::new();
    let in_between_file = data_file("in_between.json").context("Could not get In Between file!")?;

    let mut above_three_days: BTreeMap<usize, Timer> = BTreeMap::new();
    let above_three_days_file =
        data_file("above_three_days.json").context("Could not get Above Three Days file!")?;

    let mut pariah_ids: Vec<usize> = Vec::new();
    for (client_id, timer) in timers {
        let hours = timer
            .duration
            .ok_or_else(|| anyhow!("Timer for client {client_id} has no duration"))?;

        if hours <= 2 {
            under_two_hours.insert(client_id, timer);
        } else if hours >= 72 {
            above_three_days.insert(client_id, timer);
            pariah_ids.push(client_id);
        } else {
            in_between.insert(client_id, timer);
        }
    }

    change_pariah(&pariah_ids).context("Could not change Pariah for Clients!")?;
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
