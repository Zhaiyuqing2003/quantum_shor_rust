use std::{collections::{HashMap, hash_map::Iter}};
use num::{complex::Complex32, Zero};
use rand::{thread_rng, Rng};

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
        if value.is_zero() {
            self.data.remove(&key);
        } else {
            self.data.insert(key, value);
        }   
    }

    pub fn increment_state(&mut self, key : usize, value : Complex32) {
        self.set_state(key, self.get_state(key) + value);
    }

    pub fn get_bit_length(&self) -> usize {
        self.bit_length
    }

    pub fn iter(&self) -> Iter<'_, usize, Complex32> {
        self.data.iter()
    }

    pub fn measure(&self) -> usize {
        // generate random number between 0 and 1
        
        let probability_list : Vec<(usize, f64)> = self.data.iter()
            .map(|(key, value)| (*key, (value.norm() * value.norm()) as f64))
            .collect();
        
        let sum : f64 = probability_list.iter().fold(0.0, |acc, (_, value)| acc + value);
        
        if (sum - 1.0).abs() > 1e-10 {
            panic!("Probability list is not normalized");
        }
        
        let random_number : f64 = thread_rng().gen_range(0.0..1.0);
        let mut current_value: f64 = 0.0;

        for (key, value) in probability_list.iter() {
            current_value += value;

            if current_value > random_number {
                return *key;
            }
        }

        panic!("Something went wrong");
    }
}