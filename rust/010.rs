/*
    Problem 10

    The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

    Find the sum of all the primes below two million.
*/

#![feature(core)]

use std::iter;
use std::iter::AdditiveIterator;

extern crate mathlib;

fn main() {
    let sum_of_primes =
        // All the natural numbers
        iter::count(0i64, 1).
        // Keep just the primes
        filter(|&n| mathlib::is_prime(n)).
        // Keep only the ones below 2 million
        take_while(|&n| n < 2000000).
        // Add them up
        sum();

    println!("{}", sum_of_primes);
}

