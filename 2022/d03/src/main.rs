#![crate_name = "d03"]

//! ## --- Day 3: Rucksack Reorganization ---
//!
//! One Elf has the important job of loading all of the rucksacks with supplies for the jungle journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//!
//! Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! * The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
//! * The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
//! * The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
//! * The fourth rucksack's compartments only share item type v.
//! * The fifth rucksack's compartments only share item type t.
//! * The sixth rucksack's compartments only share item type s.
//!
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//! * Lowercase item types a through z have priorities 1 through 26.
//! * Uppercase item types A through Z have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
//!
//! Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?
//!
//! Your puzzle answer was 7980.
//!
//! ### --- Part Two ---
//!
//! As you finish identifying the misplaced items, the Elves come to you with another issue.
//!
//! For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.
//!
//! The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.
//!
//! Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:
//!
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! ```
//!
//! And the second group's rucksacks are the next three lines:
//!
//! ```
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.
//!
//! Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.
//!
//! Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
//!
//! Your puzzle answer was 2881.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **
//!
use std::fs;
use byte_set::ByteSet;

fn main() {
  println!("Advent of code 2022 day 3");
  let input = read_input("input.txt");

  let part_one = one(input.clone());
  println!("Part 1 result: {}", part_one);

  let part_two= two(input);
  println!("Part 2 result: {}", part_two);
}

fn one(input: String) -> u32 {
  let vec = input.lines().collect::<Vec<&str>>();
  let mut splits = vec!();
  for s in vec {
    let (first, second) = s.split_at(s.len()/2);
    // let set = str_to_set(first);
    let set = ByteSet::from(first);
    let mut ret: i32 = 0;
    for byte in second.as_bytes() {
      if set.contains(byte.to_owned()) {
          ret = byte.to_owned() as i32;
          break
      }
    }
    splits.push(score(ret as u32));
  }
  splits.iter().sum::<u32>()
}

fn two(input: String) -> u32 {
  let vec = input.lines().collect::<Vec<&str>>();
  let mut splits = vec!();
  let groups: Vec<&[&str]> = vec.chunks(3).collect();
  for group in groups {
    let ret = ByteSet::from(group[0])
      .intersection(ByteSet::from(group[1]))
      .intersection(ByteSet::from(group[2])).first().unwrap();
    splits.push(score(ret as u32));
  }
  splits.iter().sum::<u32>()
}

fn score(mut s: u32) -> u32 {
  if s >= 97 {
    s = s - 96;
  } else {
    s = s - 38;
  }
  s
}

pub fn read_input(filename: &str) -> String {
  fs::read_to_string(filename)
      .expect("failed to read file")
      // .lines()
      // // .map(|line: &str| line.parse::<usize>().expect("cannot parse a usize"))
      // .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_one() {
    let input: String = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();

    assert_eq!(one(input), 157);
  }

#[test]
  fn test_two() {
    let input: String = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
    assert_eq!(two(input), 70);
  }
}
