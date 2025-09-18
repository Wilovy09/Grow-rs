pub use grow_core::SqlValue;
use std::collections::BTreeMap;

pub async fn run_seeder(
    db_url: String,
    tables: BTreeMap<String, Vec<Vec<(String, SqlValue)>>>,
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
    entry: Vec<(String, SqlValue)>,
) -> Result<(), String> {
    let (columns, values) = entry.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    let escaped_table = escape_table_name(table);

    let placeholders = (1..=values.len())
        .map(|i| format!("?{}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let sql_query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        escaped_table,
        columns
            .iter()
            .map(|col| escape_column_name(col))
            .collect::<Vec<_>>()
            .join(", "),
        placeholders
    );

    let params: Vec<libsql::Value> = values
        .into_iter()
        .map(|v| match v {
            SqlValue::Integer(i) => libsql::Value::Integer(i),
            SqlValue::Float(f) => libsql::Value::Real(f),
            SqlValue::Text(s) => libsql::Value::Text(s),
            SqlValue::Boolean(b) => {
                libsql::Value::Integer(if b { 1 } else { 0 })
            }
            SqlValue::Null => libsql::Value::Null,
        })
        .collect();

    conn.execute(&sql_query, params)
        .await
        .map_err(|err| format!("Error executing query ({sql_query}): {err}"))?;

    Ok(())
}

pub fn escape_table_name(table: &str) -> String {
    if table.contains('.') {
        let parts: Vec<&str> = table.split('.').collect();
        parts
            .iter()
            .map(|part| format!("\"{}\"", part))
            .collect::<Vec<_>>()
            .join(".")
    } else {
        format!("\"{}\"", table)
    }
}

pub fn escape_column_name(column: &str) -> String {
    format!("\"{}\"", column)
}
