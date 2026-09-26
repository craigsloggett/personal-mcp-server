#[derive(Debug, Clone)]
pub struct ServerConfig;

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerConfig {
    pub fn new() -> Self {
        Self
    }
}
