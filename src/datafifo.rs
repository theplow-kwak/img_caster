use core::fmt;
use log::{debug, error, info, trace, warn};

pub struct DataFIFO {
    buffer: Box<Vec<u8>>,
    pub slicebase: usize,
    pub startpoint: usize,
    pub endpoint: usize,
}

impl DataFIFO {
    pub fn new() -> Self {
        Self {
            buffer: Box::new(Vec::new()),
            slicebase: 0,
            startpoint: 0,
            endpoint: 0,
        }
    }

    pub fn push(&mut self, data: &mut [u8]) -> &mut Self {
        let size = data.len();
        self.buffer.extend_from_slice(&data);
        self.endpoint += size;
        self
    }

    pub fn pop(&mut self, size: usize) -> Option<Vec<u8>> {
        let remain: Vec<_> = self.buffer.drain(..size).collect();
        self.startpoint += size;
        Some(remain)
    }

    pub fn get(&mut self, pos: usize, size: u32) -> Vec<u8> {
        let start = pos - self.startpoint;
        let len = self.endpoint - self.startpoint;
        let mut end = start + size as usize;
        if end > len {
            end = len;
        }
        let data = self.buffer[start as usize..end as usize].to_vec();
        data
    }

    pub fn set(&mut self, pos: usize, data: &[u8]) -> &mut Self {
        let size = data.len();
        let start = pos - self.startpoint;
        let end = start + size;
        self.buffer
            .splice(start as usize..end as usize, data.to_vec());
        self
    }

    pub fn reserve(&mut self, size: u32) -> usize {
        let base = self.slicebase;
        let mut zvec = vec![0u8; size as usize];
        self.buffer.append(&mut zvec);
        self.slicebase += size as usize;
        self.endpoint = base;
        trace!("reserve {size}: start {} - end {} - slice {}", self.startpoint, self.endpoint, self.slicebase);
        base
    }

    pub fn assign(&mut self, size: u32) -> usize {
        let base = self.slicebase;
        self.slicebase += size as usize;
        if self.slicebase > self.endpoint {
            self.slicebase = self.endpoint;
        }
        base
    }

    pub fn written_bytes(&self) -> usize {
        self.startpoint
    }

    pub fn is_empty(&mut self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.buffer.clear();
        self
    }

    pub fn len(&mut self) -> usize {
        self.endpoint - self.startpoint
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
