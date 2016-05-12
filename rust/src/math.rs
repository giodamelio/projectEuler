// Get all the factors of a number
pub fn factors(num: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = Vec::new();
    let max = ((num as f64).sqrt() as i64) + 1;
    for i in 2..max {
        if num % i == 0 {
            factors.push(i);
            factors.push(num / i);
        }
    }
    factors.sort();
    factors
}

// Get the prime factors
pub fn prime_factors(num: i64) -> Vec<i64> {
    let mut f = factors(num);
    f.retain(|&x|is_prime(x));
    f
}

// Test if a number is prime or not
pub fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 || n < 3 {
        return false;
    }
    let max = ((n as f64).sqrt() as i64) + 1;
    for i in 3i64..max {
        if n % i == 0 {
            return false; 
        }
    }
    true
}

#[test]
fn test_is_prime() {
    let primes = [2, 3, 5, 7, 11, 13, 6977];
    for n in primes.iter() {
        assert!(is_prime(*n));
    }

    let not_primes = [4, 6, 8, 10, 12, 14, 15, 1005];
    for n in not_primes.iter() {
        assert!(!is_prime(*n));
    }
}

// Test is a number is a palindrome
pub fn is_palindrome(num: i64) -> bool {
    let string = num.to_string();
    let bytes = string.as_bytes();
    let iter = bytes.iter();
    let half = bytes.len() / 2;
    let first_half = iter.clone().take(half);
    let second_half = iter.clone().rev().take(half);

    // Zip them togather and make sure they match
    first_half
        .zip(second_half)
        .all(|(a, b)| a == b)
}

// Find the greatest commen divisor
pub fn greatest_commen_divisor(a: i64, b: i64) -> i64 {
    if a == 0 {
        b.abs()
    } else {
        greatest_commen_divisor(b % a, a)
    }
}

// Find the least commen multiple of two numbers
pub fn least_commen_multiple(a: i64, b: i64) -> i64 {
    (a * b) / greatest_commen_divisor(a, b)
}

// Test if three numbers form a pythagorean triplet
pub fn is_pythagorean_triplet(a: i64, b: i64, c: i64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

