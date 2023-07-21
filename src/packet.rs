use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize, Result};

// Define a struct representing the UDP message
#[derive(Debug, Serialize, Deserialize)]
pub struct msg1 {
    // Add fields as needed
    opcode: u16,
    field1: u32,
    field2: u32,
    field3: [u8; 32],
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct msg2 {
    // Add fields as needed
    opcode: u16,
    field1: u16,
    field2: u16,
    field3: [u8; 4],
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct msg3 {
    // Add fields as needed
    opcode: u16,
    field1: u16,
    field2: [u8; 16],
    field3: u32,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct msg4 {
    // Add fields as needed
    opcode: u16,
    field1: u16,
    field2: u16,
    field3: [u8; 8],
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct msg5 {
    // Add fields as needed
    opcode: u16,
    field1: u16,
    field2: u16,
    field3: [u8; 8],
    field4: [u8; 8],
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub enum message {
    ok(msg1),
    hello(msg2),
    retransmit(msg3),
    reqack(msg4),
    ready(msg5),
    none,
}

use core::convert::TryInto;

impl message {
    pub fn parse(data: &[u8]) -> Self {
        match u16::from_le_bytes(data[..2].try_into().unwrap()) {
            0 => Self::ok(deserialize(data).unwrap()),
            1 => Self::hello(deserialize(data).unwrap()),
            2 => Self::retransmit(deserialize(data).unwrap()),
            3 => Self::reqack(deserialize(data).unwrap()),
            4 => Self::ready(deserialize(data).unwrap()),
            _ => Self::none,
        }
    }
}

