use std::time::Duration;

use crate::terminal_functions::TerminalFunctions;

use crossterm::{ 
  event::{ 
      read,
      Event::*,
      KeyCode, 
      poll, KeyEvent
  },
  Result
};

pub struct Editor {
  quit: bool,
  terminal: TerminalFunctions
}

impl Editor {

  pub fn new() -> Result<Self> {
    let terminal = TerminalFunctions::new()?;

    Ok(
      Self {
        quit: false,
        terminal: terminal
      }
    )

  }

  pub fn run(&self) {
    self.process_input()
  }

  fn process_input(&self) {
    let mut key = self.input_event();

    println!("{:?}", key);
  }

  fn input_event(&self) -> Result<KeyEvent> { // Evento para registar cada ms que passou e as teclas que foram precionadas

    loop {
      if poll(Duration::from_millis(100))? {
          if let Ok(event) = read() {
            if let Key(key_event) = event { 
              return Ok(key_event);
            } 
          }
        }
      }
  }

}