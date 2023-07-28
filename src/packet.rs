use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

// Define a struct representing the UDP message
#[derive(Debug, Serialize, Deserialize)]
pub struct MsgOk {
    field1: u32,
    field2: u32,
    field3: [u8; 32],
}

impl MsgOk {
    pub fn new(field1: u32, field2: u32, field3: [u8; 32]) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgHello {
    field1: u16,
    field2: u16,
    field3: [u8; 4],
}

impl MsgHello {
    pub fn new(field1: u16, field2: u16, field3: [u8; 4]) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgRetransmit {
    field1: u16,
    field2: [u8; 16],
    field3: u32,
    // ...
}

impl MsgRetransmit {
    pub fn new(field1: u16, field2: [u8; 16], field3: u32) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgReqack {
    field1: u16,
    field2: u16,
    field3: [u8; 8],
}

impl MsgReqack {
    pub fn new(field1: u16, field2: u16, field3: [u8; 8]) -> Self {
        Self {
            field1,
            field2,
            field3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgReady {
    field1: u16,
    field2: u16,
    field3: [u8; 8],
    field4: [u8; 8],
}

impl MsgReady {
    pub fn new(field1: u16, field2: u16, field3: [u8; 8], field4: [u8; 8]) -> Self {
        Self {
            field1,
            field2,
            field3,
            field4,
        }
    }
}

#[repr(u16)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ok(MsgOk),
    Hello(MsgHello),
    Retransmit(MsgRetransmit),
    Reqack(MsgReqack),
    Ready(MsgReady),
    None,
}

const OPCODE_LEN:usize = 4;

use core::convert::TryInto;

impl Message {
    pub fn decode(data: &[u8]) -> Self {
        let mut data = data.to_vec();
        let (opcode, data) = data.split_at_mut(OPCODE_LEN);
        let opcode = u32::from_le_bytes(opcode.try_into().unwrap());
        match opcode {
            0 => Self::Ok(deserialize(data).unwrap()),
            1 => Self::Hello(deserialize(data).unwrap()),
            2 => Self::Retransmit(deserialize(data).unwrap()),
            3 => Self::Reqack(deserialize(data).unwrap()),
            4 => Self::Ready(deserialize(data).unwrap()),
            _ => Self::None,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        serialize(self).unwrap()
    }
}
