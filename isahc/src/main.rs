use futures::AsyncReadExt;
use isahc::{
    config::{Configurable, VersionNegotiation},
    Request,
};
use std::io::Write;

#[tokio::main]
async fn main() {
    let client = isahc::HttpClientBuilder::new()
        .tcp_nodelay()
        .version_negotiation(VersionNegotiation::http11())
        .build()
        .expect("failed to create client");

    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");

    eprintln!("Fetching {url} with Isahc");

    let mut request = Request::get(url);

    if let Ok(auth) = std::env::var("TEST_AUTH") {
        eprintln!("Using TEST_AUTH header");
        request = request.header("authorization", format!("Bearer {auth}"));
    }

    let response = client
        .send_async(request.body(()).expect("failed to build request"))
        .await
        .expect("failed to send request");

    let mut body = response.into_body();

    let stdout = std::io::stdout();
    let mut guard = stdout.lock();

    let mut buf = vec![0_u8; 8 * 1024];

    loop {
        let amt = body.read(&mut buf).await.expect("failed to read");

        if amt == 0 {
            break;
        }

        guard
            .write_all(&buf[0..amt])
            .expect("failed to write to stdout");
    }
}
