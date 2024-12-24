use byte_unit::Byte;
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::SocketAddrV4;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::bitarray::BitArray;
use crate::datafifo::DataFIFO;
use crate::dev::disk::Disk;
use crate::multicast::*;
use crate::packet::*;
use crate::slice::Slice;
use crate::*;

pub struct McastReceiver {
    pub socket: MultiCast,
    data_fifo: Arc<RwLock<DataFIFO>>,
    rcvbuf: u32,
    client_number: u32,
    block_size: u32,
    max_slices: u32,
    pub transferstarted: bool,
    pub slices: HashMap<u32, Slice>,
    pub start_time: Instant,
    elaps_time: Instant,
    written_elaps: u128,
    max_pipesize: usize,
}

impl McastReceiver {
    pub fn new(nic: usize, rcvbuf: usize, data_fifo: Arc<RwLock<DataFIFO>>) -> Self {
        let socket = MultiCast::receiver(nic, rcvbuf);
        socket.join_multicast().unwrap();

        Self {
            socket,
            data_fifo,
            client_number: 0,
            block_size: 0,
            rcvbuf: rcvbuf as u32,
            max_slices: MAX_SLICE_SIZE,
            transferstarted: false,
            slices: HashMap::new(),
            start_time: Instant::now(),
            elaps_time: Instant::now(),
            written_elaps: 0,
            max_pipesize: MAX_READ_PIPE,
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
            "IP: {} Connected as #{} to {}",
            self.socket.myip_addr,
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

    pub fn id(&self) -> String {
        self.client_number.to_string()
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
        warn!("Request retransmit {:?}: {}", msg, msg.rxmit);
        let slice = self.get_slice(msg.sliceno, msg.bytes);
        let mut map = slice.retransmit.map.bits();
        let mut buffer =
            Message::CmdRetransmit(MsgRetransmit::new(msg.sliceno, msg.rxmit)).encode();
        buffer.append(&mut map);
        self.socket.send_msg(&buffer)
    }

    fn get_slice(&mut self, slice_no: u32, bytes: u32) -> &mut Slice {
        if !self.slices.contains_key(&slice_no) {
            while self.data_fifo.read().unwrap().len() > self.max_pipesize {
                thread::sleep(Duration::from_micros(100));
                debug!("get_slice_mut: waiting for free buffer");
            }
            let base = self.data_fifo.write().unwrap().reserve(bytes);
            self.slices.insert(
                slice_no,
                Slice::new(slice_no, bytes, self.block_size, base, self.max_slices),
            );
        }
        let slice = self.slices.get_mut(&slice_no).unwrap();
        return slice;
    }

    fn process_datablock(&mut self, msg: &DataBlock, data: Vec<u8>) -> bool {
        let slice = self.get_slice(msg.sliceno, msg.bytes);
        if slice.update_block(msg.blockno as u32) {
            let pos = slice.get_block_pos(msg.blockno as u32);
            self.data_fifo.write().unwrap().set(pos, &data);
        }
        RUNNING
    }

    pub fn display_progress(&mut self, final_disp: bool) {
        let elapsed = self.elaps_time.elapsed();
        let writtenbytes = self.data_fifo.read().unwrap().written_bytes() as u128;
        if elapsed.as_secs() > 0 || final_disp {
            let difftime = self.start_time.elapsed();
            if difftime.as_millis() == 0 {
                return;
            }
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
            let _ = io::stdout().flush();
            self.elaps_time = Instant::now();
            self.socket.packet_count = 0;
        }
        if final_disp {
            println!("\n");
            info!(
                "{} written in {:?}",
                Byte::from_bytes(writtenbytes)
                    .get_appropriate_unit(false)
                    .to_string(),
                self.start_time.elapsed()
            );
        }
    }

    fn process_reqack(&mut self, msg: &MsgReqAck, ready_set: Vec<u8>) -> bool {
        let ready_set = BitArray::from(ready_set);
        if ready_set.get(self.client_number as usize) {
            return RUNNING;
        }
        let slice = self.get_slice(msg.sliceno, msg.bytes);
        if msg.rxmit == 0 && msg.bytes == 0 {
            self.data_fifo.write().unwrap().close();
            let _ = self.send_ok(msg.sliceno);
            return ENDLOOP;
        }
        if slice.is_completed() {
            slice.end_time = Instant::now();
            let _ = self.send_ok(msg.sliceno);
            self.get_slice(msg.sliceno, msg.bytes)
                .event("ok".to_string());
        } else {
            let _ = self.send_retransmit(msg);
            self.get_slice(msg.sliceno, msg.bytes)
                .event("retransmit".to_string());
        }
        self.display_progress(false);
        if getch(0) == Some('q') {
            self.data_fifo.write().unwrap().close();
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
            Err(ref err) if err.kind() == io::ErrorKind::TimedOut => {
                return Ok(RUNNING);
            }
            Err(_err) => return Err("Unexpected error!!"),
        }
    }

    pub fn set_pipesize(&mut self, pipesize: usize) {
        self.max_pipesize = pipesize;
    }

    pub fn get_events(&mut self) -> Vec<(String, Instant, Instant)> {
        let mut events: Vec<(String, Instant, Instant)> = Vec::new();
        for (_, slice) in self.slices.iter_mut() {
            let start_time = slice.start_time;
            events.push(("slice".to_owned(), start_time, slice.end_time));
            for (event_id, event_time) in slice.events() {
                events.push((event_id.to_string(), start_time, *event_time));
            }
        }
        events
    }
}

pub fn write(
    disk: &mut Option<Disk>,
    data_fifo: Arc<RwLock<DataFIFO>>,
    write_chunk: usize,
    disk_trace: Arc<RwLock<Box<Vec<(Instant, Instant)>>>>,
) {
    loop {
        {
            let delay = Instant::now() + Duration::from_millis(50);
            let mut size = data_fifo.read().unwrap().len();
            if !data_fifo.read().unwrap().is_closed() && ((size % write_chunk) != 0) {
                size -= size % write_chunk;
            }
            if size > 20 * 1024 * 1024 {
                size = 20 * 1024 * 1024;
            }
            if size > 0 {
                let start = Instant::now();
                debug!(" -> start write {}", data_fifo.read().unwrap().len());
                let data = data_fifo.write().unwrap().pop(size);
                if let Some(data) = data {
                    if let Some(ref mut disk) = disk {
                        let mut iter = data.chunks(write_chunk);
                        while let Some(data) = iter.next() {
                            if disk.fua.is_some() {
                                if let Err(e) = disk.scsi_write(&data) {
                                    error!("Disk write Error: {:?}", e);
                                    data_fifo.write().unwrap().close();
                                    break;
                                }
                            } else {
                                if let Err(e) = disk.write(&data) {
                                    error!("Disk write Error: {:?}", e);
                                    data_fifo.write().unwrap().close();
                                    break;
                                }
                            }
                        }
                    }
                }
                let end = Instant::now();
                debug!(" <- end write {:?}", end - start);
                disk_trace.write().unwrap().push((start, end));
            }
            thread::sleep(delay.saturating_duration_since(Instant::now()));
        }
        if data_fifo.read().unwrap().is_closed() && data_fifo.read().unwrap().len() <= 0 {
            break;
        }
    }
}
