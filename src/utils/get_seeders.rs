use core::fmt;
use std::path::{Path, PathBuf};
use std::{env, io};

use tokio::fs;

pub async fn get_seeders() -> Result<PathBuf, String> {
    let seeders_path = env::var("GROW_SEEDERS")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new("seeders").to_path_buf());

    match seeders_path.metadata() {
        Ok(m) if !m.is_dir() => return Err(format!("{seeders_path:#?} is not a folder")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            eprintln!("Creating folder {seeders_path:#?}");
            fs::create_dir(&seeders_path)
                .await
                .map_err(map_io_error(&seeders_path))?;
        }
        Err(e) => return Err(map_io_error(&seeders_path)(e)),
        _ => {}
    };

    Ok(seeders_path)
}

pub fn map_io_error(path: impl fmt::Debug) -> impl Fn(io::Error) -> String {
    move |err| {
        if let io::ErrorKind::NotFound = err.kind() {
            format!("File {path:#?} not found")
        } else {
            format!("{path:#?}: {err}")
        }
    }
}
