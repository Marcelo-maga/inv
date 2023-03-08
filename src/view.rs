use std::io::{stdout, Write};

use crossterm::terminal::ClearType;
use crossterm::{ terminal, execute, Result, cursor };

pub struct View {
  win_size: (usize, usize)
}

impl View {
  pub fn new(win_size:(usize, usize)) -> Self {

    Self {
      win_size: win_size
    }
  }

  pub fn refresh_screen(&self) -> Result<()>{
    self.update_terminal()
  }
  
  fn draw_rows(&self) {
    let screen_rows = self.win_size.1;

    for number_line in 0..screen_rows {
        print!("{}", number_line+1);

        if number_line < screen_rows - 1 {
          println!("\r")
        }
        
        stdout().flush();

    }
  }

  fn update_terminal(&self) -> Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::UntilNewLine))?;

    self.draw_rows();

    execute!(stdout(), cursor::MoveTo(0, 0))
  }


}