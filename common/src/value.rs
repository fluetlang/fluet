use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "'{}'", string),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Null => write!(f, "null"),
        }
    }
}
