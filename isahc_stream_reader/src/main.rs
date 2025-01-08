use futures::{AsyncRead, StreamExt};
use isahc::{
    config::{Configurable, VersionNegotiation},
    AsyncBody,
};
use pin_project::pin_project;
use std::{
    io::Write,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead as TokioAsyncRead, ReadBuf};
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

#[tokio::main]
async fn main() {
    let client = isahc::HttpClientBuilder::new()
        .tcp_nodelay()
        .version_negotiation(VersionNegotiation::http11())
        .build()
        .expect("failed to create client");

    let url = std::env::var("TEST_URL").expect("failed to get TEST_URL environment variable");

    eprintln!("Fetching {url} with Isahc");

    let response = client.get_async(url).await.expect("failed to send request");

    let stdout = std::io::stdout();
    let mut guard = stdout.lock();

    let body = response.into_body();

    let mut stream = ReaderStream::new(AsyncBodyShim(body));

    while let Some(next) = stream.next().await {
        let bytes = next.expect("stream returned error");
        guard.write_all(&bytes).expect("failed to write to stdout");
    }
}
