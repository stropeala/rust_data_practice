use anyhow::Context;

use crate::db::{get_conn, is_db};

pub fn add_client(
    surname: &str,
    name: &str,
    telephone_number: &str,
    city: &str,
) -> anyhow::Result<()> {
    if is_db()? {
        let conn = get_conn()?;
        let sql_statement = "INSERT INTO Clients
        ('surname', 'name', 'telephone_number','city_of_residence',
        'datetime_entrance', 'datetime_exit','hours_Parked', 'pariah')
        VALUES
        (?, ?, ?, ?, 'None', 'None', 'None', 'False')";

        conn.execute(sql_statement, (surname, name, telephone_number, city))
            .context("Could not add Client to table!")?;
    }
    Ok(())
}
