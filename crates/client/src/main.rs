use std::net::{Ipv4Addr, SocketAddr};
use std::panic;

use tracing_panic::panic_hook;
use tracing_subscriber::fmt::format::FmtSpan;

use client::PingPongHandler;
use protocol::{Client, WebTransportClient};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    panic::set_hook(Box::new(panic_hook));

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);
    let handler = PingPongHandler;

    let client = WebTransportClient::new("localhost", handler);

    client
        .connect(address)
        .await
        .expect("Client connection error");
}
