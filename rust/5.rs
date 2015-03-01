/*
    Problem 5

    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

extern crate mathlib;

fn main() {
    let mut biggest_divisible_number = 1;
    for n in 1..21 {
        biggest_divisible_number = mathlib::least_commen_multiple(biggest_divisible_number, n);
    }
    println!("{}", biggest_divisible_number);
}

