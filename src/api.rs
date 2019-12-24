use actix_web::{http, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::db;

#[derive(Debug, Clone)]
pub struct QueryKeys {
    keys: Vec<String>,
}

impl QueryKeys {
    pub fn from_req(req: &HttpRequest) -> QueryKeys {
        let keys = req
            .path()
            .split("/")
            .skip(1)
            .map(|seg| seg.to_string())
            .collect();
        if keys == vec![""] {
            QueryKeys { keys: vec![] }
        } else {
            QueryKeys { keys }
        }
    }

    pub fn json_ptr(&self) -> String {
        if self.len() == 0 {
            String::new()
        } else {
            let mut res = String::from("/");
            res.push_str(&self.keys.join("/"));
            res
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.keys.remove(index)
    }
}

pub fn server_info() -> HttpResponse {
    HttpResponse::Ok().json(json!({
      "name": "mockrs",
      "author": "PrivateRookie"
    }))
}

pub fn do_get(req: HttpRequest, data: web::Data<db::Database>) -> HttpResponse {
    let mut database = data.data.lock().unwrap();
    let mut keys = QueryKeys::from_req(&req);
    match db::Database::get(&mut keys, &mut database) {
        Ok(obj) => HttpResponse::Ok().json(obj),
        Err(e) => HttpResponse::build(http::StatusCode::BAD_REQUEST).json(e),
    }
}

pub fn do_post(
    req: HttpRequest,
    data: web::Data<db::Database>,
    obj: web::Json<Value>,
) -> HttpResponse {
    let mut database = data.data.lock().unwrap();
    let mut keys = QueryKeys::from_req(&req);
    match db::Database::insert(&mut keys, &mut database, obj.0) {
        Ok(_) => HttpResponse::new(http::StatusCode::CREATED),
        Err(e) => HttpResponse::build(http::StatusCode::BAD_REQUEST).json(e),
    }
}

pub fn do_delete(req: HttpRequest, data: web::Data<db::Database>) -> HttpResponse {
    let mut database = data.data.lock().unwrap();
    let mut keys = QueryKeys::from_req(&req);
    match db::Database::delete(&mut keys, &mut database) {
        Ok(_) => HttpResponse::new(http::StatusCode::NO_CONTENT),
        Err(e) => HttpResponse::build(http::StatusCode::BAD_REQUEST).json(e),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlushConfig {
    file: String,
}

pub fn flush(data: web::Data<db::Database>, conf: web::Json<FlushConfig>) -> HttpResponse {
    let json_obj = data.data.lock().unwrap();
    let file = conf.0.file;
    match db::Database::flush(&json_obj, file) {
        Ok(_) => HttpResponse::new(http::StatusCode::NO_CONTENT),
        Err(e) => HttpResponse::build(http::StatusCode::INTERNAL_SERVER_ERROR).json(e),
    }
}
