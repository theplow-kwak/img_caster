use byte_unit::Byte;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use std::net::SocketAddrV4;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use crate::bitarray::BitArray;
use crate::datafifo::DataFIFO;
use crate::multicast::*;
use crate::packet::*;
use crate::slice::Slice;
use crate::*;

#[derive(Debug)]
pub struct McastSender {
    pub socket: MultiCast,
    data_fifo: DataFIFO,
    blocksize: u32,
    capabilities: u32,
    clientlist: HashMap<SocketAddrV4, (usize, u32, u32)>,
    slices: HashMap<u32, Slice>,
    xmit_slice: i32,
    slice_size: u32,
    max_slices: u32,
    pub read_chunk: usize,
    retransmits: u32,
    start_time: Instant,
    elaps_time: Instant,
    lastsendtime: Instant,
    written_elaps: u128,
    rx: std::sync::mpsc::Receiver<Vec<u8>>,
}

impl McastSender {
    pub fn new(
        nic: usize,
        ttl: u32,
        max_slices: u32,
        rx: std::sync::mpsc::Receiver<Vec<u8>>,
    ) -> Self {
        let socket = MultiCast::sender(nic);
        let _ = socket.set_ttl(ttl);

        Self {
            socket,
            max_slices,
            blocksize: BLOCK_SIZE,
            capabilities: 0,
            retransmits: 0,
            slice_size: 130,
            read_chunk: CHUNK_SIZE * 16,
            xmit_slice: -1,
            clientlist: HashMap::new(),
            slices: HashMap::new(),
            data_fifo: DataFIFO::new(),
            start_time: Instant::now(),
            elaps_time: Instant::now(),
            lastsendtime: Instant::now(),
            written_elaps: 0,
            rx,
        }
    }

    pub fn enumerate(&mut self, timeout: Duration, p2p: bool) -> Result<usize, &'static str> {
        let mut buff = [0u8; UDP_PACK_SIZE];
        let _ = self.send_hello();
        self.elaps_time = Instant::now();
        loop {
            if let Some(c) = getch(0) {
                if c == '\r' {
                    break;
                }
            }
            if self.elaps_time.elapsed() > timeout {
                break;
            }
            if let Ok((msg, _remain)) = self.socket.recv_msg(&mut buff) {
                match msg {
                    Message::CmdConnectReq(m) => {
                        let clientaddr = self.socket.receivefrom.unwrap();
                        if !self.clientlist.contains_key(&clientaddr) {
                            let client_no = self.clientlist.len();
                            self.clientlist
                                .insert(clientaddr, (client_no, m.capabilities, m.rcvbuf));
                            info!(
                                "New client #{client_no} connected: {} {:?}",
                                clientaddr,
                                self.clientlist.get(&clientaddr)
                            );
                        }
                        if let Some(client) = self.clientlist.get(&clientaddr) {
                            let _ = self.send_connectreply(client.0 as u32);
                        }
                    }
                    Message::CmdDisconnect(_m) => {
                        let clientaddr = self.socket.receivefrom.unwrap();
                        if let Some(client) = self.clientlist.get(&clientaddr) {
                            info!("remove client #{}: {:?}", client.0, client);
                            self.clientlist.remove(&clientaddr);
                        }
                    }
                    Message::CmdGo(_m) => {
                        info!("Let's Go");
                        break;
                    }
                    _ => {}
                }
                println!("\nReady. Press 'Enter' to start sending data. or It will start automatically after {} seconds.\n", timeout.as_secs());
                self.elaps_time = Instant::now();
            }
        }

        info!("{} clients found", self.clientlist.len());
        info!("{:?}\n", self.clientlist);
        self.start_time = Instant::now();

        let clients = self.clientlist.len();
        if clients == 1 && p2p {
            self.socket.multicast_addr = self.socket.receivefrom.unwrap();
        }

