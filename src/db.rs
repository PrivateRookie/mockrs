use crate::api;
use log::debug;
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
        keys: &mut api::QueryKeys,
        json_obj: &'a mut Value,
    ) -> Result<&'a mut Value, Value> {
        let json_ptr = &keys.json_ptr();
        match json_obj.pointer_mut(json_ptr) {
            Some(obj) => Ok(obj),
            None => Err(json!({"reason": "key error"})),
        }
    }

    pub fn insert(
        keys: &mut api::QueryKeys,
        json_obj: &mut Value,
        value: Value,
    ) -> Result<(), Value> {
        let target_key = keys.remove(keys.len() - 1);
        match json_obj.pointer_mut(&keys.json_ptr()) {
            Some(parent_obj) => match parent_obj {
                Value::Object(obj) => {
                    obj.insert(target_key, value);
                    Ok(())
                }
                Value::Array(array) => match target_key.parse::<usize>() {
                    Ok(idx) => {
                        array.insert(idx, value);
                        Ok(())
                    }
                    Err(_) => Err(json!({"reason": "Parse index as usize error"})),
                },
                _ => Err(json!({"reason": "Invalid json struct"})),
            },
            None => Err(json!({"reason": "key error"})),
        }
    }

    pub fn delete(keys: &mut api::QueryKeys, json_obj: &mut Value) -> Result<(), Value> {
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
                debug!("Flush data to {:?} -- done", file);
                Ok(())
            }
            Err(e) => {
                debug!("Flush data to {:?} -- failed", file);
                Err(json!({ "reason": format!("flush failed due to {:?}", e) }))
            }
        }
    }
}
