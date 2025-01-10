use isahc::{
    config::{Configurable, VersionNegotiation},
    Request,
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
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

        let body = response.into_body();

        tokio::io::copy(&mut body.compat(), &mut tokio::io::stdout())
            .await
            .expect("failed to copy to stdout");
    })
    .await
    .expect("task returned error");
}
