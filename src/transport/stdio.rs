use std::fmt::{Display, Formatter};
use std::io::{self, BufRead, Write};
use std::sync::mpsc::{Receiver, Sender};

use super::{McpMessage, Transport, TransportHandle};

pub struct StdioConfig {
    // SPEC: The server MAY write UTF-8 strings to its standard error (stderr)
    //       for logging purposes.
    pub log_to_stderr: bool,
}

// Using the Default trait to define default configuration.
impl Default for StdioConfig {
    fn default() -> Self {
        Self { log_to_stderr: true }
    }
}

pub struct StdioTransport {
    pub config: StdioConfig,
}

impl StdioTransport {
    pub fn new() -> Self {
        Self { config: StdioConfig::default() }
    }

    pub fn with_config(mut self, config: StdioConfig) -> Self {
        self.config = config;
        self
    }

    // A writer can be any type that implements the Write trait.
    pub fn write_message(writer: &mut impl Write, msg: &str) -> io::Result<()> {
        validate_message(msg)?;
        writer.write_all(msg.as_bytes())?;
        writer.write_all(b"\n")?;
        writer.flush()
    }

    // A reader can be any type that implements the BufRead trait. By borrowing the reader we can
    // use the same reader across calls (enabling the ability to read partially read messages).
    pub fn read_message(reader: &mut impl BufRead) -> io::Result<Option<McpMessage>> {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            return Ok(None);
        }

        if line.ends_with('\n') {
            line.pop();
            if line.ends_with('\r') {
                line.pop();
            }
        }

        validate_message(&line)?;
        Ok(Some(line))
    }

    // This loop uses (and owns) a reader that can be any type that implements the BufRead trait.
    // All messages read by the reader from stdin are sent to the queue using a Sender that sends
    // messages of type McpMessage.
    pub fn reader_loop(reader: &mut impl BufRead, tx: &Sender<McpMessage>) -> io::Result<()> {
        while let Some(msg) = Self::read_message(reader)? {
            // If the receiver side is gone, surface BrokenPipe as an I/O error
            tx.send(msg).map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e.to_string()))?;
        }
        Ok(())
    }

    // This loop uses a writer that can be any type that implements the Write trait. Messages are
    // read from the queue using a Receiver that can read messages of type McpMessage. They are
    // then written to stdout by the writer.
    pub fn writer_loop(writer: &mut impl Write, rx: &Receiver<McpMessage>) -> io::Result<()> {
        while let Ok(msg) = rx.recv() {
            Self::write_message(writer, &msg)?; // validates and frames
        }
        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl Transport for StdioTransport {
    // This will start a StdioTransport process which creates the mpsc Receiver and Sender for
    // stdio.
    fn start(self) -> io::Result<TransportHandle> {
        let _ = self;
        unimplemented!()
    }
}

// SPEC: Messages are delimited by newlines and MUST NOT contain embedded
//       newlines.
#[derive(Debug)]
enum StdioMessageError {
    Empty,
    ContainsNewline,
}

impl Display for StdioMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StdioMessageError::Empty => write!(f, "the message is empty"),
            StdioMessageError::ContainsNewline => write!(f, "the message contains an embedded newline"),
        }
    }
}

impl std::error::Error for StdioMessageError {}

// When converting a StdioMessageError to an io::Error, use the "InvalidData" kind.
impl From<StdioMessageError> for io::Error {
    fn from(error: StdioMessageError) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, error)
    }
}

fn validate_message(payload: &str) -> Result<(), StdioMessageError> {
    if payload.is_empty() {
        return Err(StdioMessageError::Empty);
    }
    if payload.contains('\n') {
        return Err(StdioMessageError::ContainsNewline);
    }
    Ok(())
}
