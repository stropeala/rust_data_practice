use std::env::var;
use std::path::PathBuf;

use anyhow::Context;
use chrono::NaiveDateTime;
use rusqlite::Connection;

use crate::data::{data_file, get_data};
use crate::timers::DATETIME_FORMAT;

pub fn get_conn() -> anyhow::Result<Connection> {
    let db = data_file("parkinglot.db")?;
    let conn = Connection::open(&db).context("Failed connecting to DB!")?;
    Ok(conn)
}

pub fn create_table() -> anyhow::Result<()> {
    let conn = get_conn()?;
    let sql_schema = get_data(&PathBuf::from(
        var("SQL_SCHEMA").context("Failed getting SQL schema path from env!")?,
    ))?;

    if !conn.table_exists(Some("main"), "Clients")? {
        conn.execute(&sql_schema, ())
            .context("Failed creating clients table!")?;
    }
    Ok(())
}

pub fn is_db() -> anyhow::Result<bool> {
    let db = data_file("parkinglot.db")?;
    Ok(db.exists())
}

pub fn get_entry_time_by_phone_nr(client_phone_nr: &str) -> anyhow::Result<NaiveDateTime> {
    let conn = get_conn()?;
    let sql_search_query = "SELECT datetime_entrance FROM Clients WHERE telephone_number=?";

    let entry_time_string: String = conn
        .query_row(sql_search_query, [client_phone_nr], |row| row.get(0))
        .context("Failed getting entry time with client phone nr.!")?;
    let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, DATETIME_FORMAT)?;
    Ok(entry_time)
}

pub fn create_organizer_tables() -> anyhow::Result<()> {
    let conn = get_conn()?;
    let sql_queries = [
        "CREATE TABLE under_2_hours AS SELECT * FROM Clients WHERE hours_parked <= 2;",
        "CREATE TABLE over_2_hours AS SELECT * FROM Clients WHERE hours_parked > 2 AND hours_Parked <= 72;",
        "CREATE TABLE over_3_days AS SELECT * FROM Clients WHERE hours_parked > 72;",
    ];
    for query in sql_queries {
        conn.execute(query, ()).context("Failed organizing data!")?;
    }
    Ok(())
}

fn _get_client_id_by_phone_nr(client_phone_nr: &str) -> anyhow::Result<i64> {
    let conn = get_conn()?;
    let sql_search_query = "SELECT id FROM Clients WHERE telephone_number=?";

    let id = conn.query_row(sql_search_query, [client_phone_nr], |row| row.get(0))?;
    Ok(id)
}

fn _get_entry_time_by_id(client_id: i64) -> anyhow::Result<NaiveDateTime> {
    let conn = get_conn()?;
    let sql_search_query = "SELECT datetime_entrance FROM Clients WHERE id=?";

    let entry_time_string: String =
        conn.query_row(sql_search_query, [client_id], |row| row.get(0))?;
    let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, DATETIME_FORMAT)?;
    Ok(entry_time)
}
