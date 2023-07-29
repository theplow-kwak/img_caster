use std::io::{self, Read};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

fn main() {
    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent { code, modifiers, ..}) = event::read().unwrap() {
                if let KeyCode::Char(c) = code {
                    println!("Character pressed: {}", c);
                    if c == 'q' {
                        break; // Exit the loop if 'q' is pressed
                    }
                }
                // You can also check for other key events here
                // e.g., KeyCode::Up, KeyCode::Down, KeyCode::Enter, etc.
            }
        }
    }
}
