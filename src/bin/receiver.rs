use std::io;
use std::io::Write; // <--- bring flush() into scope
use std::time::{Duration, Instant};

use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use img_caster::packet::Message;
use img_caster::statistics;

use bincode::{deserialize, serialize, Result};
use device_query::{DeviceQuery, DeviceState, Keycode};
use serde::{Deserialize, Serialize};

fn main() {
    // Create a UDP socket
    let mut receiver = MultiCast::receiver();

    let mut buf = [0u8; 1024];
    let mut count = 0;
    let mut start = Instant::now();
    let mut elapstime = Instant::now();
    let mut unpacked_message: Option<Message> = None;
    let mut receive_bytes: usize = 0;
    let mut state = statistics::State::default();
    let mut latency = Instant::now();
    let mut snap = state.now();

    loop {
        latency = Instant::now();
        match receiver.recv_msg(&mut buf) {
            Ok((size, address)) => {
                state.add_net(latency.elapsed(), size);
                unpacked_message = Some(Message::decode(&buf));
                // println!("message {:?}", unpacked_message);
                // println!("serialize {:?}\n", serialize(&unpacked_message).unwrap());
                count += 1;
                receive_bytes += size;
                state.add_io(latency.elapsed(), size);
                if elapstime.elapsed().as_secs() >= 1 {
                    let (size, unit) = img_caster::format_size(receive_bytes as u64);
                    print!("Bytes = {size} {unit}, {count} pps ");
                    print!("state {:?}", state.now() - snap);
                    print!("\n");
                    io::stdout().flush().unwrap();
                    elapstime = Instant::now();
                    snap = state.now();
                    count = 0;
                    if img_caster::kbdcheck('q') {
                        break;
                    }
                }
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if elapstime.elapsed().as_secs() >= 1 {
                    let (size, unit) = img_caster::format_size(receive_bytes as u64);
                    print!("Bytes = {size} {unit}, {count} pps ");
                    print!("state {:?}", state.now() - snap);
                    print!("\n");
                    io::stdout().flush().unwrap();
                    elapstime = Instant::now();
                    snap = state.now();
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
