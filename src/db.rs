use serde_json::Value::{Array, Object};
use serde_json::{json, Value};
use std::fs;
use std::sync::Mutex;

pub struct Database {
    pub data: Mutex<Value>,
}

impl Database {
    pub fn load(file: &String) -> Database {
        let db = fs::read_to_string(&file).expect(&format!("Unable to read file: {}", file));
        let data = Mutex::new(serde_json::from_str(&db).expect("parse db file error"));
        Database { data }
    }

    pub fn get<'a>(
        keys: &mut Vec<String>,
        json_obj: &'a mut Value,
    ) -> Result<&'a mut Value, Value> {
        if keys.len() == 0 {
            Ok(json_obj)
        } else {
            let key = keys.remove(0);
            match json_obj {
                &mut Object(ref mut object) => match object.get_mut(&key) {
                    Some(obj) => Self::get(keys, obj),
                    None => Err(json!({"reason": "key error"})),
                },
                &mut Array(ref mut array) => match key.parse::<usize>() {
                    Ok(index) => match array.get_mut(index) {
                        None => Err(json!({"reason": "index error"})),
                        Some(obj) => Self::get(keys, obj),
                    },
                    Err(_) => Err(json!({"reason": "uszie expected"})),
                },
                _ => Err(json!({"reason": "Unvalid Json Structure"})),
            }
        }
    }
}
