

struct BitArray {
    bits: Vec<bool>,
    size: usize,
}

impl BitArray {
    fn new(size: usize) -> Self {
        let byte_len = (size + 7) / 8;  // 비트 개수를 바이트 개수로 변환
        let bits = vec![false; byte_len * 8];  // 비트 배열 초기화

        Self { bits, size }
    }

    fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.size, "Index out of range");

        self.bits[index] = value;
    }

    fn get(&self, index: usize) -> bool {
        assert!(index < self.size, "Index out of range");

        self.bits[index]
    }
}
