use personal_mcp_server::{ServerConfig, StdioTransport, Transport};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // No real configuration implemented yet.
    let _config = ServerConfig::new();

    let mut transport = StdioTransport::new();

    println!("MCP Server started. Type 'quit' to exit.");

    loop {
        print!(">> ");
        match transport.receive_message() {
            Ok(message) => {
                if message.trim() == "quit" {
                    println!("Goodbye!");
                    break;
                }

                let response = format!("Echo: {}", message);
                if let Err(e) = transport.send_message(&response) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                // You might want to break here depending on the error type
            }
        }
    }

    transport.close()?;
    Ok(())
}
