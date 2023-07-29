use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::{Duration, Instant};

use img_caster::disk::DiskHandler;
use img_caster::multicast::MultiCast;
use img_caster::packet;

use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

fn main() {
    // Create a UDP socket
    let mut sender = MultiCast::sender();

    let MsgOk = packet::MsgOk::new(2, 0x1234, [0; 32]);
    let message = packet::Message::Ok(MsgOk).encode();
    sender.send_msg(&message);

    let MsgReady = packet::MsgReady::new(9, 0x5678, [4; 8], [7; 8]);
    let message = packet::Message::Ready(MsgReady).encode();
    sender.send_msg(&message);

    let MsgHello = packet::MsgHello::new(12, 0xabcd, [6; 4]);
    let message = packet::Message::Hello(MsgHello).encode();
    sender.send_msg(&message);

    let MsgRetransmit = packet::MsgRetransmit::new(0x100, [0xa; 16], 0xffbb);
    let message = packet::Message::Retransmit(MsgRetransmit).encode();
    sender.send_msg(&message);

    let mut count = 0;
    let mut start = Instant::now();
    loop {
        // Pack the message into a byte vector
        let MsgOk = packet::MsgOk::new(2, 0x1234, [0; 32]);
        let message = packet::Message::Ok(MsgOk).encode();
        sender.send_msg(&message);
        count += 1;
        if start.elapsed().as_secs() >= 1 {
            println!{"{count} pps"}
            start = Instant::now();
            count = 0;
        }
    }
}
