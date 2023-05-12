mod controllers;
mod models;

use axum::{
    routing::get,
    Router,
};
use controllers::my_controller::hello;
use models::test_model::test;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route("/", get(hello));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello world!"
}
