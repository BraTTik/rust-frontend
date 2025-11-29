use crate::tinyscv_db::*;

#[derive(Debug)]
pub struct Row {
    pub values: Vec<Value>
}

impl Row {
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}