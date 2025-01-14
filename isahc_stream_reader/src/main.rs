use futures::StreamExt;
use isahc::config::{Configurable, VersionNegotiation};
use std::io::Write;
use tokio_util::{compat::FuturesAsyncReadCompatExt, io::ReaderStream};

#[tokio::main]
async fn main() {
    let stdout = std::io::stdout();
    let mut guard = stdout.lock();

    let client = isahc::HttpClientBuilder::new()
        .tcp_nodelay()
        .version_negotiation(VersionNegotiation::http11())
        .build()
        .expect("failed to create client");

    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");

    eprintln!("Fetching {url} with Isahc");

    let mut request = isahc::Request::get(url);

    if let Ok(auth) = std::env::var("TEST_AUTH") {
        eprintln!("Using TEST_AUTH header");
        request = request.header("authorization", format!("Bearer {auth}"));
    }

    let response = client
        .send_async(request.body(()).expect("failed to build request"))
        .await
        .expect("failed to send request");

    let body = response.into_body();

    let mut stream = ReaderStream::new(body.compat());

    while let Some(next) = stream.next().await {
        let bytes = next.expect("stream returned error");
        guard.write_all(&bytes).expect("failed to write to stdout");
    }
}
