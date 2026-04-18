use super::drivers::SchemeDriver;
use std::str::FromStr;

/// Resolves all `{query(SQL)}` placeholders in `text` by executing each SQL
/// against the database and substituting the first column of the first row.
pub async fn resolve_query_placeholders(
    text: &str,
    database_url: &str,
) -> Result<String, String> {
    let calls = extract_query_calls(text);
    if calls.is_empty() {
        return Ok(text.to_string());
    }

    let mut result = text.to_string();
    // Reverse order so byte offsets stay valid after each substitution
    for (start, end, sql) in calls.into_iter().rev() {
        let value = execute_query_for_value(sql.trim(), database_url).await?;
        result.replace_range(start..end, &value);
    }

    Ok(result)
}

async fn execute_query_for_value(
    sql: &str,
    database_url: &str,
) -> Result<String, String> {
    let scheme =
        SchemeDriver::from_str(database_url).map_err(|e| e.to_string())?;

    match scheme {
        SchemeDriver::Mock => Ok(format!("QUERY_RESULT({sql})")),
        #[cfg(feature = "sqlx")]
        SchemeDriver::Sqlx => {
            grow_sqlx::query_single_text(database_url.to_string(), sql).await
        }
        #[cfg(feature = "libsql")]
        SchemeDriver::Libsql => {
            grow_libsql::query_single_text(database_url.to_string(), sql).await
        }
        #[cfg(feature = "surrealdb")]
        SchemeDriver::Surrealdb => {
            Err("query() is not yet supported for SurrealDB".to_string())
        }
    }
}

/// Finds all `{query(SQL)}` spans in `text`.
/// Returns Vec of (start, end, sql) where `text[start..end]` is the full placeholder.
fn extract_query_calls(text: &str) -> Vec<(usize, usize, String)> {
    let marker = "{query(";
    let mut results = Vec::new();
    let mut search_from = 0;

    while let Some(rel_start) = text[search_from..].find(marker) {
        let abs_start = search_from + rel_start;
        let after_marker = abs_start + marker.len();
        let bytes = text.as_bytes();

        let mut depth = 1usize;
        let mut pos = after_marker;

        while pos < bytes.len() && depth > 0 {
            match bytes[pos] {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
            pos += 1;
        }

        // `pos` should be at the closing `)`, next char should be `}`
        if pos < bytes.len()
            && bytes[pos] == b')'
            && pos + 1 < bytes.len()
            && bytes[pos + 1] == b'}'
        {
            let sql = text[after_marker..pos].to_string();
            results.push((abs_start, pos + 2, sql));
            search_from = pos + 2;
        } else {
            search_from = abs_start + 1;
        }
    }

    results
}
