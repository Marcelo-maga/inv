use std::time::Duration;

use crate::view::View;
use crate::{rows::Row, terminal_functions::TerminalFunctions};

use crossterm::{
    event::{poll, read, Event::*, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

pub struct Editor {
    terminal: TerminalFunctions,
    view: View,
    row: Row,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let terminal = TerminalFunctions::new();
        let view = View::new(terminal.win_size);
        let row = Row::new();

        Ok(Self {
            terminal: terminal,
            view: view,
            row: row,
        })
    }

    pub fn run(&mut self) -> Result<bool> {
        self.view.refresh_screen()?;
        self.process_input()
    }

    // Pensar em uma forma de fazer dentro do arquivo de config que vc ainda vai fazer
    fn process_input(&mut self) -> Result<bool> {
        match self.input_event()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),

            KeyEvent {
                code: direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => self.view.move_cursor(direction, &self.row),

            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: _,
                kind: _,
                state: _,
            } => self.view.insert_char(c),

            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: _,
                kind: _,
                state: _,
            } => self.view.remove_char(),

            _ => {}
        }

        Ok(true)
    }

    fn input_event(&self) -> Result<KeyEvent> {
        loop {
            if poll(Duration::from_millis(100))? {
                if let Ok(event) = read() {
                    if let Key(key_event) = event {
                        // println!("{}", self.row.number_of_rows());
                        return Ok(key_event);
                    }
                }
            }
        }
    }
}
