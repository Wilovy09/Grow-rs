use std::error::Error;

pub async fn infer_database(
    database_url: String,
    file_name: Option<&String>,
) -> Result<String, Box<dyn Error>> {
    if let Some(scheme) = database_url.split("://").next() {
        match scheme {
            #[cfg(feature = "libsql")]
            "libsql" => {
                grow_libsql::run_seeder(database_url, file_name).await;
                Ok("LibSQL".to_string())
            }
            #[cfg(not(feature = "libsql"))]
            "libsql" => {
                let error_message = format!(
                    "The schema {} is only available with libsql feature\n\
                    Run: cargo install grow-rs -F libsql\n\
                    https://github.com/Wilovy09/Grow-rs",
                    scheme
                );
                Err(error_message.into())
            }
            #[cfg(feature = "sqlx")]
            "postgres" | "mysql" | "sqlite" => {
                grow_sqlx::run_seeder(&database_url, file_name).await?;
                Ok(scheme.to_string())
            }
            #[cfg(not(feature = "sqlx"))]
            "postgres" | "mysql" | "sqlite" => {
                let error_message = format!(
                    "The schema {} is only available with sqlx feature\n\
                        Run: cargo install grow-rs -F sqlx\n\
                        https://github.com/Wilovy09/Grow-rs",
                    scheme
                );
                Err(error_message.into())
            }
            _ => {
                let error_message = format!("Unknown schema: {}", scheme);
                Err(error_message.into())
            }
        }
    } else {
        let error_message = "Invalid database URL".to_string();
        eprintln!("{}", error_message);
        Err(error_message.into())
    }
}
