use std::collections::BTreeMap;

pub async fn run_seeder(
    db_url: String,
    tables: BTreeMap<String, Vec<Vec<(String, String)>>>,
) -> Result<(), String> {
    let db_token = std::env::var("TURSO_AUTH_TOKEN").map_err(|err| {
        format!(
            "\
            `TURSO_AUTH_TOKEN`: {err}.\
            \nPlease, be sure to set the `TURSO_AUTH_TOKEN` environment variable.\
        "
        )
    })?;

    let client = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await
        .map_err(|err| format!("Could not build the database client: {err}"))?;

    let conn = client
        .connect()
        .map_err(|err| format!("Could not connect to the database: {err}"))?;

    for (table, rows) in tables {
        for fields in rows {
            insert_entry(&conn, &table, fields).await?
        }
    }

    Ok(())
}

async fn insert_entry(
    conn: &libsql::Connection,
    table: &str,
    entry: Vec<(String, String)>,
) -> Result<(), String> {
    let (columns, values) = entry.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        values.join(", ")
    );

    conn.execute(&query, [0; 0])
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    Ok(())
}
