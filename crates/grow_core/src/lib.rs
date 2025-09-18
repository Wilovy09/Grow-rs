use std::fmt::Display;

/// Represents a SQL value that can be used across different database drivers
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Null,
}

impl SqlValue {
    /// Creates a new Text variant from a string
    pub fn text<T: Into<String>>(value: T) -> Self {
        SqlValue::Text(value.into())
    }

    /// Creates a new Integer variant
    pub fn integer(value: i64) -> Self {
        SqlValue::Integer(value)
    }

    /// Creates a new Float variant
    pub fn float(value: f64) -> Self {
        SqlValue::Float(value)
    }

    /// Creates a new Boolean variant
    pub fn boolean(value: bool) -> Self {
        SqlValue::Boolean(value)
    }

    /// Creates a Null variant
    pub fn null() -> Self {
        SqlValue::Null
    }

    /// Returns true if the value is NULL
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    }

    /// Returns the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            SqlValue::Integer(_) => "INTEGER",
            SqlValue::Float(_) => "REAL",
            SqlValue::Text(_) => "TEXT",
            SqlValue::Boolean(_) => "BOOLEAN",
            SqlValue::Null => "NULL",
        }
    }
}

impl Display for SqlValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlValue::Integer(i) => write!(f, "{}", i),
            SqlValue::Float(fl) => write!(f, "{}", fl),
            SqlValue::Text(s) => write!(f, "{}", s),
            SqlValue::Boolean(b) => write!(f, "{}", b),
            SqlValue::Null => write!(f, "NULL"),
        }
    }
}

impl From<i64> for SqlValue {
    fn from(value: i64) -> Self {
        SqlValue::Integer(value)
    }
}

impl From<f64> for SqlValue {
    fn from(value: f64) -> Self {
        SqlValue::Float(value)
    }
}

impl From<String> for SqlValue {
    fn from(value: String) -> Self {
        SqlValue::Text(value)
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        SqlValue::Text(value.to_string())
    }
}

impl From<bool> for SqlValue {
    fn from(value: bool) -> Self {
        SqlValue::Boolean(value)
    }
}

impl<T> From<Option<T>> for SqlValue
where
    T: Into<SqlValue>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => SqlValue::Null,
        }
    }
}
