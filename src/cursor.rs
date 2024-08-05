// TODO -> Implementar funções para manter os atributos privados
use crossterm::event::KeyCode;
use std::cmp;

use crate::rows::Row;

pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub screen_columns: usize,
    pub screen_rows: usize,
    pub y_off_screen: usize,
    pub x_off_screen: usize,
}

impl Cursor {
    pub fn new(screen_columns: usize, screen_rows: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            screen_columns: screen_columns,
            screen_rows: screen_rows,
            y_off_screen: 0,
            x_off_screen: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, row: &Row) {
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
                if self.y < number_of_rows - 1 {
                    self.y += 1;
                } else {
                    self.y = number_of_rows - 1;
                }
            }

            KeyCode::Right => {
                if self.y < number_of_rows && self.x < row.get_row(self.y).len() {
                    self.x += 1;
                } else if self.x == number_of_chars && self.y != number_of_rows - 1 {
                    self.y += 1;
                    self.x = 0;
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn scroll(&mut self) {
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
