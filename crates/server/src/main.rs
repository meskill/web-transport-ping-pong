use std::net::{Ipv4Addr, SocketAddr};
use std::panic;

use tokio::spawn;
use tracing_panic::panic_hook;
use tracing_subscriber::fmt::format::FmtSpan;

use protocol::{Server, WebTransportServer, WebTransportServerConfig};
use server::PingPongHandler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .init();

    panic::set_hook(Box::new(panic_hook));

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);
    let config = WebTransportServerConfig {
        path_to_certificate: "certs/certificate.pem",
        path_to_key: "certs/private_key.pem",
    };
    let handler = PingPongHandler;

    let server = WebTransportServer::new(config, handler);

    spawn(async move { server.listen(address).await.expect("Server listen error") })
        .await
        .unwrap();
}
