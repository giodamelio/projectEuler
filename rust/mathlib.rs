#![feature(core)]

use std::num::Float;
use std::iter::order::equals;

// Get all the factors of a number
pub fn factors(num: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();
    let max = ((num as f64).sqrt() as i64) + 1;
    for i in 2..max {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors.sort();
    factors
}

// Get the prime factors
pub fn prime_factors(num: i64) -> Vec<i64> {
    let mut f = factors(num);
    f.retain(|&x|is_prime(x));
    f
}

// Test if a number is prime or not
pub fn is_prime(num: i64) -> bool {
    let max = (num as f64).sqrt().ceil() as i64;
    for i in 2..max {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

// Test is a number is a palindrome
pub fn is_palindrome(num: i64) -> bool {
    let string = num.to_string();
    let bytes = string.as_bytes();
    let iter = bytes.iter();
    let half = bytes.len() / 2;
    equals(iter.clone().take(half), iter.clone().rev().take(half))
}

