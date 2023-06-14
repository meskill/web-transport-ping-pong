use async_trait::async_trait;

use protocol::{Communication, CommunicationError, Handler};

#[derive(Clone)]
pub struct PingPongHandler;

#[async_trait]
impl<T: Communication + 'static> Handler<T> for PingPongHandler {
    async fn handle(&mut self, mut connection: T) -> Result<(), CommunicationError> {
        loop {
            let input = connection.read().await?;

            let output: &[u8] = match &*input {
                b"ping" => b"pong",
                _ => b"?",
            };

            connection.write(output).await?;
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
        write_data: Option<Result<(), CommunicationError>>,
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
            *self.last_data.lock().unwrap() = Some(data.as_ref().to_vec());

            self.write_data
                .take()
                .unwrap_or(Err(CommunicationError::StreamClosed))
        }
    }

    #[tokio::test]
    async fn should_response_on_ping() {
        let last_data = Arc::new(Mutex::new(None));
        let mock_communication = MockCommunication {
            read_data: Some(Ok(b"ping".to_vec())),
            write_data: Some(Ok(())),
            last_data: Arc::clone(&last_data),
        };

        let mut handler = PingPongHandler;

        let result = handler.handle(mock_communication).await;

        assert!(matches!(result, Err(CommunicationError::StreamClosed)));
        assert_eq!(last_data.lock().unwrap().as_ref().unwrap(), b"pong");
    }

    #[tokio::test]
    async fn should_response_on_unknown_message() {
        let last_data = Arc::new(Mutex::new(None));
        let mock_communication = MockCommunication {
            read_data: Some(Ok(b"Unknown".to_vec())),
            write_data: Some(Ok(())),
            last_data: Arc::clone(&last_data),
        };

        let mut handler = PingPongHandler;

        let result = handler.handle(mock_communication).await;

        assert!(matches!(result, Err(CommunicationError::StreamClosed)));
        assert_eq!(last_data.lock().unwrap().as_ref().unwrap(), b"?");
    }

    #[tokio::test]
    async fn should_not_response_with_anything_on_error() {
        let last_data = Arc::new(Mutex::new(None));
        let mock_communication = MockCommunication {
            read_data: Some(Err(CommunicationError::ConnectionError)),
            write_data: Some(Ok(())),
            last_data: Arc::clone(&last_data),
        };

        let mut handler = PingPongHandler;

        let result = handler.handle(mock_communication).await;

        assert!(matches!(result, Err(CommunicationError::ConnectionError)));
        assert!(last_data.lock().unwrap().is_none());
    }
}
