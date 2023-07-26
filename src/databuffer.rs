pub struct DataBuffer {
    start: u64,
    end: u64,
    index: u64,
    capacity: usize,
    buffer: Box<Vec<u8>>,
}

impl DataBuffer {
    pub fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            index: 0,
            capacity: 0,
            buffer: Box::new(Vec::new()),
        }
    }

    fn push(&mut self, data: &mut [u8]) {
        self.buffer.append(&mut data.to_vec());
    }

    fn pop(&mut self, size: u64) -> Vec<u8> {
        let mut remain: Vec<_> = self.buffer.drain(..size as usize).collect();    // pop
        remain
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
