use std::io;
use std::net::SocketAddr;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Connection error")]
    ConnectionError,
    #[error("Communication error")]
    CommunicationError(#[from] CommunicationError),
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Connection error")]
    ConnectionError,
    #[error("Communication error")]
    CommunicationError(#[from] CommunicationError),
}

#[derive(Debug, Error)]
pub enum CommunicationError {
    #[error("Connection error")]
    ConnectionError,
    #[error("Stream error")]
    StreamError,
    #[error("Stream closed")]
    StreamClosed,
}

#[async_trait]
pub trait Server {
    async fn listen(self, address: SocketAddr) -> Result<(), ServerError>;
}

#[async_trait]
pub trait Client {
    async fn connect(self, address: SocketAddr) -> Result<(), ClientError>;
}

#[async_trait]
pub trait Communication: Send + Sync {
    async fn read(&mut self) -> Result<Vec<u8>, CommunicationError>;
    async fn write(
        &mut self,
        data: impl AsRef<[u8]> + Send + Sync,
    ) -> Result<(), CommunicationError>;
}

#[async_trait]
pub trait Handler<T: Communication>: Clone + Send + Sync + 'static {
    async fn handle(&mut self, communication: T) -> Result<(), CommunicationError>;
}
