/*
    Problem 9

    A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

        a^2 + b^2 = c^2

    For example, 3^2 + 4^2 = 9 + 16 = 2^5 = 52.

    There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    Find the product abc.
*/

extern crate mathlib;

fn main() {
    let mut the_triplet = 0;
    'a: for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if a + b + c == 1000 {
                    if mathlib::is_pythagorean_triplet(a, b, c) {
                        the_triplet = a * b * c;
                        break 'a;
                    }
                }
            }
        }
    }
            
    println!("{}", the_triplet);    
}

