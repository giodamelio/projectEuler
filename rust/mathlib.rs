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
pub fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 || n < 3 {
        return false;
    }
    for i in range(3i64, ((n as f64).sqrt() as i64) + 1) {
        if n % i == 0 {
            return false; 
        }
    }
    true
}

#[test]
fn test_is_prime() {
    let primes = [2, 3, 5, 7, 11, 13, 6977];
    for n in primes.iter() {
        assert!(is_prime(*n));
    }

    let not_primes = [4, 6, 8, 10, 12, 14, 15, 1005];
    for n in not_primes.iter() {
        assert!(!is_prime(*n));
    }
}

// Test is a number is a palindrome
pub fn is_palindrome(num: i64) -> bool {
    let string = num.to_string();
    let bytes = string.as_bytes();
    let iter = bytes.iter();
    let half = bytes.len() / 2;
    equals(iter.clone().take(half), iter.clone().rev().take(half))
}

