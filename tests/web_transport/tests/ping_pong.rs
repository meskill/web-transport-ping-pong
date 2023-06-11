use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddr},
    sync::{Arc, Mutex},
    time::Duration,
};

use insta::{assert_snapshot, with_settings};
use tokio::{spawn, time::sleep};

use protocol::{Client, Server, WebTransportClient, WebTransportServer, WebTransportServerConfig};

struct MockWriter(Mutex<Vec<u8>>);

impl MockWriter {
    fn data(&self) -> String {
        String::from_utf8_lossy(self.0.lock().unwrap().as_slice()).into_owned()
    }
}

impl Write for &MockWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn should_do_ping_pong() {
    let writer = Arc::new(MockWriter(Mutex::new(Vec::new())));

    tracing_subscriber::fmt()
        .without_time()
        .with_writer(Arc::clone(&writer))
        .with_ansi(false)
        .init();
    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4433);

    let server_config = WebTransportServerConfig {
        path_to_certificate: "../../crates/server/certs/certificate.pem",
        path_to_key: "../../crates/server/certs/private_key.pem",
    };

    let server = WebTransportServer::new(server_config, server::PingPongHandler);

    let client = WebTransportClient::new("localhost", client::PingPongHandler);

    let server_handler = spawn(async move { server.listen(address).await });
    let client_handler = spawn(async move { client.connect(address).await });

    sleep(Duration::from_millis(4500)).await;

    client_handler.abort();
    server_handler.abort();

    with_settings!({
      filters => vec![
       (r":4433", ":[server_port]"),
       (r":[[:xdigit:]]+", ":[client_port]"),
      ]
     }, {
      assert_snapshot!(writer.data());
    });
}
