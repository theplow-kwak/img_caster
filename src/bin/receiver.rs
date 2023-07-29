use std::time::{Duration, Instant};

use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use img_caster::packet::Message;

use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::io::{self, Read};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};

fn kbcheck(ch: char) -> bool {
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
fn main() {
    // Create a UDP socket
    let mut receiver = MultiCast::receiver();

    let mut buf = [0u8; 1024];
    let mut count = 0;
    let mut start = Instant::now();
    let mut unpacked_message: Option<Message> = None;

    let device_state = DeviceState::new();

    loop {
        match receiver.recv_msg(&mut buf) {
            Ok((size, address)) => {
                unpacked_message = Some(Message::decode(&buf));
                // println!("message {:?}", unpacked_message);
                // println!("serialize {:?}\n", serialize(&unpacked_message).unwrap());
                count += 1;
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if start.elapsed().as_secs() >= 1 {
                    println! {"{count} pps"}
                    if let Some(ref msg) = unpacked_message {
                        println!("message {:?}", msg);
                    }
                    start = Instant::now();
                    count = 0;
                    if kbcheck('q') {
                        break;
                    }
                }
            }
            Err(err) => {
                // Handle other errors
                // ...
            }
        }

    }
}
