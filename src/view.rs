use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
    Result, event::KeyCode,
};
use std::io::{ Write };
use std::cmp;

use crate::{ buffer::Buffer, rows::Row, editor };

struct EditorCursor {
    x: usize,
    y: usize,
    screen_columns: usize,
    screen_rows: usize,
    off_screen: usize
}

impl EditorCursor {
    fn new(screen_columns:usize, screen_rows:usize) -> Self {
        Self {
            x: 0, 
            y: 0,
            screen_columns: screen_columns,
            screen_rows: screen_rows,
            off_screen: 0
        }
    }

    // Implementar uma keybind, e usar control e as teclas do VIM
    // pela quantidade de atalhos, isso tera que ser uma impl 
    fn move_cursor(&mut self, direction: KeyCode, number_of_rows: usize) {
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
                if self.y < number_of_rows {
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
    
    fn scroll(&mut self) {
        self.off_screen = cmp::min(self.off_screen, self.y);
        if self.y >= self.off_screen + self.screen_rows {
            self.off_screen = self.y - self.screen_rows + 1;
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

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.update_terminal()
    }

    pub fn move_cursor(&mut self, direction: KeyCode, number_of_row: usize) {
        self.cursor.move_cursor(direction, number_of_row)
    }


    fn draw_rows(&mut self) {   
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;

        for row in 0..screen_rows {
            let file_row = row + self.cursor.off_screen;
            if file_row >= self.row.number_of_rows() {
                if self.row.number_of_rows() == 0 && row == screen_rows / 3 {
                    let mut message = format!("Inv");
    
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
                let len = cmp::min(self.row.get_row(file_row).len(), screen_columns);
                // let row_string = format!("{} | {}", row+1, &self.row.get_row(row)[..len]);
                self.buffer
                    .push_str(&self.row.get_row(file_row)[..len])
            }
    
            queue!(self.buffer, terminal::Clear(ClearType::UntilNewLine)).unwrap();

            if row < screen_rows - 1 {
                self.buffer.push_str("\r\n")
            }
        }
    }

    fn update_terminal(&mut self) -> Result<()> {
        self.cursor.scroll();
        execute!(self.buffer, cursor::Hide, cursor::MoveTo(0, 0))?;
        
        let cursor_x = self.cursor.x;
        let cursor_y = self.cursor.y - self.cursor.off_screen;
        
        self.draw_rows();

        queue!(
            self.buffer,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16), 
            cursor::Show
        )?;

        self.buffer.flush()
    }
}

