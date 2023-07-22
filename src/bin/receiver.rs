use std::time::{Duration, Instant};

use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use img_caster::packet::Message;

use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

fn main() {
    // Create a UDP socket
    let mut receiver = MultiCast::receiver();

    let mut buf = [0u8; 1024];
    let mut count = 0;
    let mut start = Instant::now();
    loop {
        let (size, src_addr) = receiver.recv_msg(&mut buf);
        let unpacked_message = Message::decode(&buf);
        println!("message {:?}", unpacked_message);
        println!("serialize {:?}\n", serialize(&unpacked_message).unwrap());
        count += 1;
        if start.elapsed().as_secs() >= 1 {
            println! {"{count} pps"}
            start = Instant::now();
            count = 0;
        }
    }
}
