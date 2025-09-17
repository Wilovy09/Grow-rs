use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use ron_edit::*;

use crate::sql_value::SqlValue;
use crate::utils;

#[derive(Debug)]
pub enum Entry {
    Repeat {
        count: usize,
        table_name: String,
        fields: BTreeMap<String, SqlValue>,
    },
    Static {
        table_name: String,
        values: Vec<BTreeMap<String, SqlValue>>,
    },
}

impl Entry {
    pub async fn get_from_seeders(
        file_name: Option<&String>,
    ) -> Result<Vec<Entry>, String> {
        let mut seeders_path = utils::get_seeders().await?;

        if let Some(file_name) = file_name {
            let file_name_with_extension = if file_name.ends_with(".ron") {
                file_name.clone()
            } else {
                format!("{}.ron", file_name)
            };

            seeders_path.push(file_name_with_extension);

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
        let content =
            fs::read_to_string(&path).map_err(utils::map_io_error(&path))?;

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
        // Process inline attributes
        let (mut repeat_count, mut schema_name): (
            Option<usize>,
            Option<String>,
        ) = (None, None);

        if let Some(ref attributes) = map_item.attributes {
            // Convert Vec<WsLead<InlineAttribute>> to slice of InlineAttribute
            let attrs: Vec<&ron_edit::InlineAttribute> =
                attributes.iter().map(|w| &w.content).collect();
            repeat_count = extract_repeat_count(&attrs);
            schema_name = extract_schema_name(&attrs);
        }

        // Then process the key (fallback to old syntax if no attributes)
        let (table_name, repeated) = match map_item.key {
            Value::Unit(content)
            | Value::Str(Str::Baked(content) | Str::Raw { content, .. }) => {
                (content, repeat_count)
            }
            Value::Tuple(Tuple {
                ident: Some(ident),
                fields,
            }) => {
                let Some(first_field) = fields.values.first() else {
                    return Err(
                        "Tuple as key must have the repeated times".to_owned()
                    );
                };

                let repeated_times = match &first_field.content {
                    Value::Int(i) => i
                        .to_string()
                        .parse::<usize>()
                        .map_err(|err| format!("Cannot parse int: {err}"))?,
                    Value::Float(_) => {
                        return Err("Repeated times must be a number".to_owned())
                    }
                    _ => {
                        return Err("Repeated times must be a number".to_owned())
                    }
                };

                (ident.content, Some(repeated_times))
            }
            Value::Tuple(Tuple {
                ident: None,
                fields,
            }) => {
                if fields.values.len() != 2 {
                    return Err("Tuple without identifier must have exactly 2 elements: (table_name, count)".to_owned());
                }

                let table_name = match &fields.values[0].content {
                    Value::Str(Str::Baked(content) | Str::Raw { content, .. }) => *content,
                    _ => {
                        return Err(
                            "First element of tuple must be a string for table name".to_owned()
                        )
                    }
                };

                let repeated_times = match &fields.values[1].content {
                    Value::Int(i) => i
                        .to_string()
                        .parse::<usize>()
                        .map_err(|err| format!("Cannot parse int: {err}"))?,
                    Value::Float(_) => {
                        return Err("Second element of tuple must be a number"
                            .to_owned())
                    }
                    _ => {
                        return Err("Second element of tuple must be a number"
                            .to_owned())
                    }
                };

                (table_name, Some(repeated_times))
            }

            Value::Struct(_) => todo!("Struct as key is planned"),

            _ => {
                return Err(
                    "Expect string, unit, table or struct as key".to_owned()
                )
            }
        };

        // Apply schema from inline attributes if present
        let final_table_name = if let Some(schema) = schema_name {
            if table_name.contains('.') {
                table_name.to_owned() // Keep existing schema if table already has one
            } else {
                format!("{}.{}", schema, table_name) // Apply schema from attribute
            }
        } else {
            normalize_table_name(table_name)
        };

        if let Some(count) = repeated {
            let (_, fields) =
                fields_from_value(map_item.value.content, &final_table_name)?;

            Ok(Entry::Repeat {
                count,
                table_name: final_table_name,
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

                _ => {
                    return Err(format!(
                        "Expect list as value in {final_table_name}"
                    ))
                }
            };

            Ok(Entry::Static {
                table_name: final_table_name,
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

fn sql_value_from_value(value: Value) -> Result<SqlValue, String> {
    match value {
        Value::Int(i) => {
            let int_val = i
                .to_string()
                .parse::<i64>()
                .map_err(|err| format!("Cannot parse int: {err}"))?;
            Ok(SqlValue::Integer(int_val))
        }
        Value::Float(v) => {
            let float_val = v
                .to_string()
                .parse::<f64>()
                .map_err(|err| format!("Cannot parse float: {err}"))?;
            Ok(SqlValue::Float(float_val))
        }
        Value::Str(Str::Baked(content)) => {
            Ok(SqlValue::Text(content.to_string()))
        }
        Value::Str(Str::Raw { content: v, .. }) => {
            Ok(SqlValue::Text(v.to_string()))
        }
        Value::Char(v) => Ok(SqlValue::Text(v.to_string())),
        Value::Bool(v) => Ok(SqlValue::Boolean(v)),
        _ => Err("Expected primitive as value".to_owned()),
    }
}

fn fields_from_value(
    value: Value,
    table_name: &str,
) -> Result<(String, BTreeMap<String, SqlValue>), String> {
    match value {
        Value::Map(m) => {
            let fields =
                m.0.values
                    .into_iter()
                    .map(|field| {
                        let key = match field.content.key {
                            Value::Unit(content)
                            | Value::Str(
                                Str::Baked(content) | Str::Raw { content, .. },
                            ) => content.to_owned(),
                            _ => {
                                return Err(
                                    "Expected unit or string as key in fields"
                                        .to_owned(),
                                )
                            }
                        };

                        let value = field.content.value.content;
                        let value = sql_value_from_value(value)?;

                        Ok((key, value))
                    })
                    .collect::<Result<BTreeMap<_, _>, String>>()?;

            Ok((table_name.to_owned(), fields))
        }

        Value::Struct(Struct { ident, fields }) => {
            let table_name =
                ident.map_or(table_name, |ident| ident.content).to_owned();
            let normalized_table_name = normalize_table_name(&table_name);

            let fields = fields
                .values
                .into_iter()
                .map(|field| {
                    let key = field.content.key.to_owned();

                    let value = field.content.value.content;
                    let value = sql_value_from_value(value)?;

                    Ok((key, value))
                })
                .collect::<Result<BTreeMap<_, _>, String>>()?;
            Ok((normalized_table_name, fields))
        }
        _ => Err("Expect map or struct as value".to_owned()),
    }
}

/// Extract repeat count from inline attributes
fn extract_repeat_count(
    attributes: &[&ron_edit::InlineAttribute],
) -> Option<usize> {
    attributes.iter().find_map(|attr| match *attr {
        ron_edit::InlineAttribute::KeyValue { ident, value, .. }
            if *ident == "repeat" =>
        {
            // Try to parse the value as a number
            if let ron_edit::Value::Int(int_value) = value {
                int_value.to_string().parse::<usize>().ok()
            } else {
                None
            }
        }
        _ => None,
    })
}

/// Extract schema name from inline attributes
fn extract_schema_name(
    attributes: &[&ron_edit::InlineAttribute],
) -> Option<String> {
    attributes.iter().find_map(|attr| match *attr {
        ron_edit::InlineAttribute::KeyValue { ident, value, .. }
            if *ident == "schema" =>
        {
            // Extract string value
            if let ron_edit::Value::Str(str_value) = value {
                match str_value {
                    ron_edit::Str::Baked(content) => Some(content.to_string()),
                    ron_edit::Str::Raw { content, .. } => {
                        Some(content.to_string())
                    }
                }
            } else {
                None
            }
        }
        _ => None,
    })
}
