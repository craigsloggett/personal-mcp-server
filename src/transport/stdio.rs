use std::io::{BufRead, Write};
use std::sync::mpsc::{Receiver, Sender};

use crate::transport::{McpMessage, Transport, TransportHandle};

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
    pub fn write_message(mut writer: &impl Write, msg: &str) -> std::io::Result<()> {
        let _ = (&mut writer, msg); // TODO: Placeholder to avoid warnings when compiling.
        unimplemented!()
    }

    // A reader can be any type that implements the BufRead trait. By borrowing the reader we can
    // use the same reader across calls (enabling the ability to read partially read messages).
    pub fn read_message(reader: &mut impl BufRead) -> std::io::Result<McpMessage> {
        let _ = reader; // TODO: Placeholder to avoid warnings when compiling.
        unimplemented!()
    }

    // This loop uses (and owns) a reader that can be any type that implements the BufRead trait.
    // All messages read by the reader from stdin are sent to the queue using a Sender that sends
    // messages of type McpMessage.
    pub fn reader_loop<R: BufRead>(mut reader: R, tx: Sender<McpMessage>) -> std::io::Result<()> {
        let _ = (&mut reader, tx); // TODO: Placeholder to avoid warnings when compiling.
        unimplemented!()
    }

    // This loop uses a writer that can be any type that implements the Write trait. Messages are
    // read from the queue using a Receiver that can read messages of type McpMessage. They are
    // then written to stdout by the writer.
    pub fn writer_loop<W: Write>(mut writer: W, rx: Receiver<McpMessage>) -> std::io::Result<()> {
        let _ = (&mut writer, rx); // TODO: Placeholder to avoid warnings when compiling.
        unimplemented!()
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
    fn start(self) -> std::io::Result<TransportHandle> {
        let _ = self;
        unimplemented!()
    }
}
