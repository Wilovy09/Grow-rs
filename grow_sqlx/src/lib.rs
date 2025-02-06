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

    let sql_query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        values.join(", ")
    );

    let query = sqlx::query(&sql_query);

    query
        .execute(pool)
        .await
        .map_err(|err| format!("Cannot execute query ({sql_query}): {err}"))?;

    Ok(())
}
