use std::net::{Ipv4Addr, SocketAddr};

use client::PingPongHandler;
use protocol::{Client, WebTransportClient};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();
    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);
    let handler = PingPongHandler;

    let client = WebTransportClient::new("localhost", handler);

    client
        .connect(address)
        .await
        .expect("Client connection error");
}
