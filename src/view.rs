use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
    Result,
};
use std::io::{stdout, Write};

use crate::buffer::Buffer;

pub struct View {
    win_size: (usize, usize),
    buffer: Buffer,
}

impl View {
    pub fn new(win_size: (usize, usize)) -> Self {
        let buffer = Buffer::new();

        Self {
            win_size: win_size,
            buffer: buffer,
        }
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.update_terminal()
    }

    fn draw_rows(&mut self) {
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;

        for row in 0..screen_rows {

            if row == screen_rows / 3 {
                let mut message = format!("Bem-vindo ao Inv ❤️");

                if message.len() > screen_columns {
                    message.truncate(screen_columns)
                }

                let mut padding = (screen_columns - message.len()) / 2;
                if padding != 0 {
                    self.buffer.push_ch('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.buffer.push_ch(' '));

                self.buffer.push_str(&message)

            } else {

                self.buffer.push_ch('~')
            }

            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if row < screen_rows - 1 {
                self.buffer.push_str("\r\n")
            }
        }
    }

    fn update_terminal(&mut self) -> Result<()> {
        execute!(self.buffer, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0), cursor::Show)?;
        self.buffer.flush()
    }
}
