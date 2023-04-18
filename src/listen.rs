use axum::{response::IntoResponse, Json, http::StatusCode};
use chrono::Utc;
use tracing::info;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Dog {
    name: String,
    age: u32,
}


pub async fn get_dog() -> impl IntoResponse {
    let now = Utc::now();
    let name = "Millie at ".to_owned() + &now.to_rfc3339();

    info!("dog called, will return name {}", name);

    let dog = Dog {
        name: name,
        age: 1,
    };

    (StatusCode::OK, Json(dog))
}

pub async fn health() -> impl IntoResponse {
    Json("healthy")
}
