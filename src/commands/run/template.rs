use std::collections::BTreeMap;

use srtemplate::SrTemplate;

use super::entry::Entry;

pub type RenderedTable = Vec<Vec<(String, String)>>;

pub fn start<'a>() -> SrTemplate<'a> {
    let mut templating = SrTemplate::default();
    templating.set_delimiter("{", "}");

    super::fake_generated::setup_faker_variables(&templating);

    templating.add_function("fake", super::fake::fake);

    templating
}

pub fn render_tables(entries: Vec<Entry>) -> Result<BTreeMap<String, RenderedTable>, String> {
    let mut tables = BTreeMap::new();

    let templating = start();

    for entry in entries {
        match entry {
            Entry::Repeat {
                count,
                table_name,
                fields,
            } => {
                let table: &mut Vec<Vec<(String, String)>> =
                    tables.entry(table_name.clone()).or_default();

                for i in 0..count {
                    templating.add_variable("i", &i);

                    let mut row = Vec::with_capacity(fields.len());

                    for (key, value) in fields.iter() {
                        let key = templating.render(key).map_err(|err| {
                            format!("Cannot resolve key of {table_name}.{key}: {err}")
                        })?;

                        let value = templating.render(value).map_err(|err| {
                            format!("Cannot resolve value of {table_name}.{key}: {err}")
                        })?;

                        row.push((key, value));
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

                        let value = templating.render(value).map_err(|err| {
                            format!("Cannot resolve value of {table_name}.{key}: {err}")
                        })?;

                        row.push((key, value));
                    }

                    table.push(row)
                }
            }
        }
    }

    Ok(tables)
}
