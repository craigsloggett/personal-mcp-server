use personal_mcp_server::StdioTransport;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    StdioTransport::write_message(&mut out, r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#)?;

    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    if let Some(msg) = StdioTransport::read_message(&mut input)? {
        eprintln!("read valid message: {}", msg); // stderr logging
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
