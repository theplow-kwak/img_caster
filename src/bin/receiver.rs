use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::time::{Duration, Instant};

use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use img_caster::packet::Message;

use bincode::{deserialize, serialize, Result};
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};

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
                    print!{"\r{count} pps"}
                    // if let Some(ref msg) = unpacked_message {
                    //     println!("message {:?}", msg);
                    // }
                    io::stdout().flush().unwrap();
                    start = Instant::now();
                    count = 0;
                    if img_caster::kbdcheck('q') {
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
