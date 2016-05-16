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
        let module_path: &'static str = module_path!()
        let problem_number: usize = module_path
            // Take the last four chars
            .chars().skip(module_path.len() - 4).take(4).collect::<String>()

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
