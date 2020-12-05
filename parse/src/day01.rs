use std::fs::File; // Access files
use std::io::{BufRead, BufReader}; // Buffered reader with associated trait

pub fn parse_data_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).expect("Not a path!"); // Read in the file from the path, throw an error if it fails.
    let file_reader = BufReader::new(file); // Create a new buffer for the file.

    let result = file_reader.lines() // Produces an iterator over the lines 
                            .filter_map(Result::ok) // Unwrap the Result objects
                            .map(|line| line.parse::<i32>()) // Parse the lines into integers
                            .filter_map(Result::ok) // Unwrap the parsed lines
                            .collect(); // Collect the iterator

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
       let mut parsed_data = super::parse_data_from_file("../data/day1.txt");
       assert_eq!(1619, parsed_data.pop().unwrap())
    }
}