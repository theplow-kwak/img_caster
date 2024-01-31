use log::trace;
use std::fmt;

use crate::*;

pub struct DataFIFO {
    buffer: Box<Vec<u8>>,
    capacity: usize,
    slicebase: usize,
    startpoint: usize,
    endpoint: usize,
    close: bool,
}

impl DataFIFO {
    pub fn new(capacity: usize) -> Self {
        let mut capacity = capacity;
        if capacity < MAX_BUFFER_SIZE {
            capacity = MAX_BUFFER_SIZE;
        }
        Self {
            buffer: Box::new(vec![0; capacity]),
            capacity,
            slicebase: 0,
            startpoint: 0,
            endpoint: 0,
            close: false,
        }
    }

    pub fn push(&mut self, data: &mut [u8]) -> &mut Self {
        let size = data.len();
        let start = self.endpoint % self.capacity;
        let end = start + size;
        let split = self.capacity - start;
        self.endpoint += size;
        if end <= self.capacity {
            self.buffer[start..end].copy_from_slice(&data[..size]);
        } else {
            self.buffer[start..].copy_from_slice(&data[..split]);
            self.buffer[..size - split].copy_from_slice(&data[split..]);
        }
        self
    }

    pub fn pop(&mut self, size: usize) -> Option<Vec<u8>> {
        let start = self.startpoint % self.capacity;
        let end = start + size;
        let split = self.capacity - start;
        self.startpoint += size;
        if end <= self.capacity {
            Some(self.buffer[start..end].to_vec())
        } else {
            let mut data = vec![0; size];
            data[..split].copy_from_slice(&self.buffer[start..]);
            data[split..].copy_from_slice(&self.buffer[..size - split]);
            Some(data)
        }
    }

    pub fn set(&mut self, pos: usize, data: &[u8]) -> &mut Self {
        let size = data.len();
        let start = pos % self.capacity;
        let end = start + size;
        let split = self.capacity - start;
        if end <= self.capacity {
            self.buffer[start..end].copy_from_slice(&data[..size]);
        } else {
            self.buffer[start..].copy_from_slice(&data[..split]);
            self.buffer[..size - split].copy_from_slice(&data[split..]);
        }
        self
    }

    pub fn get(&mut self, pos: usize, size: u32) -> Vec<u8> {
        let mut size = size as usize;
        if pos + size > self.endpoint {
            size = self.endpoint - pos;
        }
        let start = pos % self.capacity;
        let end = start + size;
        let split = self.capacity - start;
        if end <= self.capacity {
            self.buffer[start..end].to_vec()
        } else {
            let mut data = vec![0; size];
            data[..split].copy_from_slice(&self.buffer[start..]);
            data[split..].copy_from_slice(&self.buffer[..size - split]);
            data
        }
    }

    // reserve buffer for received data from server
    pub fn reserve(&mut self, size: u32) -> usize {
        let base = self.slicebase;
        self.slicebase += size as usize;
        self.endpoint = base;
        trace!(
            "reserve {size}: start {} - end {} - slice {}",
            self.startpoint,
            self.endpoint,
            self.slicebase
        );
        base
    }

    // Allocate as slice to transfer data read from file
    pub fn assign(&mut self, size: u32) -> usize {
        let base = self.slicebase;
        self.slicebase += size as usize;
        if self.slicebase > self.endpoint {
            self.slicebase = self.endpoint;
        }
        base
    }

    pub fn drain(&mut self, size: usize) -> &mut Self {
        self.startpoint += size;
        self
    }

    pub fn written_bytes(&self) -> usize {
        self.startpoint
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.buffer.clear();
        self
    }

    pub fn len(&self) -> usize {
        self.endpoint - self.startpoint
    }

    pub fn remain(&self) -> usize {
        self.endpoint - self.slicebase
    }

    pub fn slicebase(&self) -> usize {
        self.slicebase
    }

    pub fn endpoint(&self) -> usize {
        self.endpoint
    }

    pub fn close(&mut self) -> &mut Self {
        self.close = true;
        self
    }

    pub fn is_closed(&self) -> bool {
        self.close
    }
}

impl fmt::Debug for DataFIFO {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

impl fmt::Display for DataFIFO {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}:{} - {}, len: {}",
            self.startpoint,
            self.endpoint,
            self.slicebase,
            self.buffer.len()
        )
    }
}
