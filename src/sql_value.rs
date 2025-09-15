use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SqlValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    #[allow(dead_code)]
    Null,
}

impl SqlValue {}

impl Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlValue::Integer(i) => write!(f, "{i}"),
            SqlValue::Float(fl) => write!(f, "{fl}"),
            SqlValue::Text(s) => write!(f, "{s}"),
            SqlValue::Boolean(b) => write!(f, "{b}"),
            SqlValue::Null => write!(f, "NULL"),
        }
    }
}
