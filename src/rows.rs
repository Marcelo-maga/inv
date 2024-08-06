use crate::buffer::Buffer;
use std::{env, fs, path::Path};

pub struct Row {
    row_contents: Vec<Box<str>>,
    buffer: Buffer,
}

impl Row {
    pub fn new() -> Self {
        let mut arg = env::args();

        match arg.nth(1) {
            None => Self {
                row_contents: Vec::new(),
                buffer: Buffer::new(),
            },

            Some(file) => Self::from_file(file.as_ref()),
        }
    }

    pub fn from_file(file: &Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            row_contents: file_contents.lines().map(|it| it.into()).collect(),
            buffer: Buffer::new(),
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, at: usize) -> &str {
        &self.row_contents[at]
    }

    pub fn number_of_chars(&self, at: usize) -> usize {
        self.row_contents[at].len()
    }

    pub fn insert_char(&mut self, y: usize, x: usize, char_insert: char) {
        let mut row = self.row_contents[y].to_string();
        row.insert(x, char_insert);
        self.buffer.push_str(&row);
        self.row_contents[y] = row.into();
    }

    pub fn remove_char(&mut self, y: usize, x: usize) {
        let mut row = self.row_contents[y].to_string();

        if x < row.len() {
            row.remove(x);
            self.buffer.push_str(&row);
            self.row_contents[y] = row.into();
        }

        if self.row_contents[y].is_empty() && self.row_contents.len() > 1 {
            self.row_contents.remove(y);
        }
    }
}
