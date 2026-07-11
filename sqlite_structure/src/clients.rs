use anyhow::Context;
use sqlx::SqlitePool;

use crate::db::{create_table, get_entry_time_by_phone_nr, is_table};
use crate::timers::{entry_timer, exit_timer};

pub async fn add_client(
    pool: &SqlitePool,
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> anyhow::Result<()> {
    if !is_table(pool).await? {
        create_table(pool).await?;
    }

    let entry_time = entry_timer()?;
    let sql_query = "INSERT INTO Clients
        (surname, name, telephone_number, city_of_residence,
         datetime_entrance, datetime_exit, hours_parked, pariah)
        VALUES
        (?, ?, ?, ?, ?, NULL, 0, 0)";
    sqlx::query(sql_query)
        .bind(surname)
        .bind(name)
        .bind(telephone_number)
        .bind(city)
        .bind(entry_time.to_string())
        .execute(pool)
        .await
        .context("Failed adding client to table!")?;
    Ok(())
}

pub async fn add_client_exit_time(pool: &SqlitePool, client_phone_nr: &str) -> anyhow::Result<()> {
    let entry_time = get_entry_time_by_phone_nr(pool, client_phone_nr).await?;
    let exit_time = exit_timer(entry_time)?;
    let duration = (exit_time - entry_time).num_hours();
    let pariah = duration > 72;

    let sql_query = "UPDATE Clients SET datetime_exit = ?, hours_parked = ?, pariah = ? WHERE telephone_number = ?";
    sqlx::query(sql_query)
        .bind(exit_time.to_string())
        .bind(duration)
        .bind(pariah)
        .bind(client_phone_nr)
        .execute(pool)
        .await
        .context("Failed adding exit time!")?;
    Ok(())
}
