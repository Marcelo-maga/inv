use crossterm::{ terminal };

pub struct TerminalFunctions {

}

impl TerminalFunctions {

  pub fn start_raw_mode() {
    terminal::enable_raw_mode();
  }

  pub fn finish_raw_mode() {
    terminal::disable_raw_mode();
  }

}