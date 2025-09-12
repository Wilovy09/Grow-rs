use std::collections::BTreeMap;

use sqlx::any::install_default_drivers;
use sqlx::AnyPool;

pub async fn run_seeder(
    db_url: String,
    tables: BTreeMap<String, Vec<Vec<(String, String)>>>,
) -> Result<(), String> {
    install_default_drivers();
    let pool = AnyPool::connect(&db_url)
        .await
        .map_err(|err| format!("Cannot establish db connection: {err}"))?;

    for (table, rows) in tables {
        for fields in rows {
            insert_entry(&pool, &table, fields).await?
        }
    }

    Ok(())
}

async fn insert_entry(
    pool: &AnyPool,
    table: &str,
    entry: Vec<(String, String)>,
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
        query = query.bind(value);
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
