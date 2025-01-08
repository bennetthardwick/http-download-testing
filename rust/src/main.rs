use futures::StreamExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new()
        .http1_only()
        .build()
        .expect("failed to build client");

    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");

    eprintln!("Fetching {url} with Reqwest");

    let response = client
        .get(url)
        .send()
        .await
        .expect("failed to send request");

    let mut stream = response.bytes_stream();
    let mut stdout = tokio::io::stdout();

    while let Some(next) = stream.next().await {
        let bytes = next.expect("stream returned error");
        stdout
            .write_all(&bytes)
            .await
            .expect("failed to write to stdout");
    }
}
