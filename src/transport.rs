pub mod error;
pub mod stdio;

pub use error::{TransportError, TransportResult};
pub use stdio::StdioTransport;

pub trait Transport {
    fn send_message(&mut self, message: &str) -> TransportResult<()>;
    fn receive_message(&mut self) -> TransportResult<String>;
    fn close(&mut self) -> TransportResult<()>;
}
