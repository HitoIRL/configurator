// lua interpreter

use rlua::{Table, Value};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Entry {
    String(String, String),
    Int(String, i64),
    Float(String, f64), // number
    Bool(String, bool),
    Table(String, Vec<Entry>),
}

pub fn walk_table(entries: &mut Vec<Entry>, table: Table<'_>) {
    for pair in table.pairs::<Value, Value>() {
        let (key, value) = pair.unwrap();

        // I guess key can only be a string
        if let Value::String(name) = key {
            let entry_name = name.to_str().unwrap().to_string();

            let e = match value {
                Value::String(v) => Entry::String(entry_name, v.to_str().unwrap().to_string()),
                Value::Integer(v) => Entry::Int(entry_name, v),
                Value::Number(v) => Entry::Float(entry_name, v),
                Value::Boolean(v) => Entry::Bool(entry_name, v),
                Value::Table(v) => {
                    let mut entries = Vec::new();
                    walk_table(&mut entries, v);
                    Entry::Table(entry_name, entries)
                }
                other => panic!("Value of type {} is not yet implemented!", other.type_name())
            };

            entries.push(e);
        }
    }
}
