use log::{debug, info, warn};
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
        let data = Mutex::new(serde_json::from_str(&db).expect("Parse db file error"));
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

    pub fn insert(keys: &mut Vec<String>, json_obj: &mut Value, value: Value) -> Result<(), Value> {
        match Self::get(keys, json_obj) {
            Ok(target_obj) => {
                *target_obj = value;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(keys: &mut Vec<String>, json_obj: &mut Value) -> Result<(), Value> {
        let target_key = keys.remove(keys.len() - 1);
        match Self::get(keys, json_obj) {
            Ok(parent_obj) => match parent_obj {
                &mut Value::Object(ref mut map) => match map.remove(&target_key) {
                    Some(_) => Ok(()),
                    None => Err(json!({"reason": "key not found"})),
                },
                &mut Value::Array(ref mut array) => match target_key.parse::<usize>() {
                    Ok(index) => {
                        array.remove(index);
                        Ok(())
                    }
                    Err(_) => Err(json!({"reason": "Parse int error"})),
                },
                _ => Err(json!({"reason": "Invalid Json Struct"})),
            },
            Err(e) => Err(e),
        }
    }

    pub fn flush(json_obj: &Value, file: String) -> Result<(), Value> {
        let new_db = &serde_json::to_string(json_obj).unwrap();
        debug!("Flush data to {:?} -- start", file);
        match fs::write(&file, new_db) {
            Ok(_) => {
                debug!("Flush data to {:?} -- done");
                Ok(())
            }
            Err(e) => {
                debug!("Flush data to {:?} -- failed");
                Err(json!({ "reason": format!("flush failed due to {:?}", e) }))
            }
        }
    }
}
