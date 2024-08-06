use std::io::{self, stdout};

pub struct Buffer {
    lines: String,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            lines: String::new(),
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.lines.push_str(string)
    }

    pub fn push_ch(&mut self, string: char) {
        self.lines.push(string)
    }
}

impl std::io::Write for Buffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(string) => {
                self.lines.push_str(string);
                Ok(string.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let out = write!(stdout(), "{}", self.lines);
        stdout().flush()?;
        self.lines.clear();
        out
    }
}
