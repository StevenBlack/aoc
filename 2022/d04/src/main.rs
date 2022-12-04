#![crate_name = "d04"]

//! # Advent of code 2022
//! [day 4](https://adventofcode.com/2020/day/4)
//!
//! ## --- Day 4: Camp Cleanup ---
//!
//! Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp. Every section has a unique ID number, and each Elf is assigned a range of section IDs.
//!
//! However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments overlap. To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list of the section assignments for each pair (your puzzle input).
//!
//! For example, consider the following list of section assignment pairs:
//!
//! ```
//! 2-4,6-8
//! 2-3,4-5
//! 5-7,7-9
//! 2-8,3-7
//! 6-6,4-6
//! 2-6,4-8
//! ```
//!
//! For the first few pairs, this list means:
//!
//! * Within the first pair of Elves, the first Elf was assigned sections 2-4 (sections 2, 3, and 4), while the second Elf was assigned sections 6-8 (sections 6, 7, 8).
//! * The Elves in the second pair were each assigned two sections.
//! * The Elves in the third pair were each assigned three sections: one got sections 5, 6, and 7, while the other also got 7, plus 8 and 9.
//!
//! This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers. Visually, these pairs of section assignments look like this:
//!
//! ```
//! .234.....  2-4
//! .....678.  6-8
//!
//! .23......  2-3
//! ...45....  4-5
//!
//! ....567..  5-7
//! ......789  7-9
//!
//! .2345678.  2-8
//! ..34567..  3-7
//!
//! .....6...  6-6
//! ...456...  4-6
//!
//! .23456...  2-6
//! ...45678.  4-8
//! ```
//!
//! Some of the pairs have noticed that one of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration. In this example, there are 2 such pairs.
//!
//! In how many assignment pairs does one range fully contain the other?
//!
//! Your puzzle answer was 556.
//!
//! ### --- Part Two ---
//!
//! It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.
//!
//! In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:
//!
//! * 5-7,7-9 overlaps in a single section, 7.
//! * 2-8,3-7 overlaps all of the sections 3 through 7.
//! * 6-6,4-6 overlaps in a single section, 6.
//! * 2-6,4-8 overlaps in sections 4, 5, and 6.
//!
//! So, in this example, the number of overlapping assignment pairs is 4.
//!
//! In how many assignment pairs do the ranges overlap?
//!
//! Your puzzle answer was 876.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **

use common::file_to_string;
use range_ext::intersect::*;
use std::fs;

fn main() {
    println!("Advent of code 2022 day 4");
    let input = file_to_string("input.txt".to_string());
    let part_one = one(input);
    println!("Part 1 result: {}", part_one);

    let part_two = two(read_input("input.txt"));
    println!("Part 2 result: {}", part_two);
}

fn one(input: String) -> u32 {
    let vec = input.lines().collect::<Vec<&str>>();
    let mut contained = 0;
    for s in vec {
        let splits = s.split(",");
        let mut bucket = vec![];
        for split in splits {
            let bounds = split.split(",");
            for bound in bounds {
                let lr: Vec<_> = bound.split("-").collect();
                let r = std::ops::Range {
                    start: lr[0].parse::<u32>().unwrap(),
                    end: lr[1].parse::<u32>().unwrap() + 1,
                };
                bucket.push(r);
            }
        }
        contained += match bucket[0].intersect(&bucket[1]) {
            Intersection::Bellow => 0, // no intersection
            Intersection::BellowOverlap => 0,
            Intersection::Within => 1,
            Intersection::Same => 1,
            Intersection::Over => 1,
            Intersection::AboveOverlap => 0,
            Intersection::Above => 0, // no intersection
        };
    }

    contained
}

fn two(input: String) -> u32 {
    let vec = input.lines().collect::<Vec<&str>>();
    // println!("{:?}", vec);
    let mut contained = 0;
    for s in vec {
        let splits = s.split(",");
        // println!("{:?}", splits);
        let mut bucket = vec![];
        for split in splits {
            let bounds = split.split(",");
            // println!("split: {:?}\nboungd: {:?}", split, bounds);
            for bound in bounds {
                let lr: Vec<_> = bound.split("-").collect();
                let r = std::ops::Range {
                    start: lr[0].parse::<u32>().unwrap(),
                    end: lr[1].parse::<u32>().unwrap() + 1,
                };
                bucket.push(r);
                // println!("r: {:?}", r);
            }
        }
        contained += match bucket[0].intersect(&bucket[1]) {
            Intersection::Bellow => 0, // no intersection
            Intersection::BellowOverlap => 1,
            Intersection::Within => 1,
            Intersection::Same => 1,
            Intersection::Over => 1,
            Intersection::AboveOverlap => 1,
            Intersection::Above => 0, // no intersection
        };
    }

    contained
}

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input: String = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(one(input), 2);
    }

    #[test]
    fn test_two() {
        let input: String = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(two(input), 4);
    }
}
