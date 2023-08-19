use byte_unit::Byte;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::net::SocketAddrV4;
use std::time::{Duration, Instant, SystemTime};

use crate::datafifo::DataFIFO;
use crate::disk::Disk;
use crate::multicast::*;
use crate::packet::*;
use crate::slice::Slice;
use crate::*;

pub struct McastReceiver {
    pub socket: MultiCast,
    pub disk: Option<Disk>,
    data_fifo: DataFIFO,
    rcvbuf: u32,
    client_number: u32,
    block_size: u32,
    max_slices: u32,
    pub write_chunk: usize,
    pub transferstarted: bool,
    slices: HashMap<u32, Slice>,
    start_time: Instant,
    elaps_time: Instant,
    written_elaps: u128,
}

impl McastReceiver {
    pub fn new(nic: usize, filename: &str, rcvbuf: usize) -> Self {
        let socket = MultiCast::receiver(nic, rcvbuf);
        socket.join_multicast().unwrap();

        let disk = Disk::open(filename.to_string(), 'w');
        if let Some(ref d) = disk {
            info!("{:?}", d);
        }

        Self {
            socket,
            disk,
            client_number: 0,
            block_size: 0,
            rcvbuf: rcvbuf as u32,
            max_slices: MAX_SLICE_SIZE,
            write_chunk: CHUNK_SIZE,
            transferstarted: false,
            slices: HashMap::new(),
            data_fifo: DataFIFO::new(),
            start_time: Instant::now(),
            elaps_time: Instant::now(),
            written_elaps: 0,
        }
    }

    pub fn enumerate(&mut self) -> Result<bool, &'static str> {
        let mut connect_req_sent = false;
        let mut buff: [u8; 2048] = [0; 2048];
        loop {
            if !connect_req_sent {
                let _ = self.send_connect_req();
                connect_req_sent = true;
            }
            if let Ok((msg, _remain)) = self.socket.recv_msg(&mut buff) {
                match msg {
                    Message::CmdConnectReply(m) => {
                        self.client_number = m.clnr;
                        self.block_size = m.blocksize;
                        self.max_slices = m.max_slices;
                        self.socket.multicast_addr =
                            SocketAddrV4::new(m.mcastaddr(), self.socket.myip_addr.port());
                        if self.client_number == 0xffffffff {
                            return Err("Too many clients already connected");
                        }
                        break;
                    }
                    Message::CmdHello(m) => {
                        connect_req_sent = false;
                        self.block_size = m.blocksize as u32;
                        self.socket.multicast_addr =
                            SocketAddrV4::new(m.mcastaddr(), self.socket.myip_addr.port());
                    }
                    _ => {}
                }
            }
        }
        info!(
            "Connected as #{} to {}",
            self.client_number,
            self.socket.receivefrom.unwrap(),
        );
        let _ = self.socket.join_multicast();
        info!(
            "Broadcast IP: {}, Multicast IP: {}",
            self.socket.broadcast_addr, self.socket.multicast_addr
        );

