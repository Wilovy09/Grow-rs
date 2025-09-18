use std::str::FromStr;

pub enum SchemeDriver {
    /// Test mock driver
    Mock,

    #[cfg(feature = "libsql")]
    Libsql,
    #[cfg(feature = "sqlx")]
    Sqlx,
    #[cfg(feature = "surrealdb")]
    Surrealdb,
}

impl FromStr for SchemeDriver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle SurrealDB special connection strings that don't have schemes
        if s == "memory" || s == "mem" {
            #[cfg(feature = "surrealdb")]
            return Ok(SchemeDriver::Surrealdb);
            #[cfg(not(feature = "surrealdb"))]
            return Err(format!(
                "SurrealDB memory connection is only available with surrealdb feature\n\
                    Run: cargo install grow-rs -F surrealdb\n\
                    https://github.com/Wilovy09/Grow-rs"
            ));
        }

        let Some((scheme, _)) = s.split_once("://") else {
            return Err("Invalid database URL".to_owned());
        };

        match scheme {
            "mock" => Ok(SchemeDriver::Mock),

            #[cfg(feature = "libsql")]
            "libsql" => Ok(SchemeDriver::Libsql),
            #[cfg(not(feature = "libsql"))]
            "libsql" => Err(format!(
                "The schema {scheme} is only available with libsql feature\n\
                    Run: cargo install grow-rs -F libsql\n\
                    https://github.com/Wilovy09/Grow-rs"
            )),
            #[cfg(feature = "sqlx")]
            "postgres" | "mysql" | "sqlite" => Ok(SchemeDriver::Sqlx),
            #[cfg(not(feature = "sqlx"))]
            "postgres" | "mysql" | "sqlite" => Err(format!(
                "The schema {scheme} is only available with sqlx feature\n\
                    Run: cargo install grow-rs -F sqlx\n\
                    https://github.com/Wilovy09/Grow-rs"
            )),
            #[cfg(feature = "surrealdb")]
            "file" | "ws" | "wss" | "http" | "https" => {
                Ok(SchemeDriver::Surrealdb)
            }
            #[cfg(not(feature = "surrealdb"))]
            "file" | "ws" | "wss" | "http" | "https" => Err(format!(
                "The schema {scheme} is only available with surrealdb feature\n\
                    Run: cargo install grow-rs -F surrealdb\n\
                    https://github.com/Wilovy09/Grow-rs"
            )),
            _ => Err(format!("Unknown schema: {scheme}")),
        }
    }
}
