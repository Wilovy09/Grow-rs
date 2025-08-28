use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

pub fn create_seeder(name: &str) {
    let dir = Path::new("seeders");
    let file_path = dir.join(format!("{name}.ron"));

    if !dir.exists() {
        eprintln!("Error: The 'seeders' directory does not exist.");
        return;
    }

    if file_path.exists() {
        println!("The file {name}.ron already exists. Do you want to overwrite it? (y/n)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted. The file was not overwritten.");
            return;
        }

        if let Err(e) = fs::remove_file(&file_path) {
            eprintln!("Error: Unable to delete the existing file. Reason: {e}");
            return;
        }
        println!("The existing file has been deleted.");
    }

    match File::create(&file_path) {
        Ok(mut file) => {
            let default_content = format!("\
{{\n\t// Static data\n\t// {name}: DATA[(OBJECT)],\n\t// User: [\n\t//\t (\n\t//\t\t column_name: \"value\",\n\t//\t )\n\t// ],\n\n\t// Repeated data\n\t// {name}(REPEATED_TIMES): {{DATA}},\n\t// User(4): {{\n\t//\t\t \"column_name\": \"hashed_password_admin{{i}}\",\n\t// }},\n}}
");
            if let Err(e) = file.write_all(default_content.as_bytes()) {
                eprintln!("Error: Unable to write to the file. Reason: {e}");
                return;
            }
            println!("Successfully created the seeder file: {name}.ron");
        }
        Err(e) => eprintln!("Error: Unable to create file. Reason: {e}"),
    }
}
