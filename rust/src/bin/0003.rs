#[macro_use]
extern crate math;

fn main() {
    let factors = math::prime_factors(600851475143);
    assert_answer!(factors[factors.len() - 1]);
}

