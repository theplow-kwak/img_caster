use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
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
    let mut sender = MultiCast::sender();

    let filename = "\\\\.\\PhysicalDrive0";
    let rwflag = 'r';
    // Open the physical drive with appropriate options
    let mut disk = DiskHandler::new(filename.to_string(), rwflag);
    let mut buffer = [0u8; 2048];

    disk.open();
    disk.read(&mut buffer).expect("Failed to read from physical drive");

    let message = UdpMessage {
        field1: 42,
        field3: 0x1234,
        field4: 0xabcd,
        field2: String::from("Hello, UDP!"), 
    };

    loop {
        // Pack the message into a byte vector
        let packed_message = pack_message(&message).unwrap();
        sender.send_msg(&packed_message);
    }
}
