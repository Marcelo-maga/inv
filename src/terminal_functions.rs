
// execute == lambda in python????
use crossterm::terminal::ClearType;
use crossterm::{ terminal, execute, Result };
use errno::errno;
use std::io::{ stdout };

pub struct TerminalFunctions {
  
}

impl TerminalFunctions {
  pub fn new() -> Result<Self> {
    TerminalFunctions::start_raw_mode();

    Ok(Self {  })

  }

  pub fn die<S: Into<String>>(message: S) { // Esteja sempre preparado ... para tudo!
    TerminalFunctions::finish_raw_mode(); // Fecha o modo raw
    eprintln!("{}: {}", message.into(), errno()); // Mostra o erro
    std::process::exit(1) // Fecha o editor
  }

  fn start_raw_mode() {
    let _ = terminal::enable_raw_mode();
    execute!(stdout(), terminal::EnterAlternateScreen).unwrap() // Função para abrir uma nova "tela" obrigado Ox
  }

  pub fn update_terminal() -> Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))
  }

  pub fn finish_raw_mode() {
    let _  = terminal::disable_raw_mode();
  }

}