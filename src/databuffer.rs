
pub struct DataBuffer {
    start: u64,
    end: u64,
    index: u64,
    data: Vec<u8>,
}

impl DataBuffer {
    pub fn new() -> Self {
        Self {
            start: 0, end: 0, index: 0, data: vec![0u8; 0],
        }
    }

}