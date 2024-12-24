use crate::functions::dbs;
use std::error::Error;

pub async fn infer_database(
    database_url: String,
    file_name: Option<&String>,
) -> Result<String, Box<dyn Error>> {
    if let Some(scheme) = database_url.split("://").next() {
        match scheme {
            "libsql" => {
                println!("Libsql");
                dbs::libsql::run_seeder(database_url, file_name).await;
                Ok("libsql".to_string())
            }
            "postgres" => {
                println!("PostgreSQL");
                Ok("PostgreSQL".to_string())
            }
            "mysql" => {
                println!("MySQL");
                Ok("MySQL".to_string())
            }
            "sqlite" => {
                println!("SQLite");
                Ok("SQLite".to_string())
            }
            _ => {
                let error_message = format!("Unknown scheme: {}", scheme);
                println!("{}", error_message);
                Err(error_message.into())
            }
        }
    } else {
        let error_message = "Invalid database URL".to_string();
        println!("{}", error_message);
        Err(error_message.into())
    }
}
