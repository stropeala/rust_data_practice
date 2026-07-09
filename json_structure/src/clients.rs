use std::collections::BTreeMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

use crate::data::data_file;

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

pub fn read_clients(clients_file_path: &Path) -> Result<BTreeMap<usize, Client>, anyhow::Error> {
    if !clients_file_path.exists() {
        return Ok(BTreeMap::new());
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
    let clients_file = data_file("clients.json").context("Could not get Clients file!")?;
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
    Ok(())
}
