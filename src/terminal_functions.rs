
// execute == lambda in python????
use crossterm::{ terminal, execute };
use errno::errno;
use std::io::{ stdout };

pub struct TerminalFunctions {
  pub win_size: (usize, usize)
}

impl TerminalFunctions {
  pub fn new() -> Self {
    TerminalFunctions::start_raw_mode();

    let win_size = terminal::size()
    .map(|(x, y)| (x as usize, y as usize))
    .unwrap(); 
    

    Self {
      win_size: win_size
    }
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


  pub fn finish_raw_mode() {
    let _  = terminal::disable_raw_mode();
  }

}