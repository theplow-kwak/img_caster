use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;

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

    let mut buf = [0u8; 2048];
    loop {
        let (size, src_addr) = receiver.recv_msg(&mut buf);
        let unpacked_message = unpack_message(&buf).unwrap();
        println!{"unpacked_message {:#?}", unpacked_message}
    }
}
