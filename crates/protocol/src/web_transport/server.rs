use std::{net::SocketAddr, path::Path};

use async_trait::async_trait;
use tokio::spawn;
use wtransport::{connection::Connecting, tls::Certificate, Endpoint, ServerConfig};

use crate::{web_transport::common::WebTransportCommunication, Handler, Server, ServerError};

pub struct WebTransportServerConfig<T: AsRef<Path>> {
    pub path_to_certificate: T,
    pub path_to_key: T,
}

pub struct WebTransportServer<H: Handler<WebTransportCommunication>> {
    certificate: Certificate,
    handler: H,
}

impl<H: Handler<WebTransportCommunication>> WebTransportServer<H> {
    pub fn new<T: AsRef<Path>>(config: WebTransportServerConfig<T>, handler: H) -> Self {
        let certificate = Certificate::load(config.path_to_certificate, config.path_to_key)
            .expect("Could not load certificate");

        Self {
            certificate,
            handler,
        }
    }

    async fn handle(connection: Option<Connecting>, mut handler: H) -> Result<(), ServerError> {
        let Some(connection) = connection else {
          // self endpoint is closed just ignore the connection
          return Ok(())
        };

        let connection = connection.await.or(Err(ServerError::ConnectionError))?;
        let remote_address = connection.remote_address().to_string();

        tracing::info!("New connection from: {}", remote_address);

        let span = tracing::info_span!("Handle request", remote_address);

        span.in_scope(|| async {
            let (send, recv) = connection.accept_bi().await?;

            let communication = WebTransportCommunication::new(recv, send);

            Ok(handler.handle(communication).await?)
        })
        .await
    }
}

#[async_trait]
impl<H: Handler<WebTransportCommunication>> Server for WebTransportServer<H> {
    async fn listen(self, address: SocketAddr) -> Result<(), ServerError> {
        let WebTransportServer {
            certificate,
            handler,
        } = self;
        let address_info = address.to_string();

        let config = ServerConfig::builder()
            .with_bind_address(address)
            .with_certificate(certificate);

        let server = Endpoint::server(config)?;

        tracing::info!("Server started on {address_info}");

        loop {
            let connection = server.accept().await;
            let handler = handler.clone();

            spawn(async move {
                if let Err(error) = WebTransportServer::handle(connection, handler).await {
                    tracing::error!("Error while handling request: {}", error);
                }
            });
        }
    }
}
