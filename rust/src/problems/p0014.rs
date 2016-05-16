use math;

pub fn solve() {
    let (_, longest_collatz_sequence) = (1..1_000_000)
        // Convert numbers into the collatz sequences
        .map(|n| math::collatz_sequence(n).collect::<Vec<i64>>())

        // Convert to length
        .map(|n| n.len())

        // Zip with index
        .zip((1..1_000_000))

        // Find the largest
        .max_by_key(|n| n.0).unwrap();

    assert_answer!(longest_collatz_sequence);
}
