use personal_mcp_server::ServerConfig;

fn run(config: ServerConfig) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", config.program_name());
    println!("{}", config.config_home());
    Ok(())
}

fn main() {
    let config = ServerConfig::build().unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {e}");
        std::process::exit(2);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
