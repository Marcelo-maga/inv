use std::time::Duration;

use crate::{ terminal_functions::TerminalFunctions };
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

  pub fn new_editor() -> Result<()> {
    TerminalFunctions::start_raw_mode();

    let mut seconds_count: i32 = 0;
    
    loop {
      let mut key = None;
      seconds_count += 1;
      
      if let Ok(true) = poll(Duration::from_millis(100)) {
          if let Ok(event) = read() {
             if let Key(key_event) = event {
              key = Some(key_event)
             }
          }
      }

      if let Some(key) = key {
          if key.code == KeyCode::Char('q') {
              break;
          } else { 
              println!("{seconds_count:4} {key:?}\r")
          }
      }

    }

    TerminalFunctions::finish_raw_mode();

    Ok(())

  }

}