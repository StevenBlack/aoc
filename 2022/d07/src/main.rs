#![crate_name = "d07"]

//! # Advent of code 2022
//! [day 5](https://adventofcode.com/2020/day/7)
//!

use common::file_to_string;
use std::collections::VecDeque;
use std::{cmp, fs};

fn main() {
    println!("Advent of code 2022 day 7");
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
