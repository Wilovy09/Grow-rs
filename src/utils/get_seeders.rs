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

pub async fn list_seeders() -> Result<Vec<String>, String> {
    let seeders_path = get_seeders().await?;

    let mut entries = fs::read_dir(&seeders_path)
        .await
        .map_err(map_io_error(&seeders_path))?;

    let mut seeders = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(map_io_error(&seeders_path))?
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "ron" {
                    if let Some(file_stem) = path.file_stem() {
                        if let Some(name) = file_stem.to_str() {
                            seeders.push(name.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(seeders)
}
