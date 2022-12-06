#![crate_name = "d06"]

//! # Advent of code 2022
//! [day 6](https://adventofcode.com/2020/day/6)
//!

use common::file_to_string;
use std::collections::{HashSet};
use std::{fs};

fn main() {
    println!("Advent of code 2022 day 6");
    let input = file_to_string("input.txt".to_string());
    let part_one = one(&input, 4);
    println!("Part 1 result: {}", part_one);

    let part_two = one(&input, 14);
    println!("Part 2 result: {}", part_two);
}

fn one(input: &str, windowing_size: usize) -> usize {
    // let windowing_size = 4;
    let packets = String::from(input);
    let packets_slice = packets.chars().collect::<Vec<char>>();
    let windows = packets_slice.windows(windowing_size);
    let mut count = windowing_size;
    for w in windows {
        if w.len()
            == w.into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .len()
        {
            break;
        }
        count += 1;
    }
    count
}


pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(one("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(one("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]

    fn test_two() {
        assert_eq!(one("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(one("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(one("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
