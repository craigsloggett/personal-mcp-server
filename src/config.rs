use crate::error::ConfigResult;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub transport_type: TransportType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransportType {
    Stdio,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            transport_type: TransportType::Stdio,
        }
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_transport(mut self, transport: TransportType) -> Self {
        self.transport_type = transport;
        self
    }

    pub fn validate(&self) -> ConfigResult<()> {
        Ok(())
    }
}
