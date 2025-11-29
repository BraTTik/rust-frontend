use crate::tinyscv_db::*;

#[derive(Debug)]
pub struct Schema {
    pub columns: Vec<(String, DataType)>
}

impl Schema {
    pub fn new(columns: Vec<(String, DataType)>) -> Self {
        Self { columns }
    }
}

pub fn find_column_index(schema: &Schema, column_name: &str) -> Option<usize> {
    for (index, column) in schema.columns.iter().enumerate() {
        if column.0 == column_name {
            return Some(index);
        }
    }

    None
}
