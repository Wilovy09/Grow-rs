use std::collections::BTreeMap;

use srtemplate::SrTemplate;

use super::entry::Entry;
use crate::sql_value::SqlValue;

pub type RenderedTable = Vec<Vec<(String, SqlValue)>>;

pub fn start<'a>() -> SrTemplate<'a> {
    let mut templating = SrTemplate::default();
    templating.set_delimiter("{", "}");

    #[cfg(feature = "fake")]
    super::fake_generated::setup_faker_variables(&templating);

    templating.add_function("fake", super::fake::fake);

    templating
}

pub fn render_tables(
    entries: Vec<Entry>,
) -> Result<BTreeMap<String, RenderedTable>, String> {
    let mut tables = BTreeMap::new();

    let templating = start();

    for entry in entries {
        match entry {
            Entry::Repeat {
                count,
                table_name,
                fields,
            } => {
                let table: &mut Vec<Vec<(String, SqlValue)>> =
                    tables.entry(table_name.clone()).or_default();

                for i in 0..count {
                    templating.add_variable("i", &i);

                    let mut row = Vec::with_capacity(fields.len());

                    for (key, value) in fields.iter() {
                        let key = templating.render(key).map_err(|err| {
                            format!("Cannot resolve key of {table_name}.{key}: {err}")
                        })?;

                        // For repeated data, render templates in text values only
                        let rendered_value = match value {
                            SqlValue::Text(text) => {
                                let rendered = templating.render(text).map_err(|err| {
                                    format!("Cannot resolve value of {table_name}.{key}: {err}")
                                })?;
                                SqlValue::Text(rendered)
                            }
                            other => other.clone(),
                        };

                        row.push((key, rendered_value));
                    }

                    table.push(row)
                }

                templating.remove_variable("i");
            }
            Entry::Static { table_name, values } => {
                let table = tables.entry(table_name.clone()).or_default();

                for fields in values {
                    let mut row = Vec::with_capacity(fields.len());

                    for (key, value) in fields.iter() {
                        let key = templating.render(key).map_err(|err| {
                            format!("Cannot resolve key of {table_name}.{key}: {err}")
                        })?;

                        // For static data, render templates in text values only
                        let rendered_value = match value {
                            SqlValue::Text(text) => {
                                let rendered = templating.render(text).map_err(|err| {
                                    format!("Cannot resolve value of {table_name}.{key}: {err}")
                                })?;
                                SqlValue::Text(rendered)
                            }
                            other => other.clone(),
                        };

                        row.push((key, rendered_value));
                    }

                    table.push(row)
                }
            }
        }
    }

    Ok(tables)
}
