use std::env::var;
use std::path::PathBuf;

use anyhow::Context;
use chrono::NaiveDateTime;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{AssertSqlSafe, SqlitePool};

use crate::data::{data_file, get_data};
use crate::timers::DATETIME_FORMAT;

const DB_NAME: &str = "parkinglot.db";

pub async fn create_sqlite_pool() -> anyhow::Result<SqlitePool> {
    let db_path = data_file(DB_NAME)?;

    let sql_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let sqlite_pool = SqlitePoolOptions::new()
        .connect_with(sql_options)
        .await
        .context("Failed connecting to DB!")?;
    Ok(sqlite_pool)
}

pub async fn is_table(pool: &SqlitePool) -> anyhow::Result<bool> {
    let sql_query = "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'Clients'";
    let table: Option<String> = sqlx::query_scalar(sql_query)
        .fetch_optional(pool)
        .await
        .context("Failed checking for Clients table!")?;
    Ok(table.is_some())
}

pub async fn create_table(pool: &SqlitePool) -> anyhow::Result<()> {
    let sql_schema = get_data(&PathBuf::from(
        var("SQL_SCHEMA").context("Failed getting SQL schema path from env!")?,
    ))?;

    let sql_query = AssertSqlSafe(sql_schema);
    sqlx::raw_sql(sql_query)
        .execute(pool)
        .await
        .context("Failed creating clients table!")?;
    Ok(())
}

pub async fn get_entry_time_by_phone_nr(
    pool: &SqlitePool,
    client_phone_nr: &str,
) -> anyhow::Result<NaiveDateTime> {
    let sql_query = "SELECT datetime_entrance FROM Clients WHERE telephone_number = ?";

    let entry_time_string: String = sqlx::query_scalar(sql_query)
        .bind(client_phone_nr)
        .fetch_one(pool)
        .await
        .context("Failed getting entry time with client phone nr.!")?;
    let entry_time = NaiveDateTime::parse_from_str(&entry_time_string, DATETIME_FORMAT)
        .context("Failed parsing entry time!")?;

    Ok(entry_time)
}

pub async fn create_organizer_tables(pool: &SqlitePool) -> anyhow::Result<()> {
    let table_names = ["under_2_hours", "over_2_hours", "over_3_days"];
    let table_conditions = [
        "hours_parked <= 2",
        "hours_parked > 2 AND hours_parked <= 72",
        "hours_parked > 72",
    ];

    for (name, condition) in table_names.iter().zip(table_conditions) {
        let sql_query =
            format!("CREATE VIEW IF NOT EXISTS {name} AS SELECT * FROM Clients WHERE {condition};");
        sqlx::query(AssertSqlSafe(sql_query))
            .execute(pool)
            .await
            .context("Failed creating organizer views!")?;
    }
    Ok(())
}
