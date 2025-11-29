#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Float(v) => {
                let mut r = v.to_string();
                if v % 1.0 == 0.0 {
                    r += ".0";
                }
                r
            },
            Value::Integer(v) => v.to_string(),
            Value::Text(v) => v.clone(),
            Value::Boolean(v) => v.to_string()
        }
    }
}

pub fn parse_value(value: &str) -> (DataType, Value) {
    match value {
        v if v.parse::<i64>().is_ok() => (DataType::Integer, Value::Integer(v.parse::<i64>().unwrap())),
        v if v.parse::<f32>().is_ok() => (DataType::Float, Value::Float(v.parse::<f32>().unwrap())),
        v if v.parse::<bool>().is_ok() => (DataType::Boolean, Value::Boolean(v.parse::<bool>().unwrap())),
        _ => (DataType::Text, Value::Text(value.to_string()))
    }
}