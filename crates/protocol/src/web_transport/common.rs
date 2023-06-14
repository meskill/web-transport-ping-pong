use async_trait::async_trait;
use wtransport::{
    error::{ConnectionError, StreamError},
    RecvStream, SendStream,
};

use crate::{ClientError, Communication, CommunicationError, ServerError};

pub const BUFFER_LENGTH: usize = 2048;

impl From<ConnectionError> for ServerError {
    fn from(_value: ConnectionError) -> Self {
        Self::ConnectionError
    }
}

impl From<ConnectionError> for ClientError {
    fn from(_value: ConnectionError) -> Self {
        Self::ConnectionError
    }
}

impl From<ConnectionError> for CommunicationError {
    fn from(_value: ConnectionError) -> Self {
        Self::ConnectionError
    }
}

impl From<StreamError> for CommunicationError {
    fn from(_value: StreamError) -> Self {
        Self::StreamError
    }
}

pub struct WebTransportCommunication {
    recv: RecvStream,
    send: SendStream,
    buffer: [u8; BUFFER_LENGTH],
}

impl WebTransportCommunication {
    pub fn new(recv: RecvStream, send: SendStream) -> Self {
        Self {
            recv,
            send,
            buffer: [0; BUFFER_LENGTH],
        }
    }
}

#[async_trait]
impl Communication for WebTransportCommunication {
    async fn read(&mut self) -> Result<Vec<u8>, CommunicationError> {
        let Some(bytes_read) = self.recv.read(&mut self.buffer).await? else {
          // if we haven't read anything from stream then the stream is possible closed
          return Err(CommunicationError::StreamClosed);
        };

        Ok(Vec::from(&self.buffer[..bytes_read]))
    }

    async fn write(
        &mut self,
        data: impl AsRef<[u8]> + Send + Sync,
    ) -> Result<(), CommunicationError> {
        Ok(self.send.write_all(data.as_ref()).await?)
    }
}
