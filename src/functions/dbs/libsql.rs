use libsql::params;
use ron::Value as RonValue;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Ejecuta seeders con conexión a la base de datos libSQL
pub async fn run_seeder(db_url: String, file_name: Option<&String>) {
    let seeders_path = Path::new("seeders");
    let db_token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN no existe");

    if !seeders_path.is_dir() {
        eprintln!("El directorio 'seeders/' no existe.");
        return;
    }

    let client = libsql::Builder::new_remote(db_url, db_token)
        .build()
        .await
        .expect("No se pudo construir el cliente de base de datos");

    let conn = client
        .connect()
        .expect("No se pudo conectar a la base de datos");

    let files: Vec<_> = match file_name {
        Some(file_name) => vec![seeders_path.join(file_name)],
        None => fs::read_dir(seeders_path)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().map_or(false, |ext| ext == "ron"))
            .collect(),
    };

    for file in files {
        if let Err(err) = process_file(&file, &conn).await {
            eprintln!("Error procesando {:?}: {}", file, err);
        }
    }
}

async fn process_file(file: &Path, conn: &libsql::Connection) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let data: RonValue = ron::from_str(&content)?;

    if let RonValue::Map(map) = data {
        for (key, value) in map {
            if let RonValue::String(table) = key {
                if let RonValue::Seq(entries) = value {
                    for entry in entries {
                        if let RonValue::Map(map) = entry {
                            let entry_map: HashMap<_, _> = map.into_iter().collect();
                            insert_entry(conn, &table, entry_map).await?;
                        }
                    }
                    println!("- {:?} executed.", file)
                }
            } else {
                eprintln!(
                    "Error: La clave principal debe ser una cadena. Clave no válida: {:?}",
                    key
                );
            }
        }
    }

    Ok(())
}

async fn insert_entry(
    conn: &libsql::Connection,
    table: &str,
    entry: HashMap<RonValue, RonValue>,
) -> Result<(), Box<dyn Error>> {
    let columns: Vec<_> = entry
        .keys()
        .map(|k| {
            if let RonValue::String(s) = k {
                s.clone()
            } else {
                panic!("Las columnas deben ser strings");
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
                .expect("Número inválido"),
            other => format!("{:?}", other),
        })
        .collect();

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table,
        columns.join(", "),
        values.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
    );

    conn.execute(&query, params::params_from_iter(values.clone()))
        .await
        .expect("Error al ejecutar la inserción");

    Ok(())
}
