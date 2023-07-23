use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.size, "Index out of range");

        let byte_index = index / 8;
        let bit_index = index % 8;

        if value {
            self.bits[byte_index] |= 1 << bit_index;
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn get(&mut self, index: usize) -> Option<bool> {
        if index >= self.size {
            return None;
        }

        let byte_index = index / 8;
        let bit_index = index % 8;

        Some((self.bits[byte_index] & (1 << bit_index)) != 0)
    }

    pub fn bit_or(&mut self, other: BitArray) -> &mut Self {
        self.bits = self.bits.iter().zip(other.bits).map(|(x, y)| x | y).collect();
        self
    }
}
