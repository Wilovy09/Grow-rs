use std::fs;

pub fn list_seeders() {
    let seeder_dir = "./seeders";

    match fs::read_dir(seeder_dir) {
        Ok(entries) => {
            let files = entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .filter_map(|entry| {
                    entry.file_name().to_string_lossy().to_string();
                    // Remove the ".ron" extension
                    let path = entry.path();
                    if path.extension()?.to_str()? == "ron" {
                        Some(path.file_stem()?.to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

            if files.is_empty() {
                println!("No seeders available.");
            } else {
                println!("Available seeders:");
                for file in files {
                    println!("- {}", file);
                }
            }
        }
        Err(_) => {
            println!("Unable to read seeders directory.");
        }
    }
}