        Ok(true)
    }

    pub fn start_transfer(&mut self) {
        let _ = self.send_go();
        self.start_time = Instant::now();
    }

    pub fn send_connect_req(&mut self) -> io::Result<usize> {
        let msg = Message::CmdConnectReq(MsgConnectReq::new(0x81, self.rcvbuf));
        if let Some(sendto) = self.socket.receivefrom {
            self.socket.send_to(&msg.encode(), sendto)
        } else {
            self.socket
                .send_to(&msg.encode(), self.socket.broadcast_addr)
        }
    }

    pub fn send_ok(&mut self, slice_no: u32) -> io::Result<usize> {
        let msg = Message::CmdOk(MsgOk::new(slice_no));
        self.socket.send_msg(&msg.encode())
    }

    pub fn send_go(&mut self) -> io::Result<usize> {
        let msg = Message::CmdGo(MsgGo::new());
        self.socket.send_msg(&msg.encode())
    }

    pub fn send_disconnect(&mut self) -> io::Result<usize> {
        let msg = Message::CmdDisconnect(MsgDisconnect::new());
        self.socket.send_msg(&msg.encode())
    }

    pub fn send_retransmit(&mut self, msg: &MsgReqAck) -> io::Result<usize> {
        warn!("handle {:?}: {}", msg, msg.rxmit);
        let slice = self.get_slice_mut(msg.sliceno, msg.bytes);
        let mut map = slice.retransmit.map.bits();
        let mut buffer =
            Message::CmdRetransmit(MsgRetransmit::new(msg.sliceno, msg.rxmit)).encode();
        buffer.append(&mut map);
        self.socket.send_msg(&buffer)
    }

    fn get_slice_mut(&mut self, slice_no: u32, bytes: u32) -> &mut Slice {
        if !self.slices.contains_key(&slice_no) {
            self.slices.insert(
                slice_no,
                Slice::new(
                    slice_no,
                    bytes,
                    self.block_size,
                    self.data_fifo.reserve(bytes),
                    self.max_slices,
                ),
            );
        }
        let slice = self.slices.get_mut(&slice_no).unwrap();
        return slice;
    }

    fn get_slice(&mut self, slice_no: u32, bytes: u32) -> &Slice {
        if !self.slices.contains_key(&slice_no) {
            self.slices.insert(
                slice_no,
                Slice::new(
                    slice_no,
                    bytes,
                    self.block_size,
                    self.data_fifo.reserve(bytes),
                    self.max_slices,
                ),
            );
        }
        let slice = self.slices.get(&slice_no).unwrap();
        return slice;
    }

    fn process_datablock(&mut self, msg: &DataBlock, data: Vec<u8>) -> bool {
        let slice = self.get_slice_mut(msg.sliceno, msg.bytes);
        if slice.update_block(msg.blockno as u32) {
            let pos = slice.get_block_pos(msg.blockno as u32);
            self.data_fifo.set(pos, &data);
        }
        RUNNING
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
            let mut embps = 0;
            if elapsed.as_millis() > 0 {
                embps = (writtenbytes - self.written_elaps) / elapsed.as_millis();
            }
            info!(
                "Total: {} ({}.{:0<3} MB/s) {:>6} pps, elaps: ({}.{:0<3} MB/s)",
                Byte::from_bytes(writtenbytes)
                    .get_appropriate_unit(false)
                    .to_string(),
                mbps / 1000,
                mbps % 1000,
                self.socket.packet_count,
                embps / 1000,
                embps % 1000
            );
            self.written_elaps = writtenbytes;
            let _ = std::io::stdout().flush();
            self.elaps_time = Instant::now();
            self.socket.packet_count = 0;
        }
        if final_disp {
            println!("\n");
            info!(
                "{} written in {:?}",
                Byte::from_bytes(self.data_fifo.written_bytes() as u128)
                    .get_appropriate_unit(false)
                    .to_string(),
                self.start_time.elapsed()
            );
        }
    }

    fn write(&mut self, size: u32) -> io::Result<()> {
        let mut required: usize = self.data_fifo.len();
        if size > 0 && ((required % self.write_chunk) != 0) {
            required -= required % self.write_chunk;
        }
        if let Some(data) = self.data_fifo.pop(required) {
            if let Some(ref mut disk) = self.disk {
                let mut iter = data.chunks(self.write_chunk);
                while let Some(data) = iter.next() {
                    let _n = disk.write(&data)?;
                }
            }
        }
        Ok(())
    }

    fn process_reqack(&mut self, msg: &MsgReqAck, ready_set: Vec<u8>) -> bool {
        if (ready_set[self.client_number as usize] & (1 << self.client_number)) != 0 {
            return RUNNING;
        }
        let slice = self.get_slice(msg.sliceno, msg.bytes);
        if msg.rxmit == 0 && msg.bytes == 0 {
            if let Err(err) = self.write(0) {
                error!("Write error {:?}", err);
                return ENDLOOP;
            };
            let _ = self.send_ok(msg.sliceno);
            return ENDLOOP;
        }
        if slice.is_completed() {
            if let Err(err) = self.write(msg.bytes) {
                error!("Write error {:?}", err);
                return ENDLOOP;
            };
            let _ = self.send_ok(msg.sliceno);
            self.display_progress(false);
        } else {
            let _ = self.send_retransmit(msg);
        }
        if getch(0) == Some('q') {
            return ENDLOOP;
        }
        RUNNING
    }

    pub fn dispatch_message(&mut self) -> Result<bool, &'static str> {
        let mut buff: [u8; 2048] = [0; 2048];
        match self.socket.recv_msg(&mut buff) {
            Ok((msg, remain)) => match msg {
                Message::CmdData(m) => {
                    if !self.transferstarted {
                        self.start_time = Instant::now();
                        self.transferstarted = true;
                    }
                    return Ok(self.process_datablock(&m, remain));
                }
                Message::CmdReqack(m) => return Ok(self.process_reqack(&m, remain)),
                Message::CmdHello(_m) => return Ok(RUNNING),
                _ => return Err("Received an unexpected message."),
            },
            Err(ref err) if err.kind() == std::io::ErrorKind::TimedOut => {
                return Ok(RUNNING);
            }
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                if let Err(err) = self.write(CHUNK_SIZE as u32) {
                    error!("Write error {:?}", err);
                    return Ok(ENDLOOP);
                };
                return Ok(RUNNING);
            }
            Err(_err) => return Err("Unexpected error!!"),
        }
    }
}
