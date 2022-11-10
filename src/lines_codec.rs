pub mod server;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;

pub struct LinesCodec {
    // Buffered reader & writers
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    // Encapsulate a TcpStream with buffered reader / writer functionality.
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        // Both BufReader and LineWriter need to own a stream.
        // Streams can be cloned to simulate splitting Tx & Rx with 'try_clone()'.
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write(&message.as_bytes())?;
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }
}