use anyhow::Context;

use crate::db::{create_table, get_conn, get_entry_time_by_phone_nr, is_db};
use crate::timers::{entry_timer, exit_timer};

pub fn add_client(
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> anyhow::Result<()> {
    if !is_db()? {
        create_table()?;
    }

    let entry_time = entry_timer()?;
    let sql_insert_client = "INSERT INTO Clients
    ('surname', 'name', 'telephone_number','city_of_residence',
    'datetime_entrance', 'datetime_exit','hours_Parked', 'pariah')
    VALUES
    (?, ?, ?, ?, ?, 'None', '0', 'False')";

    let conn = get_conn()?;
    conn.execute(
        sql_insert_client,
        (
            surname,
            name,
            telephone_number,
            city,
            entry_time.to_string(),
        ),
    )
    .context("Failed adding client to table!")?;
    Ok(())
}

pub fn add_client_exit_time(client_phone_nr: &str) -> anyhow::Result<()> {
    let entry_time = get_entry_time_by_phone_nr(client_phone_nr)?;
    let exit_time = exit_timer(entry_time)?;
    let duration = (exit_time - entry_time).num_hours();

    let conn = get_conn()?;
    if duration > 72 {
        let sql_update_query =
            "UPDATE Clients SET datetime_exit=?, hours_Parked=?, pariah=? WHERE telephone_number=?";
        conn.execute(
            sql_update_query,
            (
                exit_time.to_string(),
                duration.to_string(),
                "True",
                client_phone_nr,
            ),
        )
        .context("Failed updating exit time!")?;
    }
    let sql_update_query =
        "UPDATE Clients SET datetime_exit=?, hours_Parked=? WHERE telephone_number=?";
    conn.execute(
        sql_update_query,
        (exit_time.to_string(), duration.to_string(), client_phone_nr),
    )
    .context("Failed updating exit time!")?;
    Ok(())
}
