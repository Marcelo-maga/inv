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
  quit: bool
}

impl Editor {

  pub fn new() -> Result<()> {
    TerminalFunctions::new();

    Editor::process_input();

    Ok(())
  }

  fn process_input() {
    
  }

  fn input_event() -> Result<KeyEvent> { // Evento para registar cada ms que passou e as teclas que foram precionadas

    loop {
      
      if poll(Duration::from_millis(100))? {

          if let Ok(event) = read() {
            if let Key(key_event) = event { 
              return Ok(key_event);
            } 
          } else {
           TerminalFunctions::die("Algo de errado ocorreu na leitura")
          }

        }
      }
  }

}