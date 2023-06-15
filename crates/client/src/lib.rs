use async_trait::async_trait;

use protocol::{Communication, CommunicationError, Handler};

#[derive(Clone)]
pub struct PingPongHandler;

#[async_trait]
impl<T: Communication + 'static> Handler<T> for PingPongHandler {
    async fn handle(&mut self, mut communication: T) -> Result<(), CommunicationError> {
        loop {
            communication.write(b"ping").await?;

            let response = communication.read().await?;

            tracing::info!("Got response: {response:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use protocol::{Communication, CommunicationError, Handler};

    use crate::PingPongHandler;

    pub struct MockCommunication {
        read_data: Option<Result<Vec<u8>, CommunicationError>>,
        last_data: Arc<Mutex<Option<Vec<u8>>>>,
    }

    #[async_trait]
    impl Communication for MockCommunication {
        async fn read(&mut self) -> Result<Vec<u8>, CommunicationError> {
            self.read_data
                .take()
                .unwrap_or(Err(CommunicationError::StreamClosed))
        }

        async fn write(
            &mut self,
            data: impl AsRef<[u8]> + Send + Sync,
        ) -> Result<(), CommunicationError> {
            *self.last_data.lock().unwrap() = Some(data.as_ref().to_owned());

            Ok(())
        }
    }

    #[tokio::test]
    async fn should_send_ping() {
        let last_data = Arc::new(Mutex::new(None));
        let mock_communication = MockCommunication {
            read_data: Some(Ok(b"pong".to_vec())),
            last_data: Arc::clone(&last_data),
        };

        let mut handler = PingPongHandler;

        let result = handler.handle(mock_communication).await;

        assert!(matches!(result, Err(CommunicationError::StreamClosed)));
        assert_eq!(last_data.lock().unwrap().as_ref().unwrap(), b"ping");
    }
}
