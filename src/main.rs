#[derive(Debug)]
struct RingBuffer {
    buffer: Vec<u8>,
    size: usize,
    read_pos: usize,
    write_pos: usize,
}

impl RingBuffer {
    fn new(size: usize) -> Self {
        RingBuffer {
            buffer: vec![0; size],
            size,
            read_pos: 0,
            write_pos: 0,
        }
    }

    fn write(&mut self, data: &[u8]) -> usize {
        let available_space = self.size - self.used_space();
        let write_len = std::cmp::min(available_space, data.len());

        let write_end = self.write_pos + write_len;
        if write_end <= self.size {
            self.buffer[self.write_pos..write_end].copy_from_slice(&data[..write_len]);
        } else {
            let split = self.size - self.write_pos;
            self.buffer[self.write_pos..].copy_from_slice(&data[..split]);
            self.buffer[..write_len - split].copy_from_slice(&data[split..]);
        }

        self.write_pos = (self.write_pos + write_len) % self.size;
        write_len
    }

    fn read(&mut self, dest: &mut [u8]) -> usize {
        let available_data = self.used_space();
        let read_len = std::cmp::min(available_data, dest.len());

        let read_end = self.read_pos + read_len;
        if read_end <= self.size {
            dest[..read_len].copy_from_slice(&self.buffer[self.read_pos..read_end]);
        } else {
            let split = self.size - self.read_pos;
            dest[..split].copy_from_slice(&self.buffer[self.read_pos..]);
            dest[split..read_len].copy_from_slice(&self.buffer[..read_len - split]);
        }

        self.read_pos = (self.read_pos + read_len) % self.size;
        read_len
    }

    fn used_space(&self) -> usize {
        (self.write_pos + self.size - self.read_pos) % self.size
    }
}

fn main() {
    let mut buf = RingBuffer::new(15);

    buf.write(&vec![1,2,3,4,5,6,7,8]);
    println!("{:?}", buf);
    buf.write(&vec![9,10,11,12,13,14,15,16]);
    println!("{:?}", buf);
    buf.write(&vec![17,18,19,20,21,22,23,24,25]);
    println!("{:?}", buf);
    buf.write(&vec![26,27,28,29,30,31,32,33,34,35,36,37]);
    println!("{:?}", buf);
}