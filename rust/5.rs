/*
    Problem 5

    The sum of the squares of the first ten natural numbers is,

        1^2 + 2^2 + ... + 10^2 = 385

    The square of the sum of the first ten natural numbers is,

        (1 + 2 + ... + 10)^2 = 55^2 = 3025

    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

#![feature(core)]

use std::iter;
use std::iter::AdditiveIterator;
use std::num::Int;

fn main() {
    let sum_of_squares =
        // Get the numbers from 1 to 100
        iter::range_inclusive(1, 100).
        // Square the number
        map(|n| n.pow(2)).
        // Sum them
        sum();

    let square_of_sum =
        // Get the numbers from 1 to 100
        iter::range_inclusive(1, 100).
        // Sum them
        sum().
        // Square them
        pow(2);

    println!("{}", square_of_sum - sum_of_squares);
}

