use std::io::{self, Read};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

pub mod disk;
pub mod multicast;
pub mod packet;
pub mod bitarray;
pub mod databuffer;
pub mod statistics;

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

pub fn format_size(size_in_bytes: u64) -> (f64, &'static str) {
    const GB: u64 = 1_000_000_000;
    const MB: u64 = 1_000_000;
    const KB: u64 = 1_000;

    if size_in_bytes >= GB {
        let size_gb = size_in_bytes as f64 / GB as f64;
        return (size_gb, "GB");
    } else if size_in_bytes >= MB {
        let size_mb = size_in_bytes as f64 / MB as f64;
        return (size_mb, "MB");
    } else if size_in_bytes >= KB {
        let size_kb = size_in_bytes as f64 / KB as f64;
        return (size_kb, "KB");
    } else {
        return (size_in_bytes as f64, "Bytes");
    }
}
