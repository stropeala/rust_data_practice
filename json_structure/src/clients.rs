use std::collections::BTreeMap;
use std::env::var;
use std::fs::{DirBuilder, File, read_to_string, write};
use std::path::{Path, PathBuf};

use anyhow::Context;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::organizers::organizer;
use crate::timers::{add_entry_time, add_exit_time};

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    surname: String,
    name: String,
    telephone_number: String,
    city: String,
    pub pariah: bool,
}

impl Client {
    fn new(surname: String, name: String, telephone_number: String, city: String) -> Self {
        Self {
            surname,
            name,
            telephone_number,
            city,
            pariah: false,
        }
    }
}

pub fn clients_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir =
        PathBuf::from(var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?);
    if !data_dir.exists() {
        DirBuilder::new()
            .create(&data_dir)
            .context("Could not create Data directory!")?;
    }
    Ok(data_dir.join("clients.json"))
}

pub fn read_clients(clients_file_path: &Path) -> Result<BTreeMap<usize, Client>, anyhow::Error> {
    if !clients_file_path.exists() {
        File::create(clients_file_path).context("Could not create Clients file!")?;
    }

    let clients_string =
        read_to_string(clients_file_path).context("Could not read data from the Clients file!")?;
    if clients_string.trim().is_empty() {
        return Ok(BTreeMap::new());
    }

    let clients =
        from_str(&clients_string).context("Could not deserialize from Clients string!")?;
    Ok(clients)
}

pub fn add_client(
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> Result<(), anyhow::Error> {
    let clients_file = clients_file_path().context("Could not get Clients file!")?;
    let mut clients = read_clients(&clients_file).context("Could not read Clients file!")?;

    let client_id = clients.keys().max().copied().unwrap_or(0);
    let client = Client::new(
        surname.to_string(),
        name.to_string(),
        telephone_number.to_string(),
        city.to_string(),
    );
    clients.insert(client_id + 1, client);

    let json = to_string_pretty(&clients).context("Could not serialize Clients data!")?;
    write(clients_file, json).context("Could not write into Clients file!")?;
    add_entry_time(clients).context("Could not add Entry Time!")?;
    add_exit_time().context("Could not add Exit Time!")?;
    organizer().context("Could not organize data!")?;
    Ok(())
}
