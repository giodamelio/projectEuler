use math;

pub fn solve() {
    let factors = math::prime_factors(600851475143);
    assert_answer!(factors[factors.len() - 1]);
}
