#![crate_name = "d06"]

//! # Advent of code 2022
//! [day 6](https://adventofcode.com/2020/day/6)
//!
//! ## --- Day 6: Tuning Trouble ---
//!
//! The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.
//!
//! As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features, but the most important one to set up right now is the communication system.
//!
//! However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.
//!
//! As if inspired by comedic timing, the device emits a few colorful sparks.
//!
//! To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random characters that the device receives one at a time.
//!
//! To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.
//!
//! The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position where the four most recently received characters were all different. Specifically, it needs to report the number of characters from the beginning of the buffer to the end of the first such four-character marker.
//!
//! For example, suppose you receive the following datastream buffer:
//!
//! `mjqjpqmgbljsphdztnvjfqwrcgsmlb`
//!
//! After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.
//!
//! The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.
//!
//! Here are a few more examples:
//!
//! `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character 5
//! `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character 6
//! `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character 10
//! `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character 11
//!
//! How many characters need to be processed before the first start-of-packet marker is detected?
//!
//! Your puzzle answer was 1987.
//!
//! ### --- Part Two ---
//!
//! Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.
//!
//! A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.
//!
//! Here are the first positions of start-of-message markers for all of the above examples:
//!
//! `mjqjpqmgbljsphdztnvjfqwrcgsmlb`: first marker after character 19
//! `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character 23
//! `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character 23
//! `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character 29
//! `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character 26
//!
//! How many characters need to be processed before the first start-of-message marker is detected?
//!
//! Your puzzle answer was 3059.
//!
//! //! Both parts of this puzzle are complete! They provide two gold stars: **

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
    let sequence = packets_slice.windows(windowing_size)
      .enumerate()
      .find(|(_i, slice)| {
          let set = slice.iter().collect::<HashSet<_>>();
          slice.len() == set.len()
      })
    .unwrap();
    sequence.0 + windowing_size
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
