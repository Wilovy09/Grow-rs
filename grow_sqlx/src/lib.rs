use ron::Value as RonValue;
use sqlx::any::install_default_drivers;
use sqlx::AnyPool;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub async fn run_seeder(db_url: &str, file_name: Option<&String>) -> Result<(), Box<dyn Error>> {
    install_default_drivers();
    let seeders_path = Path::new("seeders");

    if !seeders_path.is_dir() {
        eprintln!("The directory 'seeders/' does not exist");
        return Ok(());
    }

    let pool = AnyPool::connect(db_url).await?;
    let files: Vec<_> = match file_name {
        Some(file_name) => vec![seeders_path.join(file_name)],
        None => fs::read_dir(seeders_path)?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map_or(false, |ext| ext == "ron"))
            .collect(),
    };

    for file in files {
        if let Err(err) = process_file(&file, &pool).await {
            eprintln!("Error processing {:?}: {}", file, err);
        }
    }

    Ok(())
}

async fn process_file(file: &Path, pool: &AnyPool) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let data: RonValue = ron::from_str(&content)?;

    if let RonValue::Map(map) = data {
        for (key, value) in map {
            if let RonValue::String(table) = key {
                if let RonValue::Seq(entries) = value {
                    for entry in entries {
                        if let RonValue::Map(map) = entry {
                            let entry_map: HashMap<_, _> = map.into_iter().collect();
                            insert_entry(pool, &table, entry_map).await?;
                        }
                    }
                    println!("Seeder executed: {:?}", file);
                }
            } else {
                eprintln!(
                    "Error: The primary key must be a string. Invalid key: {:?}",
                    key
                );
            }
        }
    }

    Ok(())
}

async fn insert_entry(
    pool: &AnyPool,
    table: &str,
    entry: HashMap<RonValue, RonValue>,
) -> Result<(), Box<dyn Error>> {
    let columns: Vec<_> = entry
        .keys()
        .map(|k| {
            if let RonValue::String(s) = k {
                s.clone()
            } else {
                panic!("The columns must be strings");
            }
        })
        .collect();
    let values: Vec<_> = entry
        .values()
        .map(|v| match v {
            RonValue::String(s) => s.clone(),
            RonValue::Number(n) => n
                .as_f64()
                .map(|f| f.to_string())
                .or_else(|| n.as_i64().map(|i| i.to_string()))
                .expect("Invalid number"),
            other => format!("{:?}", other),
        })
        .collect();

    let placeholders: String = (1..=values.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let sql_query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        placeholders
    );

    let mut query = sqlx::query(&sql_query);
    for value in values.iter() {
        query = query.bind(value.as_str());
    }

    query.execute(pool).await?;

    Ok(())
}
