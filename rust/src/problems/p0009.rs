use math;

pub fn solve() {
    let mut the_triplet = 0;
    'a: for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if a + b + c == 1000 && math::is_pythagorean_triplet(a, b, c) {
                    the_triplet = a * b * c;
                    break 'a;
                }
            }
        }
    }
            
    assert_answer!(the_triplet);
}

