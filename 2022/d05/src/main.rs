#![crate_name = "d05"]

//! # Advent of code 2022
//! [day 5](https://adventofcode.com/2020/day/5)
//!
//! # Advent of code 2022
//!
//! ## --- Day 5: Supply Stacks ---
//!
//! The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:
//! ```
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! ```
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.
//!
//! Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//! ```
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:
//! ```
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! ```
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:
//! ```
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! ```
//! Finally, one crate is moved from stack 1 to stack 2:
//! ```
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! ```
//! The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.
//!
//! After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was JRVNHHCSJ.
//!
//! ### --- Part Two ---
//!
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.
//!
//! Again considering the example above, the crates begin in the same configuration:
//! ```
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//! ```
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//! ```
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:
//! ```
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//! ```
//! Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:
//! ```
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//! ```
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:
//! ```
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! ```
//! In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
//!
//! Your puzzle answer was GNFBSBJLH.
//!
//! Both parts of this puzzle are complete! They provide two gold stars: **

use common::file_to_string;
use std::collections::VecDeque;
use std::{cmp, fs};

fn main() {
    println!("Advent of code 2022 day 5");
    let input = file_to_string("input.txt".to_string());
    let part_one = one(input);
    println!("Part 1 result: {}", part_one);

    let part_two = two(read_input("input.txt"));
    println!("Part 2 result: {}", part_two);
}

fn one(input: String) -> String {
    let t = input.clone();
    let raw = t.split("\n\n").collect::<Vec<&str>>();

    let mut vec = raw[0]
        .split("\n")
        .collect::<Vec<&str>>();
    // remove the "1 2 3" line (stack numbers)
    let _ = vec.pop();

    let instructions = raw[1].split("\n").collect::<Vec<&str>>();

    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::new(); 9];
    for mut line in vec {
        let mut v = vec![];
        while !line.is_empty() {
            let (chunk, rest) = line.split_at(cmp::min(4, line.len()));
            v.push(chunk.trim().replace("[", "").replace("]", ""));
            line = rest;
        }
        for (pos, e) in v.iter().enumerate() {
            if !e.is_empty() {
                stacks[pos].push_back(e.to_owned());
            }
        }
    }

    // proces instructions
    for inst in instructions {
        let words = inst.split(" ").collect::<Vec<&str>>();
        let (mv, fr, to) = (
            words[1].parse::<usize>().unwrap(),
            words[3].parse::<usize>().unwrap(),
            words[5].parse::<usize>().unwrap(),
        );
        for _i in 1..=mv {
            let mvd = stacks[fr - 1].pop_front().unwrap();
            stacks[to - 1].push_front(mvd);
        }
    }

    let fin = stacks.iter()
        .map(|v| match v.front() {
            Some(c) => c.to_string(),
            None => "".to_string(),
        })
        .collect();

    fin
}

fn two(input: String) -> String {
    let vecraw = input.lines().collect::<Vec<&str>>();
    let mut vec = vec![];
    let mut instructions = vec![];
    let mut inst = false;

    // resolve stacks
    for inputline in vecraw {
        if inputline.to_string().replace(" ", "") == "123" {
            continue;
        }
        if inputline.is_empty() {
            inst = true;
            continue;
        }
        if !inst {
            vec.push(inputline.to_owned());
        } else {
            instructions.push(inputline.to_owned());
        }
    }
    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::new(); 9];
    for mut line in vec {
        let mut v = vec![];
        while !line.is_empty() {
            let (chunk, rest) = line.split_at(cmp::min(4, line.len()));
            v.push(chunk.trim().replace("[", "").replace("]", ""));
            line = rest.to_string();
        }
        for (pos, e) in v.iter().enumerate() {
            if !e.is_empty() {
                stacks[pos].push_back(e.to_owned());
            }
        }
    }

    // proces instructions
    for inst in instructions {
        let words = inst.split(" ").collect::<Vec<&str>>();
        let (mv, fr, to) = (
            words[1].parse::<usize>().unwrap(),
            words[3].parse::<usize>().unwrap(),
            words[5].parse::<usize>().unwrap(),
        );
        let mut hold = vec![];
        for _i in 1..=mv {
            let mvd = stacks[fr - 1].pop_front().unwrap();
            hold.push(mvd);
        }
        while hold.len() > 0 {
            stacks[to - 1].push_front(hold.pop().unwrap());
        }
    }

    let fin = stacks.iter()
        .map(|v| match v.front() {
            Some(c) => c.to_string(),
            None => "".to_string(),
        })
        .collect();

    fin
}

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input: String = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        assert_eq!(one(input), "CMZ");
    }

    #[test]
    fn test_two() {
        let input: String = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .to_string();
        assert_eq!(two(input), "MCD");
    }
}
