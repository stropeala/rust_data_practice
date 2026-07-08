use std::collections::HashMap;
use std::env::var;
use std::fs::{File, read_to_string, write};
use std::path::{Path, PathBuf};

use anyhow::Context;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
struct Client {
    surname: String,
    name: String,
    telephone_number: String,
    city: String,
    pariah: bool,
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

fn clients_file_path() -> Result<PathBuf, anyhow::Error> {
    dotenv().ok();
    let data_dir = var("DATA_DIR_PATH").context("Could not get Data directory path from env!")?;
    Ok(PathBuf::from(data_dir).join("clients.json"))
}

fn read_clients(clients_file_path: &Path) -> Result<HashMap<usize, Client>, anyhow::Error> {
    if !clients_file_path.exists() {
        File::create(clients_file_path).context("Could not create Clients file!")?;
    }

    let clients =
        read_to_string(clients_file_path).context("Could not read data from the Clients file!")?;
    if clients.trim().is_empty() {
        return Ok(HashMap::new());
    }
    Ok(from_str(&clients)?)
}

pub fn add_client(
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> Result<(), anyhow::Error> {
    let clients_file = clients_file_path()?;
    let mut clients = read_clients(&clients_file)?;

    let client_id = clients.keys().max().unwrap_or(&0);
    let client = Client::new(
        surname.to_string(),
        name.to_string(),
        telephone_number.to_string(),
        city.to_string(),
    );
    clients.insert(client_id + 1, client);

    let json = to_string_pretty(&clients)?;
    write(clients_file, json)?;
    Ok(())
}
