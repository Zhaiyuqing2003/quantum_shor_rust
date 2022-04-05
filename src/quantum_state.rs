use std::{collections::{HashMap, hash_map::Iter}};
use num::{complex::Complex32, Zero};

#[derive(Debug, Clone)]
pub struct QuantumState {
    bit_length : usize,
    data : HashMap<usize, Complex32>
}

impl QuantumState {
    pub fn from_length(bit_length : usize) -> QuantumState {
        QuantumState {
            bit_length,
            data : HashMap::new()
        }
    }

    pub fn get_state(&self, key : usize) -> Complex32 {
        self.data.get(&key).unwrap_or(&Complex32::zero()).clone()
    }

    pub fn set_state(&mut self, key : usize, value : Complex32) {
        self.data.insert(key, value);         
    }

    pub fn increment_state(&mut self, key : usize, value : Complex32) {
        self.data.insert(key, self.get_state(key) + value);
    }

    pub fn get_bit_length(&self) -> usize {
        self.bit_length
    }

    pub fn iter(&self) -> Iter<'_, usize, Complex32> {
        self.data.iter()
    }
}