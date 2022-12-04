#![crate_name = "d04"]
use std::fs;

fn main() {
    println!("Advent of code 2022 day 4");
    let part_one = one(read_input("input.txt"));
    println!("Part 1 result: {}", part_one);

    let part_two = two(read_input("input.txt"));
    println!("Part 1 result: {}", part_two);
}

fn one(input: String) -> String {
    input
}

fn two(input: String) -> String {
    input
}

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}

// fn read_input(filename: &str) -> Vec<usize> {
//   fs::read_to_string(filename)
//       .expect("failed to read file")
//       .lines()
//       .map(|line: &str| line.parse::<usize>().expect("cannot parse a usize"))
//       .collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input: String = "".to_string();
        // assert_eq!(one(input), 157);
    }

    #[test]
    fn test_two() {
        let input: String = "".to_string();
        // assert_eq!(two(input), 70);
    }
}
