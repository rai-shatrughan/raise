use axum::{Json};
use serde_json::{Value, json};

pub async fn hello() -> Json<Value> {
    Json(json!({ "hello": "SR" }))
}
