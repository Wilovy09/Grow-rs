use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use ron_edit::*;

use crate::utils;

#[derive(Debug)]
pub enum Entry {
    Repeat {
        count: usize,
        table_name: String,
        fields: BTreeMap<String, String>,
    },
    Static {
        table_name: String,
        values: Vec<BTreeMap<String, String>>,
    },
}

impl Entry {
    pub async fn get_from_seeders(file_name: Option<&String>) -> Result<Vec<Entry>, String> {
        let mut seeders_path = utils::get_seeders().await?;

        if let Some(file_name) = file_name {
            seeders_path.push(file_name);

            Self::get_from_file(seeders_path)
        } else {
            Self::get_from_folder(seeders_path)
        }
    }

    fn get_from_folder(path: PathBuf) -> Result<Vec<Entry>, String> {
        let mut entries = vec![];

        let seeder_entries = path
            .read_dir()
            .map_err(utils::map_io_error(&path))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "ron"))
            .map(Self::get_from_file);

        for entry in seeder_entries {
            entries.append(&mut entry?)
        }

        Ok(entries)
    }

    fn get_from_file(path: PathBuf) -> Result<Vec<Entry>, String> {
        let content = fs::read_to_string(&path).map_err(utils::map_io_error(&path))?;

        let content = File::try_from(content.as_str())
            .map_err(|err| format!("Cannot parse {path:#?}: {err}"))?;

        match content.value.content {
            Value::Map(map) => map
                .0
                .values
                .into_iter()
                .map(|i| i.content)
                .map(Entry::try_from)
                .collect::<Result<Vec<Entry>, String>>()
                .map_err(|err| format!("{err} in {path:#?}")),

            _ => Err(format!("Expect map in {path:#?}")),
        }
    }
}

impl TryFrom<MapItem<'_>> for Entry {
    type Error = String;

    fn try_from(map_item: MapItem) -> Result<Self, Self::Error> {
        let (table_name, repeated) = match map_item.key {
            Value::Unit(content) | Value::Str(Str::Baked(content) | Str::Raw { content, .. }) => {
                (content, None)
            }
            Value::Tuple(Tuple {
                ident: Some(ident),
                fields,
            }) => {
                let Some(first_field) = fields.values.first() else {
                    return Err("Tuple as key must have the repeated times".to_owned());
                };

                let repeated_times = match &first_field.content {
                    Value::Int(i) => i
                        .to_string()
                        .parse::<usize>()
                        .map_err(|err| format!("Cannot parse int: {err}"))?,
                    Value::Float(_) => return Err("Repeated times must be a number".to_owned()),
                    _ => return Err("Repeated times must be a number".to_owned()),
                };

                (ident.content, Some(repeated_times))
            }
            Value::Tuple(_) => return Err("Tuple as key must have name".to_owned()),

            Value::Struct(_) => todo!("Struct as key is planned"),

            _ => return Err("Expect string, unit, table or struct as key".to_owned()),
        };

        let normalized_table_name = normalize_table_name(table_name);

        if let Some(count) = repeated {
            let (_, fields) = fields_from_value(map_item.value.content, &normalized_table_name)?;

            Ok(Entry::Repeat {
                count,
                table_name: normalized_table_name,
                fields,
            })
        } else {
            let values = match map_item.value.content {
                Value::List(list) => list
                    .0
                    .values
                    .into_iter()
                    .map(|item| fields_from_value(item.content, ""))
                    .map(|item| item.map(|i| i.1))
                    .collect::<Result<Vec<_>, String>>()?,

                _ => return Err(format!("Expect list as value in {normalized_table_name}")),
            };

            Ok(Entry::Static {
                table_name: normalized_table_name,
                values,
            })
        }
    }
}

fn normalize_table_name(table_name: &str) -> String {
    if table_name.contains('.') {
        return table_name.to_owned();
    }

    table_name.to_owned()
}

fn string_from_value(value: Value) -> Result<String, String> {
    match value {
        Value::Int(i) => Ok(i.to_string()),
        Value::Float(v) => Ok(v.to_string()),
        Value::Str(v @ Str::Baked(_)) => Ok(v.to_string()),
        Value::Str(Str::Raw { content: v, .. }) => Ok(format!("'{}'", v.replace("'", "''"))),
        Value::Char(v) => Ok(format!("'{}'", v)),
        Value::Bool(v) => Ok(v.to_string()),
        _ => Err("Expected primitive as value".to_owned()),
    }
}

fn fields_from_value(
    value: Value,
    table_name: &str,
) -> Result<(String, BTreeMap<String, String>), String> {
    match value {
        Value::Map(m) => {
            let fields =
                m.0.values
                    .into_iter()
                    .map(|field| {
                        let key = match field.content.key {
                            Value::Unit(content)
                            | Value::Str(Str::Baked(content) | Str::Raw { content, .. }) => {
                                content.to_owned()
                            }
                            _ => return Err("Expected unit or string as key in fields".to_owned()),
                        };

                        let value = field.content.value.content;
                        let value = string_from_value(value)?;

                        Ok((key, value))
                    })
                    .collect::<Result<BTreeMap<_, _>, String>>()?;

            Ok((table_name.to_owned(), fields))
        }

        Value::Struct(Struct { ident, fields }) => {
            let table_name = ident.map_or(table_name, |ident| ident.content).to_owned();
            let normalized_table_name = normalize_table_name(&table_name);

            let fields = fields
                .values
                .into_iter()
                .map(|field| {
                    let key = field.content.key.to_owned();

                    let value = field.content.value.content;
                    let value = string_from_value(value)?;

                    Ok((key, value))
                })
                .collect::<Result<BTreeMap<_, _>, String>>()?;
            Ok((normalized_table_name, fields))
        }
        _ => Err("Expect map or struct as value".to_owned()),
    }
}
