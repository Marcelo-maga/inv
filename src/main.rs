use crossterm::{ Result };


mod editor;
mod terminal_functions;


use editor::{ Editor };

fn main() -> Result<()> {

  if let Ok(editor) = Editor::new() {
    while editor.run()? {}
  }

  Ok(())

} 