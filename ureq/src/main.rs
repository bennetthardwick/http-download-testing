use std::io::stdout;

fn main() {
    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");
    let mut guard = stdout().lock();

    eprintln!("Fetching {url} with Ureq");

    let response = ureq::get(&url).call().expect("failed to send request");

    std::io::copy(&mut response.into_reader(), &mut guard).expect("failed to copy to stdout");
}
