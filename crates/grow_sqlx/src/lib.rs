pub use grow_core::SqlValue;
use sqlx::{AnyPool, Row};
use std::collections::BTreeMap;

pub type RenderedTable = Vec<Vec<(String, SqlValue)>>;

// External SqlValue (from main crate)
#[derive(Debug, Clone)]
pub struct ExternalSqlValue {
    pub integer: Option<i64>,
    pub float: Option<f64>,
    pub text: Option<String>,
    pub boolean: Option<bool>,
    pub null: bool,
}

// Conversion functions
pub fn sql_value_from_external(external: ExternalSqlValue) -> SqlValue {
    if external.null {
        SqlValue::Null
    } else if let Some(i) = external.integer {
        SqlValue::Integer(i)
    } else if let Some(f) = external.float {
        SqlValue::Float(f)
    } else if let Some(s) = external.text {
        SqlValue::Text(s)
    } else if let Some(b) = external.boolean {
        SqlValue::Boolean(b)
    } else {
        SqlValue::Null
    }
}

pub async fn run_seeder(
    database_url: String,
    tables: BTreeMap<String, RenderedTable>,
) -> Result<(), String> {
    // Install default drivers for AnyPool
    sqlx::any::install_default_drivers();

    let pool = AnyPool::connect(&database_url).await.map_err(|err| {
        format!("Cannot connect to database ({database_url}): {err}")
    })?;

    for (table, data) in tables {
        for entry in data {
            insert_entry(&pool, &table, entry).await?;
        }
    }

    Ok(())
}

async fn insert_entry(
    pool: &AnyPool,
    table: &str,
    entry: Vec<(String, SqlValue)>,
) -> Result<(), String> {
    let (columns, values) = entry.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

    let escaped_table = escape_table_name(table);

    let placeholders = (1..=values.len())
        .map(|i| format!("${}", i))
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

    let mut query = sqlx::query(&sql_query);

    for value in values {
        query = match value {
            SqlValue::Integer(i) => query.bind(i),
            SqlValue::Float(f) => query.bind(f),
            SqlValue::Text(s) => query.bind(s),
            SqlValue::Boolean(b) => query.bind(b),
            SqlValue::Null => query.bind(Option::<String>::None),
        };
    }

    query
        .execute(pool)
        .await
        .map_err(|err| format!("Cannot execute query ({sql_query}): {err}"))?;

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
    database_url: String,
    query: &str,
) -> Result<(), String> {
    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(&database_url).await.map_err(|err| {
        format!("Cannot connect to database ({database_url}): {err}")
    })?;

    sqlx::query(query)
        .execute(&pool)
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    Ok(())
}

/// Execute a parameterized SQL query (for INSERT with values)
pub async fn execute_query_with_params(
    database_url: String,
    query: &str,
    timestamp: i64,
    name: &str,
) -> Result<(), String> {
    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(&database_url).await.map_err(|err| {
        format!("Cannot connect to database ({database_url}): {err}")
    })?;

    sqlx::query(query)
        .bind(timestamp)
        .bind(name)
        .execute(&pool)
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    Ok(())
}

/// Execute a query and return the first column of the first row as text
pub async fn query_single_text(
    database_url: String,
    sql: &str,
) -> Result<String, String> {
    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(&database_url).await.map_err(|err| {
        format!("Cannot connect to database ({database_url}): {err}")
    })?;

    let row = sqlx::query(sql)
        .fetch_one(&pool)
        .await
        .map_err(|err| format!("Error executing query ({sql}): {err}"))?;

    if let Ok(s) = row.try_get::<String, _>(0) {
        return Ok(s);
    }
    if let Ok(i) = row.try_get::<i64, _>(0) {
        return Ok(i.to_string());
    }
    if let Ok(f) = row.try_get::<f64, _>(0) {
        return Ok(f.to_string());
    }
    if let Ok(b) = row.try_get::<bool, _>(0) {
        return Ok(b.to_string());
    }

    Err(format!("Could not convert query result to text for: {sql}"))
}

/// Execute a query that returns a single integer result (for counting)
pub async fn query_single_int(
    database_url: String,
    query: &str,
    param: &str,
) -> Result<i64, String> {
    sqlx::any::install_default_drivers();
    let pool = AnyPool::connect(&database_url).await.map_err(|err| {
        format!("Cannot connect to database ({database_url}): {err}")
    })?;

    let row: (i64,) = sqlx::query_as(query)
        .bind(param)
        .fetch_one(&pool)
        .await
        .map_err(|err| format!("Error executing query ({query}): {err}"))?;

    Ok(row.0)
}
