use std::time::Duration;

use crate::terminal_functions::TerminalFunctions;
use crate::view::View;

use crossterm::{ 
  event::{ 
      read,
      Event::*,
      KeyCode, 
      poll, KeyEvent, KeyModifiers
  },
  Result
};

pub struct Editor {
  terminal: TerminalFunctions,
  view: View
}

impl Editor {

  pub fn new() -> Result<Self> {
    let terminal = TerminalFunctions::new();
    let view = View::new();

    Ok(
      Self {
        terminal: terminal,
        view: view
      }
    )

  }

  pub fn run(&self) -> Result<bool> {
    self.view.refresh_screen();
    self.process_input()
  }

  fn process_input(&self) -> Result<bool>{
    let key = self.input_event()?;

    if key.code == KeyCode::Char('q') && key.modifiers == KeyModifiers::CONTROL {
      Ok(false)
    } else {
      Ok(true)
    }
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