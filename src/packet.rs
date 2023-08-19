use crate::bitarray::BitArray;

use core::convert::TryInto;
use endian_codec::{DecodeBE, EncodeBE, PackedSize};
use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgOk {
    reserved: u16,
    pub sliceno: u32,
}

impl MsgOk {
    pub fn new(sliceno: u32) -> Self {
        Self {
            reserved: 0,
            sliceno,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgRetransmit {
    reserved: u16,
    pub sliceno: u32,
    pub rxmit: u32,
}

impl MsgRetransmit {
    pub fn new(sliceno: u32, rxmit: u32) -> Self {
        Self {
            reserved: 0,
            sliceno,
            rxmit,
        }
    }
}

#[derive(Debug)]
pub struct Retransmit {
    pub msg: MsgRetransmit,
    pub map: BitArray,
}

impl Retransmit {
    pub fn new(sliceno: u32, rxmit: u32, max_slices: u32) -> Self {
        Self {
            msg: MsgRetransmit::new(sliceno, rxmit),
            map: BitArray::new(max_slices as usize),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgGo {
    reserved: u16,
}

impl MsgGo {
    pub fn new() -> Self {
        Self { reserved: 0 }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgConnectReq {
    reserved: u16,
    pub capabilities: u32,
    pub rcvbuf: u32,
}

impl MsgConnectReq {
    pub fn new(capabilities: u32, rcvbuf: u32) -> Self {
        Self {
            reserved: 0,
            capabilities,
            rcvbuf,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgDisconnect {
    reserved: u16,
}

impl MsgDisconnect {
    pub fn new() -> Self {
        Self { reserved: 0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgReqAck {
    reserved: u16,
    pub sliceno: u32,
    pub bytes: u32,
    pub rxmit: u32,
}

impl MsgReqAck {
    pub fn new(sliceno: u32, bytes: u32, rxmit: u32) -> Self {
        Self {
            reserved: 0,
            sliceno,
            bytes,
            rxmit,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgConnectReply {
    reserved: u16,
    pub clnr: u32,
    pub blocksize: u32,
    pub capabilities: u32,
    pub max_slices: u32,
    pub mcastaddr: [u8; 16],
}

impl MsgConnectReply {
    pub fn new(clnr: u32, blocksize: u32, capabilities: u32, max_slices: u32, mcastaddr: &Ipv4Addr) -> Self {
        let mut buf = [0; 16];
        buf[0..4].copy_from_slice(&mcastaddr.octets());
        Self {
            reserved: 0,
            clnr,
            blocksize,
            capabilities,
            max_slices,
            mcastaddr: buf,
        }
    }

    pub fn mcastaddr(&self) -> Ipv4Addr {
        let octets = u32::from_be_bytes(self.mcastaddr[..4].try_into().unwrap());
        Ipv4Addr::from(octets)
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct DataBlock {
    reserved: u16,
    pub sliceno: u32,
    pub blockno: u16,
    reserved2: u16,
    pub bytes: u32,
}

impl DataBlock {
    pub fn new(sliceno: u32, blockno: u16, bytes: u32) -> Self {
        Self {
            reserved: 0,
            sliceno,
            blockno,
            reserved2: 0,
            bytes,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct FecBlock {
    pub stripes: u16,
    pub sliceno: u32,
    pub blockno: u16,
    reserved2: u16,
    pub bytes: u32,
}

impl FecBlock {
    pub fn new(stripes: u16, sliceno: u32, blockno: u16, bytes: u32) -> Self {
        Self {
            stripes,
            sliceno,
            blockno,
            reserved2: 0,
            bytes,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PackedSize, EncodeBE, DecodeBE)]
pub struct MsgHello {
    reserved: u16,
    pub capabilities: u32,
    mcastaddr: [u8; 16],
    pub blocksize: u16,
}

impl MsgHello {
    pub fn new(capabilities: u32, mcastaddr: &Ipv4Addr, blocksize: u16) -> Self {
        let mut buf = [0; 16];
        buf[0..4].copy_from_slice(&mcastaddr.octets());
        Self {
            reserved: 0,
            capabilities,
            mcastaddr: buf,
            blocksize,
        }
    }

    pub fn mcastaddr(&self) -> Ipv4Addr {
        let octets = u32::from_be_bytes(self.mcastaddr[..4].try_into().unwrap());
        Ipv4Addr::from(octets)
    }
}

#[derive(Debug)]
pub enum Opcode {
    CmdOk,
    CmdRetransmit,
    CmdGo,
    CmdConnectReq,
    CmdDisconnect,
    CmdUnused,
    CmdReqack,
    CmdConnectReply,
    CmdData,
    CmdFec,
    CmdHelloNew,
    CmdHelloStreaming,
    CmdHello = 0x500,
}

#[derive(Debug)]
pub enum Message {
    CmdOk(MsgOk),
    CmdRetransmit(MsgRetransmit),
    CmdGo(MsgGo),
    CmdConnectReq(MsgConnectReq),
    CmdDisconnect(MsgDisconnect),
    CmdUnused,
    CmdReqack(MsgReqAck),
    CmdConnectReply(MsgConnectReply),
    CmdData(DataBlock),
    CmdFec(FecBlock),
    CmdHello(MsgHello),
    None,
}

const OPCODE_LEN: usize = 2;
const OPCODE_VAL: u16 = 0_u16;

impl Message {
    pub fn decode(data: &[u8]) -> (Self, Vec<u8>) {
        let mut data_vec = data.to_vec();
        let (opcode, data) = data_vec.split_at_mut(OPCODE_LEN);
        let opcode = u16::from_be_bytes(opcode.try_into().unwrap());
        let mut data_vec = data.to_vec();
        match opcode {
            0 => (
                Self::CmdOk(MsgOk::decode_from_be_bytes(data)),
                data_vec.split_off(MsgOk::PACKED_LEN),
            ),
            1 => (
                Self::CmdRetransmit(MsgRetransmit::decode_from_be_bytes(data)),
                data_vec.split_off(MsgRetransmit::PACKED_LEN),
            ),
            2 => (
                Self::CmdGo(MsgGo::decode_from_be_bytes(data)),
                data_vec.split_off(MsgGo::PACKED_LEN),
            ),
            3 => (
                Self::CmdConnectReq(MsgConnectReq::decode_from_be_bytes(data)),
                data_vec.split_off(MsgConnectReq::PACKED_LEN),
            ),
            4 => (
                Self::CmdDisconnect(MsgDisconnect::decode_from_be_bytes(data)),
                data_vec.split_off(MsgDisconnect::PACKED_LEN),
            ),
            6 => (
                Self::CmdReqack(MsgReqAck::decode_from_be_bytes(data)),
                data_vec.split_off(MsgReqAck::PACKED_LEN),
            ),
            7 => (
                Self::CmdConnectReply(MsgConnectReply::decode_from_be_bytes(data)),
                data_vec.split_off(MsgConnectReply::PACKED_LEN),
            ),
            8 => (
                Self::CmdData(DataBlock::decode_from_be_bytes(data)),
                data_vec.split_off(DataBlock::PACKED_LEN),
            ),
            9 => (
                Self::CmdFec(FecBlock::decode_from_be_bytes(data)),
                data_vec.split_off(FecBlock::PACKED_LEN),
            ),
            10 => (
                Self::CmdHello(MsgHello::decode_from_be_bytes(data)),
                data_vec.split_off(MsgHello::PACKED_LEN),
            ),
            _ => (Self::None, Vec::new()),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        use Message::*;

        let mut buf = [0; 2048];
        let mut opcode = OPCODE_VAL;
        let mut packet_len: usize = 0;

        match self {
            CmdOk(msg) => {
                opcode = 0;
                packet_len = MsgOk::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdRetransmit(msg) => {
                opcode = 1;
                packet_len = MsgRetransmit::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdGo(msg) => {
                opcode = 2;
                packet_len = MsgGo::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdConnectReq(msg) => {
                opcode = 3;
                packet_len = MsgConnectReq::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdDisconnect(msg) => {
                opcode = 4;
                packet_len = MsgDisconnect::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdReqack(msg) => {
                opcode = 6;
                packet_len = MsgReqAck::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdConnectReply(msg) => {
                opcode = 7;
                packet_len = MsgConnectReply::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdData(msg) => {
                opcode = 8;
                packet_len = DataBlock::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdFec(msg) => {
                opcode = 9;
                packet_len = FecBlock::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            CmdHello(msg) => {
                opcode = 10;
                packet_len = MsgHello::PACKED_LEN;
                msg.encode_as_be_bytes(&mut buf[OPCODE_LEN..]);
            }
            _ => {
                return [0].to_vec();
            }
        }
        buf[0..OPCODE_LEN].copy_from_slice(&opcode.to_be_bytes());
        buf[..packet_len + OPCODE_LEN].to_vec()
    }
}
