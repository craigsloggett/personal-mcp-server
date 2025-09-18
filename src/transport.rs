use std::sync::mpsc::{Receiver, Sender};

// TODO: The String type will ultimately be swapped out with a struct
//       (validated JSON).
pub(crate) type McpMessage = String;

// Bind the two channel endpoints. Directions are relative to the server.
pub struct TransportHandle {
    incoming: Receiver<McpMessage>,
    outgoing: Sender<McpMessage>,
}

impl TransportHandle {
    pub fn recv(&self) -> Result<McpMessage, std::sync::mpsc::RecvError> {
        self.incoming.recv()
    }

    pub fn try_recv(&self) -> Result<McpMessage, std::sync::mpsc::TryRecvError> {
        self.incoming.try_recv()
    }

    pub fn send(&self, msg: McpMessage) -> Result<(), std::sync::mpsc::SendError<McpMessage>> {
        self.outgoing.send(msg)
    }
}

pub trait Transport {
    fn start(self) -> std::io::Result<TransportHandle>;
}

pub mod stdio;
