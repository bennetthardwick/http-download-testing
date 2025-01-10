use isahc::{
    config::{Configurable, VersionNegotiation},
    Request,
};

fn main() {
    let stdout = std::io::stdout();
    let mut guard = stdout.lock();

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
        .send(request.body(()).expect("failed to build request"))
        .expect("failed to send request");

    let mut body = response.into_body();

    std::io::copy(&mut body, &mut guard).expect("failed to copy to stdout");
}
