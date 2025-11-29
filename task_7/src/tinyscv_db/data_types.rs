#[derive(Debug, PartialEq)]
pub enum DataType {
    Integer,
    Text,
    Float,
    Boolean
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Text(String),
    Float(f32),
    Boolean(bool)
}

pub enum DataTypeError {
    Integer,
    Text,
    Float,
    Boolean
}

impl DataType {
    pub fn matches(&self, value: &Value) -> Result<(), DataTypeError> {
        match (self, value) {
            (DataType::Integer, Value::Integer(_)) => Ok(()),
            (DataType::Text, Value::Text(_)) => Ok(()),
            (DataType::Float, Value::Float(_)) => Ok(()),
            (DataType::Boolean, Value::Boolean(_)) => Ok(()),
            (DataType::Integer, _) => Err(DataTypeError::Integer),
            (DataType::Text, _) => Err(DataTypeError::Text),
            (DataType::Float, _) => Err(DataTypeError::Float),
            (DataType::Boolean, _) => Err(DataTypeError::Boolean)
        }
    }
}