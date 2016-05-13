/// Check if the answer is correct
#[macro_export]
macro_rules! assert_answer {
    ($left:expr) => (
        use std::io::{BufReader, BufRead};
        use std::fs::File;
        use std::path::Path;
        use std::process::exit;

        // Read answers from a file
        let answers_file = File::open(
            Path::new("../problems/answers.txt")
        ).unwrap();

        // Convert file to array of ints
        let answers: Vec<i64> = BufReader::new(answers_file)
            // Convert to iterator of lines
            .lines().map(|x| x.unwrap())

            // Parse Strings into ints
            .map(|x| x.parse::<i64>().unwrap()).collect();
        
        // Get the number of the problem we are working on
        let problem_number = module_path!()
            // Convert it into a number
            .parse::<usize>().unwrap();

        // Make sure we have the answer
        if answers.len() < problem_number {
            println!("No answer stored. Please add it to the file");
            exit(1);
        }

        // Get the answer from the vector
        let answer = answers[problem_number - 1];

        // If the answer is -1 send error
        if answer == -1 {
            println!("Answer not discovered yet");
            exit(1);
        }

        // Compare input to answer list
        if $left == answer {
            println!(
                "{}{} is the correct answer for problem {}{}",
                "\x1b[0;32m", // Make the text green
                $left,
                problem_number,
                "\x1b[0m" // Stop the color
            );
        } else {
            println!(
                "{}{} is not the correct answer for problem {}{}",
                "\x1b[0;31m", // Make the text red
                $left, 
                problem_number,
                "\x1b[0m" // Stop the color
            );
        }
    )
}

/// Get all the factors of a number
///
/// # Examples
/// ```
/// assert_eq!(math::factors(12), vec!(2, 3, 4, 6));
/// assert_eq!(math::factors(93), vec!(3, 31));
/// ```
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

/// Get the prime factors
///
/// # Examples
/// ```
/// assert_eq!(math::prime_factors(12), vec!(2, 3));
/// assert_eq!(math::prime_factors(187), vec!(11, 17));
/// ```
pub fn prime_factors(num: i64) -> Vec<i64> {
    factors(num)
        .iter().cloned()
        .filter(|&x| is_prime(x))
        .collect()
}

/// Test if a number is prime or not
///
/// # Examples
/// ```
/// assert!(math::is_prime(7));
/// assert!(math::is_prime(827));
/// ```
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

/// Test is a number is a palindrome
///
/// # Examples
/// ```
/// assert!(math::is_palindrome(1001));
/// assert!(math::is_palindrome(10101));
/// ```
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

/// Find the greatest commen divisor
///
/// # Examples
/// ```
/// assert_eq!(math::greatest_commen_divisor(54, 144), 18);
/// assert_eq!(math::greatest_commen_divisor(8, 12), 4);
/// ```
pub fn greatest_commen_divisor(a: i64, b: i64) -> i64 {
    if a == 0 {
        b.abs()
    } else {
        greatest_commen_divisor(b % a, a)
    }
}

/// Find the least commen multiple of two numbers
///
/// # Examples
/// ```
/// assert_eq!(math::least_commen_multiple(10, 4), 20);
/// ```
pub fn least_commen_multiple(a: i64, b: i64) -> i64 {
    (a * b) / greatest_commen_divisor(a, b)
}

/// Test if three numbers form a pythagorean triplet
///
/// # Examples
/// ```
/// // Sample Pythagorean triplets
/// assert!(math::is_pythagorean_triplet(3, 4, 5));
/// assert!(math::is_pythagorean_triplet(16, 63, 65));
/// ```
pub fn is_pythagorean_triplet(a: i64, b: i64, c: i64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

