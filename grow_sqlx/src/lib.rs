use sqlx::AnyPool;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum SqlValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

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
impl SqlValue {
    pub fn from_external(external: ExternalSqlValue) -> Self {
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

fn escape_table_name(table: &str) -> String {
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

fn escape_column_name(column: &str) -> String {
    format!("\"{}\"", column)
}
