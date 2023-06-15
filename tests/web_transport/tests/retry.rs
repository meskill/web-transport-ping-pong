use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use insta::{assert_snapshot, with_settings};
use tokio::{runtime::Runtime, spawn, time::sleep};

use protocol::{Client, Server, WebTransportClient, WebTransportServer};

use crate::common::{init_tracing, SERVER_CONFIG};

mod common;

#[tokio::test]
async fn client_should_reconnect_to_server() {
    let tracing = init_tracing();
    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);

    let server = WebTransportServer::new(SERVER_CONFIG, server::PingPongHandler);
    let client = WebTransportClient::new("localhost", client::PingPongHandler);

    let client_handler = spawn(async move { client.connect(address).await });

    let server_runtime = Runtime::new().unwrap();

    server_runtime.spawn(async move { server.listen(address).await });

    sleep(Duration::from_millis(50)).await;
    server_runtime.shutdown_background();

    // wtransport doesn't support timeouts so we need some high value to see
    // the actual error after server has shutdown
    sleep(Duration::from_millis(11000)).await;

    let server = WebTransportServer::new(SERVER_CONFIG, server::PingPongHandler);
    let server_handler = spawn(async move { server.listen(address).await });

    sleep(Duration::from_millis(150)).await;

    client_handler.abort();
    server_handler.abort();

    with_settings!({
      filters => vec![
       (r":4433", ":[server_port]"),
       (r":\d+", ":[client_port]"),
       (r"( INFO client: Got response: \[112, 111, 110, 103\]\n)+", " INFO [communication]\n")
      ]
     }, {
      assert_snapshot!(tracing.data());
    });
}
