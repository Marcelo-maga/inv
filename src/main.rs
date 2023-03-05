
mod editor;
mod terminal_functions;


use editor::{ Editor };

fn main() {

  if let Ok(editor) = Editor::new() {
    editor.run()
  }


} 