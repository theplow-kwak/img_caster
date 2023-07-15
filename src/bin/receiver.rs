use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use std::time::{Duration, Instant};

use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize, Result};

// Define a struct representing the UDP message
#[derive(Debug, Serialize, Deserialize)]
struct UdpMessage {
    // Add fields as needed
    field1: u32,
    field3: u16,
    field4: u16,
    field2: String,
    // ...
}

fn pack_message(message: &UdpMessage) -> Result<Vec<u8>> {
    serialize(message)
}

fn unpack_message(data: &[u8]) -> Result<UdpMessage> {
    deserialize(data)
}


fn main() {
    // Create a UDP socket
    let mut receiver = MultiCast::receiver();

    let mut buf = [0u8; 1024];
    let mut count = 0;
    let mut start = Instant::now();
    loop {
        let (size, src_addr) = receiver.recv_msg(&mut buf);
        let unpacked_message = unpack_message(&buf).unwrap();
        count += 1;
        if start.elapsed().as_secs() >= 1 {
            println!{"{count} pps"}
            start = Instant::now();
            count = 0;
        } 
    }
}
