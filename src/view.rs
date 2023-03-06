use std::io::stdout;

use crossterm::terminal::ClearType;
use crossterm::{ terminal, execute, Result, cursor };

pub struct View;

impl View {
  pub fn new() -> Self {
    Self
  }

  pub fn refresh_screen(&self) -> Result<()>{
    self.update_terminal()
  }
  
  fn update_terminal(&self) -> Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All));
    execute!(stdout(), cursor::MoveTo(0, 0))
  }


}