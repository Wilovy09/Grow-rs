use std::error::Error;

pub async fn infer_database(
    database_url: String,
    file_name: Option<&String>,
) -> Result<String, Box<dyn Error>> {
    if let Some(scheme) = database_url.split("://").next() {
        match scheme {
            #[cfg(feature = "libsql")]
            "libsql" => {
                println!("LibSQL");
                grow_libsql::run_seeder(database_url, file_name).await;
                Ok("LibSQL".to_string())
            }
            #[cfg(feature = "sqlx")]
            "postgres" | "mysql" | "sqlite" => {
                println!("SQLx database detected: {}", scheme);
                grow_sqlx::run_seeder(&database_url, file_name).await?;
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
