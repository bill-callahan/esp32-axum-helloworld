
use axum::{routing::get, Router};
mod handlers;
use crate::webserver::handlers::{helloworld, clock_html};

const TCP_LISTENING_PORT: &str = "0.0.0.0:3000";

pub(crate) async fn axumserver() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(helloworld))
        .route("/clock",get(clock_html));

    // run it
    let listener = tokio::net::TcpListener::bind(TCP_LISTENING_PORT)
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}