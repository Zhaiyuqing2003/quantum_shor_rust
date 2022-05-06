use std::collections::HashSet;

use is_prime::is_prime;
use num::complex::Complex32;
use rand::{Rng, thread_rng};

use crate::{phase_estimation::get_phase_estimation_wire, gates::unitary_modular::UnitaryModular, quantum_state::QuantumState};

pub fn is_integer(num : f64) -> bool {
    (num - num.round()).abs() < 1e-10
}

pub fn gcd(a : f64, b : f64) -> f64 {
    if a == 0.0 {
        b
    } else {
        gcd(b % a, a)
    }
}

pub fn get_lowest_fraction(number : f64, largest_denominator: i32) -> (i32, i32) {
    let eps = 1.0e-15;

    let mut x = number;
    let mut a = x.floor();
    let mut h1 = 1.0;
    let mut k1 = 0.0;
    let mut h = a;
    let mut k = 1.0;

    let mut h2;
    let mut k2;

    while x-a > eps*k*k {
        x = 1.0 / (x-a);
        a = x.floor();
        h2 = h1; h1 = h;
        k2 = k1; k1 = k;
        let temp_h = h;
        let temp_k = k;
        h = h2 + a*h1;
        k = k2 + a*k1;
        if k > largest_denominator as f64 {
            h = temp_h;
            k = temp_k;
            break;
        }
    }

    return (h as i32, k as i32);
}

pub fn classical_shor(number : f64) -> Vec<f64> {
    if number == 1.0 {
        return Vec::new();
    }

    // check if number is prime
    if is_prime(&number.to_string()) {
        return vec![number];   
    // then exclude some frequent situation
    } else if (number % 2.0) == 0.0 {
        let mut vec = vec![2.0];
        vec.append(&mut classical_shor(number / 2.0));

        return vec;
    } else {
        let number_float = number as f64;

        let k_limit = (number_float).log(3.0) as u128;

        for k in 2..k_limit {
            // check if number**(1/k) is integer
            let exponent = number_float.powf(1.0 / (k as f64));

            if is_integer(exponent) {
                let result_vec = classical_shor(exponent);

                return result_vec.repeat(k as usize);
            }
        }

        loop {
            let random_number = thread_rng().gen_range(2..(number_float.sqrt() as u128 + 1)) as f64;
            let divisor = gcd(number, random_number);

            if divisor != 1.0 {
                let mut vec = classical_shor(divisor as f64);
                vec.append(&mut classical_shor(number / divisor));

                return vec;
            } else {
                // we find a co-prime
                for r in 2..(number as u128) {
                    if (random_number.powf(r as f64) % number) == 1.0 {
                        // if r is odd
                        if r % 2 == 1 {
                            break
                        }

                        println!("x = {}, r = {}", random_number, r);


                        let first = (random_number.powf((r / 2) as f64) - 1.0) % number;
                        let second = (random_number.powf((r / 2) as f64) + 1.0) % number;

                        let factor_one = gcd(number, first);
                        let factor_two = gcd(number, second);

                        let factor_smaller = f64::max(factor_one, factor_two);
                        let factor_bigger = f64::max(factor_one, factor_two);

                        if factor_smaller == 1.0 && factor_bigger == number {
                            break;
                        }

                        let mut vec = classical_shor(factor_one);
                        vec.append(&mut classical_shor(factor_two));

                        return vec;
                    }
                }
            }
        }
    }
}

pub fn quantum_shor(number : f64) -> Vec<f64> {
    if number == 1.0 {
        return Vec::new();
    }

    // check if number is prime
    if is_prime(&number.to_string()) {
        return vec![number];   
    // then exclude some frequent situation
    } else if (number % 2.0) == 0.0 {
        let mut vec = vec![2.0];
        vec.append(&mut quantum_shor(number / 2.0));

        return vec;
    } else {
        let number_float = number as f64;

        let k_limit = (number_float).log(3.0) as u128;

        for k in 2..k_limit {
            // check if number**(1/k) is integer
            let exponent = number_float.powf(1.0 / (k as f64));

            if is_integer(exponent) {
                let result_vec = quantum_shor(exponent);

                return result_vec.repeat(k as usize);
            }
        }

        loop {
            let random_number = thread_rng().gen_range(2..(number_float.sqrt() as u128 + 1)) as f64;

            println!("x = {}", random_number);

            let divisor = gcd(number, random_number);

            if divisor != 1.0 {
                let mut vec = quantum_shor(divisor as f64);
                vec.append(&mut quantum_shor(number / divisor));

                return vec;
            } else {
                // we find a co-prime
                let n = number.log2().ceil() as usize;

                let mut phase_set : HashSet<String> = HashSet::new();

                let wire = get_phase_estimation_wire(
                    2 * n + 1, 
                    UnitaryModular::new(
                        n, 0, n, random_number as usize, number as usize
                    )
                );

                let mut state = QuantumState::from_length(n + 2 * n + 1);
                state.increment_state(1, Complex32::new(1.0, 0.0));

                // apply all the gates
                for gate in wire {
                    state = (*(gate.get()))(&state);
                }

                loop {
                    if phase_set.len() >= n {
                        break;
                    }

                    println!("start finding phase");

                    let measured_key = state.measure();

                    let estimated_phase = ((measured_key >> n) as f64) / ((1 << (2 * n + 1)) as f64);

                    println!("estimated phase = {}", measured_key >> n);

                    phase_set.insert(estimated_phase.to_string());

                    if estimated_phase == 0.0 {
                        break;
                    }

                    println!("estimated phase = {}", estimated_phase);

                    let (s, r) = get_lowest_fraction(estimated_phase, number as i32);

                    println!("attempted s/r = {}/{}", s, r);

                    if (random_number.powf(r as f64) % number) == 1.0 {
                        // if r is odd
                        if r % 2 == 1 {
                            break
                        }

                        println!("x = {}, r = {}", random_number, r);


                        let first = (random_number.powf((r / 2) as f64) - 1.0) % number;
                        let second = (random_number.powf((r / 2) as f64) + 1.0) % number;

                        let factor_one = gcd(number, first);
                        let factor_two = gcd(number, second);

                        let factor_smaller = f64::max(factor_one, factor_two);
                        let factor_bigger = f64::max(factor_one, factor_two);

                        if factor_smaller == 1.0 && factor_bigger == number {
                            break;
                        }

                        let mut vec = quantum_shor(factor_one);
                        vec.append(&mut quantum_shor(factor_two));

                        return vec;
                    }
                }
            }
        }
    }
}