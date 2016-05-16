use math;

pub fn solve() {
    for n in math::triangle_numbers() {
        let factors = math::factors(n);

        if factors.len() > (500 - 2) {
            assert_answer!(n);
            break;
        }
    }
}
