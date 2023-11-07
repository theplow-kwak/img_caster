use crate::bitarray::BitArray;
use crate::packet::*;
use crate::*;

use core::fmt;
use std::collections::HashMap;
use std::time::Instant;

pub struct Slice {
    pub slice_no: u32,
    pub bytes: u32,
    block_size: u32,
    base: usize,
    pub blocks_in_slice: u32,
    pub blocks_transferred: u32,
    pub retransmit: Retransmit,
    pub reqack: MsgReqAck,
    pub ready_set: BitArray,
    pub rxmit_id: u32,
    pub need_rxmit: bool,
    pub nr_answered: u32,
    pub last_good_block: u32,
    pub start_time: Instant,
    pub end_time: Instant,
    events: HashMap<String, Instant>,
}

impl Slice {
    pub fn new(slice_no: u32, bytes: u32, block_size: u32, base: usize, max_slice: u32) -> Self {
        Self {
            slice_no,
            bytes,
            block_size,
            base,
            blocks_in_slice: ((bytes + block_size - 1) / block_size),
            blocks_transferred: 0,
            retransmit: Retransmit::new(slice_no, 0, max_slice),
            reqack: MsgReqAck::new(slice_no, bytes, 0),
            ready_set: BitArray::new(MAX_CLIENTS as usize),
            rxmit_id: 0,
            need_rxmit: false,
            nr_answered: 0,
            last_good_block: 0,
            start_time: Instant::now(),
            end_time: Instant::now(),
            events: HashMap::new(),
        }
    }

    pub fn update_block(&mut self, block_no: u32) -> bool {
        if self.retransmit.map.get(block_no as usize) {
            return false;
        }
        self.retransmit.map.set(block_no as usize, true);
        self.blocks_transferred += 1;
        return true;
    }

    pub fn get_block_pos(&mut self, block_no: u32) -> usize {
        self.base + (self.block_size * block_no) as usize
    }

    pub fn is_completed(&self) -> bool {
        self.blocks_in_slice == self.blocks_transferred
    }

    pub fn responce(&mut self, client_no: usize) {
        self.ready_set.set(client_no, true);
        self.nr_answered += 1;
    }

    pub fn remove_client(&mut self, client_no: usize) {
        self.ready_set.set(client_no, false);
        self.nr_answered -= 1;
    }

    pub fn event(&mut self, id: String) {
        if self.events.contains_key(&id) {
            self.events.insert(id + "1", Instant::now());
        } else {
            self.events.insert(id, Instant::now());
        }
    }

    pub fn events(&mut self) -> std::collections::hash_map::Iter<'_, String, Instant> {
        self.events.iter()
    }
}

impl fmt::Debug for Slice {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "sliceNo: {}, blockSize: {}, blocksInSlice: {}, base: {}, bytes: {}",
            self.slice_no, self.block_size, self.blocks_in_slice, self.base, self.bytes
        )
    }
}
