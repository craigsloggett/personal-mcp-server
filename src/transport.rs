use std::sync::mpsc::{Receiver, Sender};

// TODO: The String type will ultimately be swapped out with a struct
//       (validated JSON).
pub type McpMessage = String;

// Bind the two channel endpoints. Directions are relative to the server.
pub struct TransportHandle {
    pub incoming: Receiver<McpMessage>,
    pub outgoing: Sender<McpMessage>,
}

pub trait Transport {
    fn start(self) -> std::io::Result<TransportHandle>;
}

// SPEC: Messages are delimited by newlines and MUST NOT contain embedded
//       newlines.
#[derive(Debug)]
pub enum ValidationError {
    Empty,
    ContainsNewline,
}

pub fn validate_message(payload: &str) -> Result<(), ValidationError> {
    if payload.is_empty() {
        return Err(ValidationError::Empty);
    }
    if payload.contains('\n') {
        return Err(ValidationError::ContainsNewline);
    }
    Ok(())
}

pub mod stdio;
