use std::collections::btree_map::Range;

use axum::{response::Html, response::Response, routing::get, Router};

#[tokio::main]
async fn main() {
    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app_router()).await.unwrap();
}

fn app_router() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/hello", get(hello))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