        if clients > 0 {
            Ok(clients)
        } else {
            Err("There is no clients!!")
        }
    }

    pub fn send_hello(&mut self) -> io::Result<usize> {
        let msg = packet::Message::CmdHello(packet::MsgHello::new(
            self.capabilities,
            self.socket.multicast_addr.ip(),
            self.blocksize as u16,
        ));
        self.socket
            .send_to(&msg.encode(), self.socket.broadcast_addr)
    }

    pub fn send_disconnect(&mut self, sendto: SocketAddrV4) -> io::Result<usize> {
        let slice_no = self.slices.len() as u32;
        let reqack = packet::MsgReqAck::new(slice_no, 0, 0);
        let mut msg = packet::Message::CmdReqack(reqack).encode();
        let mut ready_set = BitArray::new(MAX_CLIENTS as usize);
        msg.append(&mut ready_set.bits());
        self.socket.send_to(&msg, sendto)
    }

    pub fn send_reqack(&mut self) -> io::Result<usize> {
        if self.xmit_slice >= 0 {
            let xmit_slice = self.xmit_slice as u32;
            let slice = self.slices.get_mut(&xmit_slice).unwrap();
            slice.reqack.rxmit = slice.rxmit_id;
            if self.retransmits == 0 {
                self.slice_size += self.slice_size / 4;
                if self.slice_size > self.max_slices {
                    self.slice_size = self.max_slices;
                }
            }
            if slice.last_good_block > 0 && slice.last_good_block < slice.blocks_in_slice {
                if slice.last_good_block < self.slice_size / 2 {
                    self.slice_size = self.slice_size / 2;
                } else {
                    self.slice_size = slice.last_good_block;
                }
            }
            if self.slice_size < 32 {
                self.slice_size = 32;
            }
            slice.last_good_block = 0;
            let mut msg = packet::Message::CmdReqack(slice.reqack).encode();
            msg.append(&mut slice.ready_set.bits());
            self.socket.send_to(&msg, self.socket.multicast_addr)
        } else {
            Err(Error::new(ErrorKind::Other, "There is no xmit_slice!"))
        }
    }

    pub fn send_connectreply(&mut self, clnr: u32) -> io::Result<usize> {
        let msg = packet::Message::CmdConnectReply(packet::MsgConnectReply::new(
            clnr,
            self.blocksize as u32,
            self.capabilities,
            self.max_slices,
            self.socket.multicast_addr.ip(),
        ));
        if let Some(receivefrom) = self.socket.receivefrom {
            self.socket.send_to(&msg.encode(), receivefrom)
        } else {
            Err(Error::new(ErrorKind::Other, "There is no receivefrom!"))
        }
    }

    pub fn send_datablock(&mut self, blockno: u32) -> io::Result<usize> {
        if self.xmit_slice >= 0 {
            let xmit_slice = self.xmit_slice as u32;
            let slice = self.slices.get_mut(&xmit_slice).unwrap();
            let mut msg = packet::Message::CmdData(packet::DataBlock::new(
                slice.slice_no,
                blockno as u16,
                slice.bytes,
            ))
            .encode();
            let data = self
                .data_fifo
                .get(slice.get_block_pos(blockno), self.blocksize as u32);
            msg.append(&mut data.to_vec());
            self.socket.send_to(&msg, self.socket.multicast_addr)
        } else {
            Err(Error::new(ErrorKind::Other, "There is no xmit_slice!"))
        }
    }

    pub fn display_progress(&mut self, final_disp: bool) {
        let elapsed = self.elaps_time.elapsed();
        if elapsed.as_secs() > 0 || final_disp {
            let difftime = self.start_time.elapsed();
            if difftime.as_millis() == 0 {
                return;
            }
            let writtenbytes = self.data_fifo.written_bytes() as u128;
            let mbps = writtenbytes / difftime.as_millis();
            print!(
                "Total: {} ({}.{:0<3} MB/s) {:>6} pps, slicesize={}",
                Byte::from_bytes(writtenbytes)
                    .get_appropriate_unit(false)
                    .to_string(),
                mbps / 1000,
                mbps % 1000,
                self.socket.packet_count,
                self.slice_size
            );
            if elapsed.as_millis() > 0 {
                let mbps = (writtenbytes - self.written_elaps) / elapsed.as_millis();
                print!(", elaps: ({}.{:0<3} MB/s)", mbps / 1000, mbps % 1000,);
            }
            print!("{}\n", " ".repeat(10));
            self.written_elaps = writtenbytes;
            let _ = std::io::stdout().flush();
            self.elaps_time = Instant::now();
            self.socket.packet_count = 0;
        }
        if final_disp {
            println!("\n");
            info!(
                "{} transferd in {:?}",
                Byte::from_bytes(self.data_fifo.written_bytes() as u128)
                    .get_appropriate_unit(false)
                    .to_string(),
                self.start_time.elapsed()
            );
        }
    }

    fn make_slice(&mut self, block_size: u32, slice_size: u32) -> &mut Slice {
        let mut slice_size = slice_size;
        if block_size as u32 * slice_size
            > (self.data_fifo.endpoint - self.data_fifo.slicebase) as u32
        {
            slice_size = (self.data_fifo.endpoint - self.data_fifo.slicebase) as u32 / block_size;
        }
        let mut size = block_size * slice_size;
        if size == 0 {
            size = (self.data_fifo.endpoint - self.data_fifo.slicebase) as u32;
        }
        let slice_no = self.slices.len() as u32;
        let slice = Slice::new(
            slice_no,
            size,
            block_size,
            self.data_fifo.slicebase,
            self.max_slices,
        );
        self.data_fifo.assign(size);
        self.slices.insert(slice_no, slice);
        self.xmit_slice = slice_no as i32;
        let slice = self.slices.get_mut(&slice_no).unwrap();
        trace!("makeSlice {:?}, {:?}", slice, self.data_fifo);
        return slice;
    }

    fn send_slice(&mut self, rxmit: bool) {
        let mut blocklist = Vec::new();
        if self.xmit_slice >= 0 {
            let xmit_slice = self.xmit_slice as u32;
            let slice = self.slices.get_mut(&xmit_slice).unwrap();
            for block_no in 0..slice.blocks_in_slice {
                if rxmit && slice.retransmit.map.get(block_no as usize) {
                    if block_no > slice.last_good_block {
                        slice.last_good_block = block_no;
                    }
                    continue;
                }
                blocklist.push(block_no);
            }
            slice.need_rxmit = false;
        }
        for block_no in blocklist {
            let _ = self.send_datablock(block_no);
        }
    }

    fn do_retransmissions(&mut self) {
        if self.xmit_slice >= 0 {
            let xmit_slice = self.xmit_slice as u32;
            let slice = self.slices.get_mut(&xmit_slice).unwrap();
            slice.rxmit_id += 1;
        }
        self.retransmits += 1;
        self.send_slice(true);
        let _ = self.send_reqack();
    }

    pub fn read(&mut self) -> bool {
        let mut retrys = 0;
        let mut required: usize = MAX_BUFFER_SIZE - self.data_fifo.len();
        if (required % self.read_chunk) != 0 {
            required -= required % self.read_chunk;
        }
        trace!("read: ");
        while required > 0 {
            if let Ok(mut buff) = self.rx.recv_timeout(Duration::from_millis(100)) {
                trace!("read {} bytes", buff.len());
                if buff.len() > 0 {
                    self.data_fifo.push(&mut buff);
                    required -= buff.len();
                }
            } else {
                if retrys > 100 {
                    return false;
                }
                retrys += 1;
            }
        }
        return true;
    }

    pub fn transfer_data(&mut self) -> bool {
        if self.xmit_slice >= 0 {
            let xmit_slice = self.xmit_slice as u32;
            let slice = self.slices.get_mut(&xmit_slice).unwrap();
            if slice.nr_answered < self.clientlist.len() as u32 {
                if self.lastsendtime.elapsed().as_secs() > 0 {
                    warn!(
                        "Waiting for response from clients {}/{} {}",
                        slice.nr_answered,
                        self.clientlist.len(),
                        " ".repeat(10)
                    );
                    slice.rxmit_id += 1;
                    let _ = self.send_reqack();
                    self.lastsendtime = Instant::now();
                    return RUNNING;
                }
                if slice.rxmit_id > 10 {
                    return self.drop_client() > 0;
                }
                return RUNNING;
            }
            if slice.need_rxmit {
                self.do_retransmissions();
                return RUNNING;
            }
            self.data_fifo.pop(slice.bytes as usize);
            self.xmit_slice = -1;
            if getch(0) == Some('q') {
                let _ = self.send_disconnect(self.socket.multicast_addr);
                return ENDLOOP;
            }
        }

        self.lastsendtime = Instant::now();
        self.read();

        let slice = self.make_slice(self.blocksize as u32, self.slice_size);
        if slice.bytes == 0 {
            let _ = self.send_reqack();
            return ENDLOOP;
        }
        self.send_slice(false);
        self.display_progress(false);
        let _ = self.send_reqack();
        return RUNNING;
    }

    fn handle_ok(&mut self, msg: &MsgOk) -> bool {
        let slice = self.slices.get_mut(&msg.sliceno).unwrap();
        let clientaddr = self.socket.receivefrom.unwrap();
        if let Some(&(client_no, _, _)) = self.clientlist.get(&clientaddr) {
            slice.ready_set.set(client_no, true);
            slice.nr_answered += 1;
        }
        trace!("handle {:?} -> {:?}", msg, slice.ready_set);
        return true;
    }

    fn drop_client(&mut self) -> usize {
        let mut droplist = Vec::new();
        for ref client in &self.clientlist {
            if self.xmit_slice >= 0 {
                let xmit_slice = self.xmit_slice as u32;
                let slice = self.slices.get_mut(&xmit_slice).unwrap();
                if slice.ready_set.get(client.1 .0) == false {
                    droplist.push(*client.0);
                }
            }
        }
        for client in droplist {
            warn!("drop client #{}", client);
            self.remove_client(client);
        }
        return self.clientlist.len();
    }

    fn remove_client(&mut self, client: SocketAddrV4) -> bool {
        if let Some(&(client_no, _, _)) = self.clientlist.get(&client) {
            self.clientlist.remove(&client);
            let _ = self.send_disconnect(client);
            if self.xmit_slice >= 0 {
                let xmit_slice = self.xmit_slice as u32;
                let slice = self.slices.get_mut(&xmit_slice).unwrap();
                slice.ready_set.set(client_no, false);
                slice.nr_answered -= 1;
            }
        }
        return true;
    }

    fn handle_disconnect(&mut self, _msg: &MsgDisconnect) -> bool {
        let clientaddr = self.socket.receivefrom.unwrap();
        self.remove_client(clientaddr);
        if self.clientlist.len() > 0 {
            return true;
        }
        return false;
    }

    fn handle_retransmit(&mut self, msg: &MsgRetransmit, map: Vec<u8>) -> bool {
        let slice = self.slices.get_mut(&msg.sliceno).unwrap();
        warn!("handle {:?}: {} / {}", msg, msg.rxmit, slice.rxmit_id);
        slice.nr_answered += 1;
        if msg.rxmit < slice.rxmit_id {
            return true;
        }
        let map = BitArray::from(map);
        slice.retransmit.map.bits_or(map);
        slice.need_rxmit = true;
        return true;
    }

    pub fn dispatch_message(&mut self) -> Result<bool, &'static str> {
        let mut buff: [u8; 2048] = [0; 2048];
        match self.socket.recv_msg(&mut buff) {
            Ok((msg, remain)) => match msg {
                Message::CmdOk(m) => return Ok(self.handle_ok(&m)),
                Message::CmdDisconnect(m) => return Ok(self.handle_disconnect(&m)),
                Message::CmdRetransmit(m) => return Ok(self.handle_retransmit(&m, remain)),
                _ => Err("Received an unexpected message."),
            },
            Err(ref err) if err.kind() == std::io::ErrorKind::TimedOut => {
                return Ok(true);
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                self.read();
                return Ok(true);
            }
            Err(_err) => return Err("Unexpected error!!"),
        }
    }
}
