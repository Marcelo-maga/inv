use crossterm::{
    cursor,
    event::KeyCode,
    execute, queue,
    terminal::{self, ClearType},
    Result,
};
use std::cmp;
use std::io::Write;

use crate::{buffer::Buffer, rows::Row};

struct EditorCursor {
    x: usize,
    y: usize,
    screen_columns: usize,
    screen_rows: usize,
    y_off_screen: usize,
    x_off_screen: usize,
}

impl EditorCursor {
    fn new(screen_columns: usize, screen_rows: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            screen_columns: screen_columns,
            screen_rows: screen_rows,
            y_off_screen: 0,
            x_off_screen: 0,
        }
    }

    // Implementar uma keybind, e usar control e as teclas do VIM
    // pela quantidade de atalhos, isso tera que ser uma impl
    fn move_cursor(&mut self, direction: KeyCode, row: &Row) {
        let number_of_rows = row.number_of_rows();
        let number_of_chars = row.number_of_chars(self.y);

        match direction {
            KeyCode::Up => {
                self.y = self.y.saturating_sub(1);
            }

            KeyCode::Left => {
                if self.x != 0 {
                    self.x -= 1;
                } else if self.y > 0 {
                    self.y -= 1;
                    self.x = row.get_row(self.y).len();
                }
            }

            KeyCode::Down => {
                if self.y < number_of_rows-1 {
                    self.y += 1;
                }
            }

            KeyCode::Right => {
                if self.y < number_of_rows && self.x < row.get_row(self.y).len() {
                    self.x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }

    fn scroll(&mut self) {
        self.y_off_screen = cmp::min(self.y_off_screen, self.y);
        if self.y >= self.y_off_screen + self.screen_rows {
            self.y_off_screen = self.y - self.screen_rows + 1;
        }

        self.x_off_screen = cmp::min(self.x_off_screen, self.x);
        if self.x >= self.x_off_screen + self.screen_rows {
            self.x_off_screen = self.x - self.screen_rows + 1;
        }
    }
}

pub struct View {
    win_size: (usize, usize),
    buffer: Buffer,
    cursor: EditorCursor,
    row: Row,
}

impl View {
    pub fn new(win_size: (usize, usize)) -> Self {
        let buffer = Buffer::new();
        let row = Row::new();

        Self {
            win_size: win_size,
            buffer: buffer,
            cursor: EditorCursor::new(win_size.0, win_size.1),
            row: row,
        }
    }

    pub fn refresh_screen(&mut self) -> Result<()> {
        self.update_terminal()
    }

    pub fn move_cursor(&mut self, direction: KeyCode, row: &Row) {
        self.cursor.move_cursor(direction, row)
    }

    pub fn insert_char(&mut self, c: char) {
        self.row.insert_char(self.cursor.y, self.cursor.x, c);
        self.cursor.x += 1;
    }

    pub fn remove_char(&mut self) {
        self.row.remove_char(self.cursor.y, self.cursor.x-1);

        if self.cursor.x < self.row.number_of_chars(self.cursor.y) {
            self.cursor.y -= 1;
            self.cursor.x = self.row.number_of_chars(self.cursor.y);
            
        } else {
            self.cursor.x -= 1;
        }
    }

    fn draw_rows(&mut self) {
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;

        for row in 0..screen_rows {
            let file_row = row + self.cursor.y_off_screen;
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
                let row_string = self.row.get_row(file_row);
                let column_offset = self.cursor.x_off_screen;

                let len = if row_string.len() < column_offset {
                    0
                } else {
                    let len = row_string.len() - column_offset;

                    if len > screen_columns {
                        len - screen_columns
                    } else {
                        len
                    }
                };

                let start = if len == 0 { 0 } else { column_offset };

                // let row_string = format!("{} | {}", row+1, &row_string[start..start + len]);

                let row_string = format!("{}", &row_string[start..start + len]);
                self.buffer.push_str(&row_string);
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

        let cursor_x = self.cursor.x - self.cursor.x_off_screen;
        let cursor_y = self.cursor.y - self.cursor.y_off_screen;

        self.draw_rows();

        queue!(
            self.buffer,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;

        self.buffer.flush()
    }
}
