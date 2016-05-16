use math;

pub fn solve() {
    let sum_of_primes: i64 = (1..)
        // Keep just the primes
        .filter(|&n| math::is_prime(n))

        // Keep only the ones below 2 million
        .take_while(|&n| n < 2000000)

        // Add them up
        .sum();

    assert_answer!(sum_of_primes);
}
