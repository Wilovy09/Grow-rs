#[derive(Debug, Clone)]
pub enum SqlValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    #[allow(dead_code)]
    Null,
}

impl SqlValue {
    pub fn to_string(&self) -> String {
        match self {
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Text(s) => s.clone(),
            SqlValue::Boolean(b) => b.to_string(),
            SqlValue::Null => "NULL".to_string(),
        }
    }
}
