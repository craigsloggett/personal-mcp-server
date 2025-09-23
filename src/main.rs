use personal_mcp_server::{ServerConfig, StdioTransport, Transport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _config = ServerConfig::new();

    let mut transport = StdioTransport::new();

    loop {
        match transport.receive_message() {
            Ok(message) => {
                if let Err(e) = transport.send_message(&message.to_string()) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                break;
            }
        }
    }

    transport.close()?;
    Ok(())
}
