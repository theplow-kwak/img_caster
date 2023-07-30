use std::io::{self, Read};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

pub mod disk;
pub mod multicast;
pub mod packet;
pub mod bitarray;
pub mod databuffer;

pub fn kbdcheck(ch: char) -> bool {
    if event::poll(std::time::Duration::from_millis(0)).unwrap() {
        if let event::Event::Key(KeyEvent { code, modifiers, ..}) = event::read().unwrap() {
            if let KeyCode::Char(c) = code {
                println!("Character pressed: {}", c);
                if c == ch {
                    return true; // Exit the loop if 'q' is pressed
                }
            }
            // You can also check for other key events here
            // e.g., KeyCode::Up, KeyCode::Down, KeyCode::Enter, etc.
        }
    }
    return false;
}
