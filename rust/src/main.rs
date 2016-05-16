#![feature(iter_arith)]
#![feature(inclusive_range_syntax)]
#![feature(plugin)]

#![plugin(clippy)]

use std::env;
use std::process;

#[macro_use]
mod macros;
mod math;
mod problems;

fn main() {
    let mut args = env::args().skip(1);

    // Get input number
    let raw_number = match args.next() {
        Some(arg) => arg,
        None => {
            println!("Usage 'cargo run <number>'");
            process::exit(0);
        }
    };

    // Parse the number
    let number = match raw_number.parse::<usize>() {
        Ok(number) => number,
        Err(_) => {
            println!("Usage 'cargo run <number>'");
            process::exit(0);
        }
    };

    // Big ugly match statement...
    match number {
        1  => problems::p0001::solve(),
        2  => problems::p0002::solve(),
        3  => problems::p0003::solve(),
        4  => problems::p0004::solve(),
        5  => problems::p0005::solve(),
        6  => problems::p0006::solve(),
        7  => problems::p0007::solve(),
        8  => problems::p0008::solve(),
        9  => problems::p0009::solve(),
        10 => problems::p0010::solve(),
        11 => problems::p0011::solve(),
        12 => problems::p0012::solve(),
        _ => {
            println!("Problem {} is not solved yet", number);
            process::exit(0);
        },
    }
}
