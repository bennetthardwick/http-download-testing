use std::pin::Pin;
use std::task::{Context, Poll};

use axum::{body::Body, response::IntoResponse, routing::get, Router};
use futures::AsyncRead;
use isahc::config::{Configurable, VersionNegotiation};
use isahc::{AsyncBody, Request};
use lazy_static::lazy_static;
use pin_project::pin_project;
use tokio::io::{AsyncRead as TokioAsyncRead, ReadBuf};
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;

#[pin_project]
struct AsyncBodyShim(#[pin] AsyncBody);

impl TokioAsyncRead for AsyncBodyShim {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let me = self.project();

        let bytes = buf.initialize_unfilled();

        match me.0.poll_read(cx, bytes) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(amt)) => {
                buf.advance(amt);
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
        }
    }
}

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

    let mut body = response.into_body();

    Body::from_stream(ReaderStream::new(AsyncBodyShim(response.into_body())))
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:2000")
        .await
        .expect("failed to bind to localhost:2000");

    println!(
        "Starting Rust server on localhost:2000 to server {}",
        URL.as_str()
    );

    axum::serve(listener, Router::new().route("/", get(handler)))
        .await
        .expect("failed to serve");
}
