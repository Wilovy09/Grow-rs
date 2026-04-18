pub mod drivers;
pub mod entry;
#[cfg(feature = "fake")]
pub mod fake;
#[cfg(feature = "fake")]
pub mod fake_generated;
pub mod query;
pub mod seeder_tracker;
pub mod template;

use grow_core::SqlValue;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::str::FromStr;

use crate::utils;
use drivers::SchemeDriver;
use entry::Entry;
use inquire::MultiSelect;
use seeder_tracker::SeederTracker;

pub async fn run_seeder(
    file_name: Option<&String>,
    all: bool,
) -> Result<(), Box<dyn Error>> {
    if all {
        return run_all_pending_seeders().await;
    }

    if file_name.is_none() {
        return run_seeder_interactive().await;
    }

    run_single_seeder_with_tracking(file_name).await
}

pub async fn run_all_pending_seeders() -> Result<(), Box<dyn Error>> {
    let seeders = utils::list_seeders().await?;

    if seeders.is_empty() {
        println!("No seeders available in the seeders directory.");
        return Ok(());
    }

    // Create tracker to check seeder status
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        "Please, be sure to set the `DATABASE_URL` environment variable."
    })?;

    let tracker = SeederTracker::new(database_url)?;
    tracker.ensure_seeds_table().await?;

    // Filter to get only pending seeders
    let mut pending_seeders = Vec::new();
    for seeder in seeders {
        let is_executed = tracker.is_seeder_executed(&seeder).await?;
        if !is_executed {
            pending_seeders.push(seeder);
        }
    }

    if pending_seeders.is_empty() {
        println!(
            "\x1b[1;33m [INFO] All seeders have already been executed.\x1b[0m"
        );
        return Ok(());
    }

    let mut success_count = 0;
    let mut error_count = 0;

    for seeder_name in pending_seeders {
        match run_single_seeder_with_tracking(Some(&seeder_name)).await {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                error_count += 1;
                eprintln!(
                    "\x1b[1;31;91m [ERROR] Failed to run {}: {}\x1b[0m",
                    seeder_name, e
                );
            }
        }
    }

    println!();
    if error_count == 0 {
        println!(
            "\x1b[1;32m All {} seeder(s) executed successfully!\x1b[0m",
            success_count
        );
    } else {
        println!(
            "\x1b[1;33m Execution summary: {} succeeded, {} failed\x1b[0m",
            success_count, error_count
        );
    }

    Ok(())
}

pub async fn run_seeder_interactive() -> Result<(), Box<dyn Error>> {
    let seeders = utils::list_seeders().await?;

    if seeders.is_empty() {
        println!("No seeders available in the seeders directory.");
        return Ok(());
    }

    // Create tracker to check seeder status
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        "Please, be sure to set the `DATABASE_URL` environment variable."
    })?;

    let tracker = SeederTracker::new(database_url)?;
    tracker.ensure_seeds_table().await?;

    // Filter and annotate seeders with their execution status
    let mut annotated_seeders = Vec::new();
    for seeder in seeders {
        let is_executed = tracker.is_seeder_executed(&seeder).await?;
        if !is_executed {
            annotated_seeders.push(seeder.clone());
        }
    }

    if annotated_seeders.is_empty() {
        println!("\x1b[1;33m[INFO] All available seeders have already been executed.\x1b[0m");
        return Ok(());
    }

    let selected = MultiSelect::new("Select seeders to run:", annotated_seeders)
        .with_help_message(
            "Use ↑/↓ to navigate, Space to select, A to toggle all, Enter to confirm",
        )
        .with_formatter(&|options| {
            format!(
                "\n{}",
                options
                    .iter()
                    .map(|option| format!("{} (pending)", option.value))
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
        match run_single_seeder_with_tracking(Some(&seeder_name)).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "\x1b[1;31;91m[ERROR] Failed to run {}: {}\x1b[0m",
                    seeder_name, e
                );
            }
        }
    }

    Ok(())
}

fn read_seeder_timestamp(seeder_name: &str) -> i64 {
    seeder_name
        .split_once('_')
        .and_then(|(ts, _)| ts.parse::<i64>().ok())
        .unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
        })
}

async fn run_single_seeder_with_tracking(
    file_name: Option<&String>,
) -> Result<(), Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        "Please, be sure to set the `DATABASE_URL` environment variable."
    })?;

    let tracker = SeederTracker::new(database_url.clone())?;
    tracker.ensure_seeds_table().await?;

    // Extract seeder name from file path
    let seeder_name = if let Some(name) = file_name {
        // Remove .ron extension if present
        let name = if name.ends_with(".ron") {
            &name[..name.len() - 4]
        } else {
            name
        };
        name.to_string()
    } else {
        return Err("Seeder name is required for tracking".into());
    };

    // Check if seeder was already executed
    if tracker.is_seeder_executed(&seeder_name).await? {
        println!(
            "\x1b[1;33m[SKIP] {} has already been executed\x1b[0m",
            seeder_name
        );
        return Ok(());
    }

    // Execute the seeder
    run_single_seeder(file_name).await?;

    // Mark as executed using timestamp from file
    let timestamp = read_seeder_timestamp(&seeder_name);
    tracker.mark_seeder_executed(&seeder_name, timestamp).await?;

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

    let tables = template::render_tables(entries, &database_url).await?;

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
        #[cfg(feature = "surrealdb")]
        SchemeDriver::Surrealdb => {
            let converted_tables = convert_tables_for_surrealdb(tables);
            grow_surrealdb::run_seeder_with_connection_string(
                &database_url,
                &converted_tables,
            )
            .await?
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

#[cfg(feature = "surrealdb")]
fn convert_tables_for_surrealdb(
    tables: BTreeMap<String, template::RenderedTable>,
) -> BTreeMap<String, Vec<Vec<(String, SqlValue)>>> {
    // No need for conversion since both use the same SqlValue from grow_core
    tables
}
