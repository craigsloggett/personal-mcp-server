use std::io::{self, BufRead, Write};

use super::Transport;
use super::error::{StdioError, StdioResult, TransportResult};

pub struct StdioTransport;

impl StdioTransport {
    pub fn new() -> Self {
        Self
    }

    pub fn read_message(&mut self) -> StdioResult<String> {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        match handle.read_line(&mut input)? {
            0 => Err(StdioError::EmptyMessage),
            _ => {
                // Remove trailing newline
                if input.ends_with('\n') {
                    input.pop();
                    if input.ends_with('\r') {
                        input.pop();
                    }
                }

                if input.is_empty() {
                    Err(StdioError::EmptyMessage)
                } else {
                    Ok(input)
                }
            }
        }
    }

    pub fn write_message(&mut self, message: &str) -> StdioResult<()> {
        if message.contains('\n') {
            return Err(StdioError::ContainsNewline);
        }

        println!("{}", message);
        io::stdout().flush()?;
        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for StdioTransport {
    fn send_message(&mut self, message: &str) -> TransportResult<()> {
        Ok(self.write_message(message)?)
    }

    fn receive_message(&mut self) -> TransportResult<String> {
        Ok(self.read_message()?)
    }

    fn close(&mut self) -> TransportResult<()> {
        Ok(())
    }
}
