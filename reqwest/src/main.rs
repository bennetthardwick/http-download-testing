use std::io::Write;

use futures::StreamExt;

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new()
        .http1_only()
        .tcp_nodelay(true)
        .build()
        .expect("failed to build client");

    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");

    eprintln!("Fetching {url} with Reqwest");

    let mut request = client.get(url);

    if let Ok(auth) = std::env::var("TEST_AUTH") {
        eprintln!("Using TEST_AUTH header");
        request = request.header("authorization", format!("Bearer {auth}"));
    }

    let response = request.send().await.expect("failed to send request");

    println!("{:?}", response.remote_addr());

    let mut stream = response.bytes_stream();

    let stdout = std::io::stdout();
    let mut guard = stdout.lock();

    while let Some(next) = stream.next().await {
        let bytes = next.expect("stream returned error");
        guard.write_all(&bytes).expect("failed to write to stdout");
    }
}
