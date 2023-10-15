use console_input::keypress::exit_raw_mode;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

// TODO: make use of console-input here
pub fn pause() {
    println!("-- Paused (Esc to unpause) --\r");
    loop {
        let pressed_key = Some(read().unwrap());
        if let Some(Event::Key(event_key)) = pressed_key {
            match event_key {
                KeyEvent {
                    code: KeyCode::Esc,
                    kind: KeyEventKind::Press,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char('c'), // Close
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    ..
                } => exit_raw_mode(),
                _ => (),
            }
        }
    }
}
