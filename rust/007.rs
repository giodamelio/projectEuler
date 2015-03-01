/*
    Problem 6

    By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

    What is the 10,001st prime number?
*/

#![feature(core)]

use std::iter;

extern crate mathlib;

fn main() {
    let ten_thousandth_and_first_prime =
        // Get all the natural numbers
        iter::count(1i64, 1).
        // Keep only the prime ones
        filter(|&n| mathlib::is_prime(n)).
        // Get the 10,001st prime
        skip(10000).next();

    println!("{}", ten_thousandth_and_first_prime.expect("HAHA"));
}

