use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
    Result, event::KeyCode,
};
use std::io::{ Write };
use std::cmp;

use crate::{ buffer::Buffer, rows::Row };

struct EditorCursor {
    x: usize,
    y: usize,
    screen_columns: usize,
    screen_rows: usize
}

impl EditorCursor {
    fn new(screen_columns:usize, screen_rows:usize) -> Self {
        Self {
            x: 0, 
            y: 0,
            screen_columns: screen_columns,
            screen_rows: screen_rows
        }
    }

    // Implementar uma keybind, e usar control e as teclas do VIM
    // pela quantidade de atalhos, isso tera que ser uma impl 
    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.y = self.y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.x != 0 {
                    self.x -= 1;
                }
            }
            KeyCode::Down => {
                if self.y != self.screen_rows - 1 {
                    self.y += 1;
                }
            }
            KeyCode::Right => {
                if self.x != self.screen_columns - 1 {
                    self.x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}


pub struct View {
    win_size: (usize, usize),
    buffer: Buffer,
    cursor: EditorCursor,
    row: Row
}

impl View {
    pub fn new(win_size: (usize, usize)) -> Self {
        let buffer = Buffer::new();
        let row = Row::new();

        Self {
            win_size: win_size,
            buffer: buffer,
            cursor: EditorCursor::new(win_size.0, win_size.1),
            row: row
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor.move_cursor(direction)
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.update_terminal()
    }

    fn draw_rows(&mut self) {
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;

        for row in 0..screen_rows {
            if row >= self.row.number_of_rows() {
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
            } else {
                let len = cmp::min(self.row.get_row(row).len(), screen_columns); // modify
                self.buffer
                    .push_str(&self.row.get_row(row)[..len])
            }
    
            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if row < screen_rows - 1 {
                self.buffer.push_str("\r\n")
            }
        }
    }

    fn update_terminal(&mut self) -> Result<()> {
        execute!(self.buffer, cursor::Hide, cursor::MoveTo(0, 0))?;
        
        let cursor_x = self.cursor.x as u16;
        let cursor_y = self.cursor.y as u16;
        
        self.draw_rows();

        queue!(
            self.buffer,
            cursor::MoveTo(cursor_x, cursor_y), 
            cursor::Show
        )?;

        self.buffer.flush()
    }
}

