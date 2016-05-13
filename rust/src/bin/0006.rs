#![feature(iter_arith)]
#![feature(inclusive_range_syntax)]

#[macro_use]
extern crate math;

use std::iter::Iterator;

fn main() {
    let sum_of_squares: i64 = (1...100)
        // Square the number
        .map(|n: i64| n.pow(2))

        // Sum them
        .sum();

    let square_of_sum: i64 = (1...100)
        // Sum them
        .sum::<i64>()

        // Square them
        .pow(2);

    assert_answer!(square_of_sum - sum_of_squares);
}

