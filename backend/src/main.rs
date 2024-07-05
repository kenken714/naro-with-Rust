use std::collections::btree_map::Range;

use axum::{extract::Query, http::response, response::Html, routing::get, Router};

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
    async fn index_handler() -> Html<&'static str> {
        Html("<h1>Hello, World!</h1>")
    }

    async fn ping_handler() -> Html<&'static str> {
        Html("pong")
    }

    struct FizzBuzzQuery {
        count: Option<usize>,
    }

    impl Default for FizzBuzzQuery {
        fn default() -> Self {
            Self { count: Some(30) }
        }
    }
    async fn fizzbuzz(count: usize) -> String {
        let mut response = String::new();
        for i in 1..=count {
            if i % 3 == 0 && i % 5 == 0 {
                response.push_str("FizzBuzz\n");
            } else if i % 3 == 0 {
                response.push_str("Fizz\n");
            } else if i % 5 == 0 {
                response.push_str("Buzz\n");
            } else {
                response.push_str(&i.to_string());
                response.push_str("\n");
            }
        }
        response
    }

    #[debug_handler]
    async fn fizzbuzz_handler(count: Option<Query<FizzBuzzQuery>>) -> Html<String> {
        let response = match count {
            Some(query) => {
                let count = query.count.unwrap_or_default();
                fizzbuzz(count)
            }
            None => fizzbuzz(30),
        }
        .await;
        Html(response)
    }

    Router::new()
        .route("/", get(index_handler))
        .route("/ping", get(ping_handler))
        .route("/fizzbuzz", get(fizzbuzz_handler));
}
