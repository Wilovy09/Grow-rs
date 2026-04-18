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

/// Execute a single SQL query with parameters (for seeder tracking)
pub async fn execute_query(
    db_url: String,
    query: &str,
    params: Vec<SqlValue>,
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

    let libsql_params: Vec<libsql::Value> = params
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

    conn.execute(query, libsql_params)
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    Ok(())
}

/// Execute a query and return the first column of the first row as text
pub async fn query_single_text(
    db_url: String,
    sql: &str,
) -> Result<String, String> {
    let db_token = std::env::var("TURSO_AUTH_TOKEN").map_err(|err| {
        format!("`TURSO_AUTH_TOKEN`: {err}")
    })?;

    let client = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await
        .map_err(|err| format!("Could not build the database client: {err}"))?;

    let conn = client
        .connect()
        .map_err(|err| format!("Could not connect to the database: {err}"))?;

    let mut rows = conn
        .query(sql, ())
        .await
        .map_err(|err| format!("Error executing query ({sql}): {err}"))?;

    if let Some(row) = rows
        .next()
        .await
        .map_err(|err| format!("Error reading row: {err}"))?
    {
        let value = row
            .get(0)
            .map_err(|err| format!("Error getting column: {err}"))?;
        return Ok(match value {
            libsql::Value::Text(s) => s,
            libsql::Value::Integer(i) => i.to_string(),
            libsql::Value::Real(f) => f.to_string(),
            libsql::Value::Blob(b) => String::from_utf8_lossy(&b).to_string(),
            libsql::Value::Null => "NULL".to_string(),
        });
    }

    Err(format!("Query returned no rows: {sql}"))
}

/// Execute a query that returns a single integer result (for counting)
pub async fn query_single_int(
    db_url: String,
    query: &str,
    params: Vec<SqlValue>,
) -> Result<i64, String> {
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

    let libsql_params: Vec<libsql::Value> = params
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

    let mut rows = conn
        .query(query, libsql_params)
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    if let Some(row) = rows
        .next()
        .await
        .map_err(|err| format!("Error reading row: {err}"))?
    {
        if let Ok(Some(libsql::Value::Integer(count))) = row.get(0) {
            return Ok(count);
        }
    }

    Ok(0)
}
