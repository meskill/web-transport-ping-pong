use std::net::{Ipv4Addr, SocketAddr};

use async_trait::async_trait;
use wtransport::{ClientConfig, Endpoint};

use crate::{Client, ClientError, Handler, WebTransportCommunication};

pub struct WebTransportClient<H: Handler<WebTransportCommunication>> {
    server_name: String,
    handler: H,
}

impl<H: Handler<WebTransportCommunication>> WebTransportClient<H> {
    pub fn new<S: AsRef<str>>(server_name: S, handler: H) -> Self {
        Self {
            server_name: server_name.as_ref().to_owned(),
            handler,
        }
    }
}

#[async_trait]
impl<H: Handler<WebTransportCommunication>> Client for WebTransportClient<H> {
    async fn connect(mut self, address: SocketAddr) -> Result<(), ClientError> {
        let config = ClientConfig::builder()
            .with_bind_address(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0))
            .with_no_cert_validation();

        let endpoint = Endpoint::client(config)?;

        loop {
            let result: Result<(), ClientError> = async {
                let connect = endpoint.connect(address, &self.server_name)?.await?;

                let (send, recv) = connect.open_bi().await?;

                let communication = WebTransportCommunication::new(recv, send);

                Ok(self.handler.handle(communication).await?)
            }
            .await;

            match result {
                Ok(_) => return Ok(()),
                Err(error) => {
                    tracing::error!("Client error: {}", error);
                    tracing::info!("Trying to reconnect to server");
                }
            }
        }
    }
}
