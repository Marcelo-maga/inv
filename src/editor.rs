use std::time::Duration;

use crate::terminal_functions::TerminalFunctions;

use crossterm::{ 
  event::{ 
      read,
      Event::*,
      KeyCode, 
      poll
  },
  Result
};

pub struct Editor {

}

impl Editor {

  pub fn new() -> Result<()> {
    TerminalFunctions::new();

    Editor::input_event();

    Ok(())

  }

  fn input_event() { // Evento para registar cada ms que passou e as teclas que foram precionadas
    let mut ms_count: i32 = 0;
    
    loop {
      let mut key = None;
      ms_count += 1;
      
      match poll(Duration::from_millis(100)) {
        Ok(true) => {
          if let Ok(event) = read() {
            if let Key(key_event) = event {
             key = Some(key_event)
            } 
          } else {
           TerminalFunctions::die("Read Fail")
          }
        }
        Ok(false) => {}
          _ => {
            TerminalFunctions::die("Poll Fail")
        }
       }

      if let Some(key) = key {
          if key.code == KeyCode::Char('q') {
              break;
          } else { 
              println!("{ms_count:4} {key:?}\r")
          }
      }

    }

    TerminalFunctions::finish_raw_mode()    
  }

}