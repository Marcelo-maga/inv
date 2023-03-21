use crossterm::{ Result };


mod editor;
mod terminal_functions;
mod view;
mod buffer;


use editor::{ Editor };
use terminal_functions::TerminalFunctions;

fn main() -> Result<()> {

  if let Ok(mut editor) = Editor::new() {
    while editor.run()? {}
  }

  Ok(
    TerminalFunctions::finish_raw_mode()
  )

} 