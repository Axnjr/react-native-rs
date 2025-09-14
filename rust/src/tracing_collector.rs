use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// A writer that collects messages into a Vec<String>
pub struct VecMakeWriter {
    messages: &'static Mutex<Vec<String>>,
}

impl VecMakeWriter {
    pub fn new(messages: &'static Mutex<Vec<String>>) -> Self {
        Self { messages }
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for VecMakeWriter {
    type Writer = VecWriter;

    fn make_writer(&'a self) -> Self::Writer {
        VecWriter::new(self.messages)
    }
}

pub struct VecWriter {
    messages: &'static Mutex<Vec<String>>,
    buffer: Vec<u8>,
}

impl VecWriter {
    fn new(messages: &'static Mutex<Vec<String>>) -> Self {
        Self {
            messages,
            buffer: Vec::new(),
        }
    }
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let message = String::from_utf8_lossy(&self.buffer).trim().to_string();
            if !message.is_empty() {
                if let Ok(mut messages) = self.messages.lock() {
                    messages.push(message);
                }
            }
            self.buffer.clear();
        }
        Ok(())
    }
}

impl Drop for VecWriter {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
