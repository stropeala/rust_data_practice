use std::collections::HashMap;
use std::fs::{File, metadata, read_to_string, write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

pub static CLIENT_ID: AtomicUsize = AtomicUsize::new(0);

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
        CLIENT_ID.fetch_add(1, Ordering::SeqCst);
        Self {
            surname,
            name,
            telephone_number,
            city,
            pariah: false,
        }
    }
}

fn get_data_dir_path() -> Result<String, anyhow::Error> {
    dotenvy::dotenv().ok();
    Ok(std::env::var("CLIENT_FILE_PATH")?)
}

pub fn add_client(
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> Result<(), anyhow::Error> {
    let client_path = PathBuf::from(get_data_dir_path()?).join("clients.json");
    match File::open(&client_path) {
        Ok(file) => file,
        Err(_) => File::create(&client_path)?,
    };

    let data = read_to_string(&client_path)?;
    let mut clients = Vec::new();
    if metadata(&client_path).unwrap().len() != 0 {
        clients = from_str(&data)?;
    }

    let client = Client::new(
        surname.to_string(),
        name.to_string(),
        telephone_number.to_string(),
        city.to_string(),
    );
    let mut client_plus_id = HashMap::new();
    client_plus_id.insert(CLIENT_ID.load(Ordering::SeqCst), client);
    clients.push(client_plus_id);

    let json = to_string(&clients)?;
    write(client_path, &json)?;
    Ok(())
}
