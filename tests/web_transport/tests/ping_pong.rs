use std::{
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};

use insta::{assert_snapshot, with_settings};
use tokio::{spawn, time::sleep};

use protocol::{Client, Server, WebTransportClient, WebTransportServer};

use crate::common::{init_tracing, SERVER_CONFIG};

mod common;

#[tokio::test]
async fn should_do_ping_pong() {
    let tracing = init_tracing();
    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);

    let server = WebTransportServer::new(SERVER_CONFIG, server::PingPongHandler);
    let client = WebTransportClient::new("localhost", client::PingPongHandler);

    let server_handler = spawn(async move { server.listen(address).await });
    let client_handler = spawn(async move { client.connect(address).await });

    sleep(Duration::from_millis(100)).await;

    client_handler.abort();
    server_handler.abort();

    with_settings!({
      filters => vec![
       (r":4433", ":[server_port]"),
       (r":\d+", ":[client_port]"),
      ]
     }, {
      assert_snapshot!(tracing.data().lines().take(10).collect::<Vec<&str>>().join("\n"));
    });
}
