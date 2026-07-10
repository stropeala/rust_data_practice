use std::env::var;
use std::path::PathBuf;

use anyhow::Context;
use rusqlite::Connection;

use crate::data::{data_file, get_data};

pub fn get_conn() -> anyhow::Result<Connection> {
    let db = data_file("parkinglot.db")?;
    let conn = Connection::open(&db).context("Cound not connect to DB!")?;
    Ok(conn)
}

fn create_table() -> anyhow::Result<()> {
    let conn = get_conn()?;
    let schema = get_data(&PathBuf::from(
        var("SQL_SCHEMA").context("Could not get SQL Schema path from env!")?,
    ))?;

    conn.execute(&schema, ())
        .context("Could not create Clients Table!")?;
    Ok(())
}

pub fn is_db() -> anyhow::Result<bool> {
    let db = data_file("parkinglot.db")?;
    if !db.exists() {
        create_table()?;
    }
    Ok(true)
}
