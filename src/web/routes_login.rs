use crate::{Error, Result};
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: implement db/auth logic.
    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFailed);
    }

    // TODO: set cookies

    // create body
    let body = Json(json!({
      "result": {
            "success": true,
        },
    }));

    Ok(body)
}
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
