use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

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
    // Create an instance of the UdpMessage struct
    let message = UdpMessage {
        field1: 42,
        field3: 0x1234,
        field4: 0xabcd,
        field2: String::from("Hello, UDP!"),
    };

    // Pack the message into a byte vector
    let packed_message = pack_message(&message).unwrap();

    println! {"UdpMessage {:#?} \npacked_message {:?}", message, packed_message}

    let unpacked_message = unpack_message(&packed_message).unwrap();
    println! {"unpacked_message {:#?}", unpacked_message}
}
