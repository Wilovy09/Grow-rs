mod drivers;
mod entry;
#[cfg(feature = "fake")]
mod fake;
#[cfg(feature = "fake")]
mod fake_generated;
mod template;

use grow_core::SqlValue;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::str::FromStr;

use crate::utils;
use drivers::SchemeDriver;
use entry::Entry;
use inquire::MultiSelect;

pub async fn run_seeder(
    file_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    if file_name.is_none() {
        return run_seeder_interactive().await;
    }

    run_single_seeder(file_name).await
}

pub async fn run_seeder_interactive() -> Result<(), Box<dyn Error>> {
    let seeders = utils::list_seeders().await?;

    if seeders.is_empty() {
        println!("No seeders available in the seeders directory.");
        return Ok(());
    }

    let selected = MultiSelect::new("Select seeders to run:", seeders)
        .with_help_message(
            "Use ↑/↓ to navigate, Space to select, A to toggle all, Enter to confirm",
        )
        .with_formatter(&|options| {
            format!(
                "\n{}",
                options
                    .iter()
                    .map(|option| format!("🌱 {}", option.value))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })
        .prompt()?;

    if selected.is_empty() {
        println!("No seeders selected.");
        return Ok(());
    }

    for seeder_name in selected {
        if let Err(e) = run_single_seeder(Some(&seeder_name)).await {
            eprintln!(
                "\x1b[1;31;91m[ERROR] Failed to run {}: {}\x1b[0m",
                seeder_name, e
            );
        }
    }

    Ok(())
}

async fn run_single_seeder(
    file_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    let Ok(database_url) = env::var("DATABASE_URL") else {
        return Err(
            "Please, be sure to set the `DATABASE_URL` environment variable."
                .into(),
        );
    };

    let entries = Entry::get_from_seeders(file_name).await?;

    let tables = template::render_tables(entries)?;

    let scheme = SchemeDriver::from_str(&database_url)?;

    match scheme {
        SchemeDriver::Mock => {
            for (table, rows) in tables {
                for fields in rows {
                    let (columns, values) =
                        fields.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

                    let query = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        table,
                        columns.join(", "),
                        values
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );

                    println!("{query}");
                }
            }
        }

        #[cfg(feature = "libsql")]
        SchemeDriver::Libsql => {
            let converted_tables = convert_tables_for_libsql(tables);
            grow_libsql::run_seeder(database_url, converted_tables).await?
        }
        #[cfg(feature = "sqlx")]
        SchemeDriver::Sqlx => {
            let converted_tables = convert_tables_for_sqlx(tables);
            grow_sqlx::run_seeder(database_url, converted_tables).await?
        }
    }

    Ok(())
}

#[cfg(feature = "libsql")]
fn convert_tables_for_libsql(
    tables: BTreeMap<String, template::RenderedTable>,
) -> BTreeMap<String, Vec<Vec<(String, SqlValue)>>> {
    // No need for conversion since both use the same SqlValue from grow_core
    tables
}

#[cfg(feature = "sqlx")]
fn convert_tables_for_sqlx(
    tables: BTreeMap<String, template::RenderedTable>,
) -> BTreeMap<String, Vec<Vec<(String, SqlValue)>>> {
    // No need for conversion since both use the same SqlValue from grow_core
    tables
}
