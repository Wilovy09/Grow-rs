use crate::functions::dbs::{libsql, sqlx_db};
use std::error::Error;

pub async fn infer_database(
    database_url: String,
    file_name: Option<&String>,
) -> Result<String, Box<dyn Error>> {
    if let Some(scheme) = database_url.split("://").next() {
        match scheme {
            "libsql" => {
                println!("LibSQL");
                libsql::run_seeder(database_url, file_name).await;
                Ok("LibSQL".to_string())
            }
            "postgres" | "mysql" | "sqlite" => {
                println!("SQLx database detected: {}", scheme);
                sqlx_db::run_seeder(&database_url, file_name).await?;
                Ok(scheme.to_string())
            }
            _ => {
                let error_message = format!("Unknown schema: {}", scheme);
                eprintln!("{}", error_message);
                Err(error_message.into())
            }
        }
    } else {
        let error_message = "Invalid database URL".to_string();
        eprintln!("{}", error_message);
        Err(error_message.into())
    }
}

