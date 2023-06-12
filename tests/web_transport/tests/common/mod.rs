use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use protocol::WebTransportServerConfig;

pub const SERVER_CONFIG: WebTransportServerConfig<&str> = WebTransportServerConfig {
    path_to_certificate: "../../crates/server/certs/certificate.pem",
    path_to_key: "../../crates/server/certs/private_key.pem",
};

pub struct MockWriter(Mutex<Vec<u8>>);

impl MockWriter {
    pub fn data(&self) -> String {
        let mut data = self.0.lock().unwrap();

        let result = String::from_utf8_lossy(data.as_slice()).into_owned();

        data.clear();

        result
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

pub fn init_tracing() -> Arc<MockWriter> {
    let tracing = Arc::new(MockWriter(Mutex::new(Vec::new())));

    tracing_subscriber::fmt()
        .without_time()
        .with_writer(Arc::clone(&tracing))
        .with_ansi(false)
        .init();

    tracing
}
