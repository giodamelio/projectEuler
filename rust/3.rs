/*
    Problem 3

    The prime factors of 13195 are 5, 7, 13 and 29.

    What is the largest prime factor of the number 600851475143 ?
*/

extern crate mathlib;

fn main() {
    let factors = mathlib::prime_factors(600851475143);
    println!("{}", factors[factors.len() - 1]);
}

