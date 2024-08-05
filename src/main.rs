use crossterm::Result;

mod buffer;
mod cursor;
mod editor;
mod rows;
mod terminal_functions;
mod view;

use editor::Editor;
use terminal_functions::TerminalFunctions;

fn main() -> Result<()> {
    if let Ok(mut editor) = Editor::new() {
        while editor.run()? {}
    }

    TerminalFunctions::finish_raw_mode();

    Ok(TerminalFunctions::clear_terminal())
}
