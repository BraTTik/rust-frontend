mod tinyscv_db;

use tinyscv_db::*;
use crate::tinyscv_db::database::{delete_row_by_column, find_contains, find_exact, insert_row};

fn main() {
    let schema = Schema::new(vec![
        ("id".to_string(), DataType::Integer),
        ("name".to_string(), DataType::Text),
        ("score".to_string(), DataType::Float),
        ("is_active".to_string(), DataType::Boolean)
    ]);

    let mut db = Database::new(schema);

    insert_row(&mut db, Row::new(vec![Value::Integer(1), Value::Text("John".to_string()), Value::Float(100.0), Value::Boolean(true)]));
    insert_row(&mut db, Row::new(vec![Value::Integer(2), Value::Text("Mary".to_string()), Value::Float(90.0), Value::Boolean(false)]));
    insert_row(&mut db, Row::new(vec![Value::Integer(3), Value::Text("Bob".to_string()), Value::Float(80.0), Value::Boolean(true)]));
    insert_row(&mut db, Row::new(vec![Value::Integer(4), Value::Text("Delete Text".to_string()), Value::Float(90.0), Value::Boolean(true)]));
    // panic insert_row(&mut db, Row::new(vec![Value::Integer(3),  Value::Float(80.0), Value::Text("Bob".to_string()), Value::Boolean(true)]));

    assert_eq!(find_exact(&db, "name", &Value::Text("John".to_string())), vec![Value::Integer(1)]);
    assert_eq!(find_exact(&db, "name", &Value::Text("Bob".to_string())), vec![Value::Integer(3)]);
    assert_eq!(find_exact(&db, "name", &Value::Text("Doe".to_string())), vec![]);

    assert_eq!(find_contains(&db, "name", "ob"), vec![Value::Integer(3)]);
    assert_eq!(find_contains(&db, "name", "Doe"), vec![]);
    assert_eq!(find_contains(&db, "name", "o"), vec![Value::Integer(1), Value::Integer(3)]);

    delete_row_by_column(&mut db, "name", &Value::Text("Bob".to_string()));
    assert_eq!(find_exact(&db, "name", &Value::Text("Bob".to_string())), vec![]);

    assert_eq!(find_exact(&db, "score", &Value::Float(90.0)), vec![Value::Integer(2), Value::Integer(4)]);
    delete_row_by_column(&mut db, "score", &Value::Float(90.0));
    assert_eq!(find_exact(&db, "score", &Value::Float(90.0)), vec![]);

    println!("Тесты прошли!");
}
