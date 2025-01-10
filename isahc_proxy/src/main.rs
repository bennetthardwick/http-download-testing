use axum::{body::Body, response::IntoResponse, routing::get, Router};
use isahc::config::{Configurable, VersionNegotiation};
use isahc::Request;
use lazy_static::lazy_static;
use tokio::net::TcpListener;
use tokio_util::compat::FuturesAsyncReadCompatExt;
use tokio_util::io::ReaderStream;

lazy_static! {
    static ref CLIENT: isahc::HttpClient = isahc::HttpClientBuilder::new()
        .tcp_nodelay()
        .version_negotiation(VersionNegotiation::http11())
        .build()
        .expect("failed to create client");
    static ref URL: String =
        std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");
}

async fn handler() -> impl IntoResponse {
    let client = CLIENT.clone();

    let mut request = Request::get(URL.as_str());

    if let Ok(auth) = std::env::var("TEST_AUTH") {
        eprintln!("Using TEST_AUTH header");
        request = request.header("authorization", format!("Bearer {auth}"));
    }

    let response = client
        .send_async(request.body(()).expect("failed to build request"))
        .await
        .expect("failed to send request");

    let body = response.into_body();

    Body::from_stream(ReaderStream::new(body.compat()))
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:2000")
        .await
        .expect("failed to bind to :2000");

    println!("Starting Rust server on :2000 to server {}", URL.as_str());

    axum::serve(listener, Router::new().route("/", get(handler)))
        .await
        .expect("failed to serve");
}
