use std::ops::{BitAndAssign, BitOrAssign};

#[derive(Debug)]
pub struct BitArray {
    bits: Vec<u8>,
    size: usize,
}

impl BitArray {
    pub fn new(size: usize) -> Self {
        let byte_len = (size + 7) / 8;
        let bits = vec![0; byte_len];

        Self { bits, size }
    }

    pub fn resize(&mut self, size: usize) -> &mut Self {
        let new_len = (size + 7) / 8;
        self.bits.resize(new_len, 0);
        self.size = size;
        self
    }

    pub fn set(&mut self, index: usize, value: bool) -> &mut Self {
        assert!(index < self.size, "Index out of range");

        let byte_index = index / 8;
        let bit_index = index % 8;

        if value {
            self.bits[byte_index] |= 1 << bit_index;
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
        self
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.size, "Index out of range");

        let byte_index = index / 8;
        let bit_index = index % 8;

        (self.bits[byte_index as usize] & (1 << bit_index)) != 0
    }

    pub fn bits(&mut self) -> Vec<u8> {
        self.bits.clone()
    }
}

impl BitAndAssign for BitArray {
    fn bitand_assign(&mut self, rhs: Self) {
        assert_eq!(self.bits.len(), rhs.bits.len());
        self.bits = self
            .bits
            .iter()
            .zip(rhs.bits.iter())
            .map(|(x, y)| *x & *y)
            .collect();
    }
}

impl BitOrAssign for BitArray {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(self.bits.len(), rhs.bits.len());
        self.bits = self
            .bits
            .iter()
            .zip(rhs.bits.iter())
            .map(|(x, y)| *x | *y)
            .collect();
    }
}

impl From<Vec<u8>> for BitArray {
    fn from(data: Vec<u8>) -> Self {
        let size = data.len();
        let bits = data;
        Self { bits, size }
    }
}
