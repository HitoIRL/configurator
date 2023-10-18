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
    SequenceTable(String, Vec<Entry>),
}

fn walk_sequences(entries: &mut Vec<Entry>, table: Table<'_>) {
    let sequence = table.sequence_values::<Value>();
    for (index, value) in sequence.enumerate() {
        let value = value.unwrap();

        let index = format!("[{index}]");
        let e = match value {
            Value::String(v) => Entry::String(index, v.to_str().unwrap().to_string()),
            Value::Integer(v) => Entry::Int(index, v),
            Value::Number(v) => Entry::Float(index, v),
            Value::Boolean(v) => Entry::Bool(index, v),
            Value::Table(v) => {
                let mut entries = Vec::new();
                if walk_table(&mut entries, v.clone()) {
                    Entry::Table(index, entries)
                } else {
                    walk_sequences(&mut entries, v);
                    Entry::SequenceTable(index, entries)
                }
            }
            other => panic!("Value of type {} is not yet implemented!", other.type_name())
        };

        entries.push(e);
    }
}

// TODO: read config in the same order as the lua file
pub fn walk_table(entries: &mut Vec<Entry>, table: Table<'_>) -> bool { // returns true if the table is key, value
    let mut updated = false;

    for pair in table.pairs::<Value, Value>() {
        let (key, value) = pair.unwrap();

        if let Value::String(name) = key {
            let entry_name = name.to_str().unwrap().to_string();

            let e = match value {
                Value::String(v) => Entry::String(entry_name, v.to_str().unwrap().to_string()),
                Value::Integer(v) => Entry::Int(entry_name, v),
                Value::Number(v) => Entry::Float(entry_name, v),
                Value::Boolean(v) => Entry::Bool(entry_name, v),
                Value::Table(v) => {
                    let mut entries = Vec::new();
                    if walk_table(&mut entries, v.clone()) {
                        Entry::Table(entry_name, entries)
                    } else {
                        walk_sequences(&mut entries, v);
                        Entry::SequenceTable(entry_name, entries)
                    }
                }
                other => panic!("Value of type {} is not yet implemented!", other.type_name())
            };

            updated = true;
            entries.push(e);
        }
    }

    updated
}
