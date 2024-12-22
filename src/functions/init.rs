use std::{fs, path::Path};

pub fn init_seeder() {
    let path = Path::new("seeders");

    if path.exists() {
        println!("The 'seeders' directory already exists. No action taken.");
        return;
    }

    match fs::create_dir(path) {
        Ok(_) => println!("Successfully created the 'seeders' directory."),
        Err(e) => eprintln!("Error: Unable to create 'seeders' directory. Reason: {e}"),
    }
}
