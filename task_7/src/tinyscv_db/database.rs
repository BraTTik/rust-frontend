use crate::tinyscv_db::*;
use crate::tinyscv_db::data_types::parse_value;
use crate::tinyscv_db::serializable::Serializable;

#[derive(PartialEq, Debug)]
pub struct Database {
    schema: Schema,
    rows: Vec<Row>,
}

impl Database {
    pub fn new(schema: Schema) -> Self {
        Self { schema, rows: Vec::new() }
    }
}

impl Serializable for Database {
    fn to_csv(&self) -> String {
        let mut csv = String::new();
        let names = self.schema.columns.iter().map(|column| column.0.clone()).collect::<Vec<String>>();
        csv += names.join(",").as_str();
        csv += "\n";
        for row in &self.rows {
            csv += row.to_csv().as_str();
        }
        csv
    }

    fn from_csv(csv: &str) -> Self {
        let rows = csv.split("\n").collect::<Vec<&str>>();

        if (rows.is_empty()) {
            panic!("CSV file is empty");
        }

        let names = parse_row_names(rows[0]);
        let data_types = parse_row_data_types(rows[1]);

        if data_types.len() != names.len() {
            panic!("CSV file has invalid format");
        }

        let mut schema = Schema::new(vec![]);
        for (index, name) in names.iter().enumerate() {
            schema.columns.push((name.clone(), data_types[index]));
        }

        let mut db = Database::new(schema);

        for row in rows.iter().skip(1) {
            if row.is_empty() { continue; }
            db.rows.push(Row::from_csv(row));
        }

        db
    }
}

pub fn insert_row(database: &mut Database, row: Row) {
    for (index, value) in row.values.iter().enumerate() {
        let (_, data_type) = &database.schema.columns[index];
        if data_type.matches(value).is_ok() {
            continue;
        } else {
            panic!("Invalid data type for column: {}; Expected {:?} got {:?} instead", database.schema.columns[index].0, data_type, value)
        }
    }
    database.rows.push(row);
}

pub fn find_exact(database: &Database, column_name: &str, input: &Value) -> Vec<Value> {
    let column_index = schema::find_column_index(&database.schema, column_name);
    let column_id_index = schema::find_column_index(&database.schema, "id");
    if column_index.is_none() || column_id_index.is_none() {
        return vec![];
    }

    let mut result: Vec<Value> = Vec::new();
    for row in &database.rows {
        if row.values[column_index.unwrap()] == *input {
            let value = &row.values[column_id_index.unwrap()];
            push_column_id(&mut result, value)
        }
    }

    result
}

pub fn find_contains(database: &Database, column_name: &str, input: &str) -> Vec<Value> {
    let column_index = schema::find_column_index(&database.schema, column_name);
    let column_id_index = schema::find_column_index(&database.schema, "id");
    if column_index.is_none() || column_id_index.is_none() {
        return vec![];
    }

    let mut result: Vec<Value> = Vec::new();

    for row in &database.rows {
        match &row.values[column_index.unwrap()] {
            Value::Text(value) => {
                if value.contains(input) {
                    let value = &row.values[column_id_index.unwrap()];
                    push_column_id(&mut result, value)
                }
            }
            _ => ()
        }
    }

    result
}

pub fn delete_row(database: &mut Database, row_id: &Value) {
   let column_id_index = schema::find_column_index(&database.schema, "id");
    if column_id_index.is_none() {
        return;
    }

    for (index, row) in database.rows.iter_mut().enumerate() {
        if row.values[column_id_index.unwrap()] == *row_id {
            database.rows.remove(index);
            return;
        }
    }
}

pub fn delete_row_by_column(database: &mut Database, column_name: &str, value: &Value) {
    let mut rows_ids: Vec<Value> = vec![];
    match value {
        Value::Text(text) => {
            let ids = find_contains(database, column_name, text);
            for id in ids {
                push_column_id(&mut rows_ids, &id);
            }
        },
        Value::Float(v) => {
            let ids = find_exact(database, column_name, &Value::Float(*v));
            for id in ids {
                push_column_id(&mut rows_ids, &id);
            }
        },
        Value::Integer(v) => {
            let ids = find_exact(database, column_name, &Value::Integer(*v));
            for id in ids {
                push_column_id(&mut rows_ids, &id);
            }
        },
        Value::Boolean(v) => {
            let ids = find_exact(database, column_name, &Value::Boolean(*v));
            for id in ids {
                push_column_id(&mut rows_ids, &id);
            }
        }
    }
    let column_id_index = schema::find_column_index(&database.schema, "id");
    if column_id_index.is_none() {
        return;
    }
    for id in rows_ids {
        delete_row(database, &id);
    }
}

fn parse_row_names(row: &str) -> Vec<String> {
    row.split(",").map(|column| column.to_string()).collect::<Vec<String>>()
}

fn parse_row_data_types(row: &str) -> Vec<DataType> {
    let columns = row.split(",").collect::<Vec<&str>>();
    let mut data_types: Vec<DataType> = vec![];
    for column in columns {
        let (data_type, _) = parse_value(column);
        data_types.push(data_type)
    }

    data_types
}


fn push_column_id(ids: &mut Vec<Value>, value: &Value) {
    match value {
        Value::Integer(id) => {
            ids.push(Value::Integer(*id));
        },
        Value::Text(id) => {
            ids.push(Value::Text(id.clone()))
        }
        _ => ()
    }
}