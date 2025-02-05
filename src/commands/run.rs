mod drivers;
mod entry;
mod template;
mod fake;
mod fake_generated;

use std::env;
use std::error::Error;
use std::str::FromStr;

use drivers::SchemeDriver;
use entry::Entry;

pub async fn run_seeder(file_name: Option<&String>) -> Result<(), Box<dyn Error>> {
    let Ok(database_url) = env::var("DATABASE_URL") else {
        return Err("Please, be sure to set the `DATABASE_URL` environment variable.".into());
    };

    let entries = Entry::get_from_seeders(file_name).await?;

    let tables = template::render_tables(entries)?;

    let scheme = SchemeDriver::from_str(&database_url)?;

    match scheme {
        SchemeDriver::Mock => {
            for (table, rows) in tables {
                for fields in rows {
                    let (columns, values) = fields.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

                    let query = format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        table,
                        columns.join(", "),
                        values.join(", ")
                    );

                    println!("{query}");
                }
            }
        }

        #[cfg(feature = "libsql")]
        SchemeDriver::Libsql => grow_libsql::run_seeder(database_url, tables).await?,
        #[cfg(feature = "sqlx")]
        SchemeDriver::Sqlx => grow_sqlx::run_seeder(database_url, tables).await?,
    }

    Ok(())
}
