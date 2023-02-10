use std::time::Duration;

use crossterm::{ 
    event::{ 
        read,
        Event::*,
        KeyCode, 
        poll
    },
    terminal::{ enable_raw_mode, disable_raw_mode },
    Result
};


fn main() -> Result<()> {
    enable_raw_mode()?;

    let mut seconds_count: i32 = 0;

    loop {
        seconds_count += 1;
        if let Ok(true) = poll(Duration::from_millis(100)) {

            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    } else {
                        println!("{seconds_count:4 } {:?}\r", key_event);
                    }
                }
            }
        }
    }

    disable_raw_mode()?;

    Ok(())
}