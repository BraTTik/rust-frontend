use crate::tinyscv_db::*;
use crate::tinyscv_db::data_types::parse_value;
use crate::tinyscv_db::serializable::Serializable;

#[derive(Debug, PartialEq)]
pub struct Row {
    pub values: Vec<Value>
}

impl Row {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }

}

impl Serializable for Row {
    fn to_csv(&self) -> String {
        let mut str = String::new();
        for value in &self.values {
            str += value.to_string().as_str();
            str += ",";
        }
        str.pop();
        str += "\n";

        str
    }

    fn from_csv(csv: &str) -> Self {
        let values = csv.split(",").map(|value| parse_value(value).1).collect::<Vec<Value>>();
        Self::new(values)
    }
}