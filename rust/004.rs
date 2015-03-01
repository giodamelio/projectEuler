/*
    Problem 4

    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers.
*/

extern crate mathlib;

fn main() {
    let mut biggest = 0;
    for a in 1..1000 {
        for b in 1..1000 {
            let i = a * b;
            if mathlib::is_palindrome(i) && i > biggest {
                biggest = i;
            }
        }
    }
    println!("{}", biggest);
}

