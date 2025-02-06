use std::str::FromStr;

pub enum SchemeDriver {
    /// Test mock driver
    Mock,

    #[cfg(feature = "libsql")]
    Libsql,
    #[cfg(feature = "sqlx")]
    Sqlx,
}

impl FromStr for SchemeDriver {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            _ => Err(format!("Unknown schema: {scheme}")),
        }
    }
}
