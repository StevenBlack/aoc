#![crate_name = "d03"]

use common::{*};
use std::collections::HashSet;

fn main() {
    println!("Advent of code 2022 day 3");
    one();
    two();
}

fn one() {
    let input = file_to_string("input.txt".to_string());
    //  let mut vec = string_split_to_vec(input, "\n".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    for s in vec {
        let (first, second) = s.split_at(s.len()/2);
        let mut set = HashSet::new();

        for byte in first.as_bytes() {
            set.insert(byte.clone());
        }
        let mut ret: i32 = 0;
        for byte in second.as_bytes() {
            if set.get(byte).is_some() {
                ret = byte.to_owned() as i32;
                break
            }
        }
        if ret == 0 {
            panic!("No match found");
        }
        if ret >= 97 {
            ret = ret - 96;
        } else {
            ret = ret - 38;
        }
        splits.push(ret);
    }
    println!("Part 1 result: {}", splits.iter().sum::<i32>());
}

fn two() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    let groups: Vec<&[&str]> = vec.chunks(3).collect();
    for group in groups {
        let mut set = HashSet::new();
        let mut set2 = HashSet::new();
        let mut ret: i32 = 0;
        for byte in group[0].as_bytes() {
            set.insert(byte.clone());
        }
        for byte in group[1].as_bytes() {
            if set.get(byte).is_some() {
                set2.insert(byte.clone());
            }
        }
        for byte in group[2].as_bytes() {
            if set2.get(byte).is_some() {
                ret = byte.to_owned() as i32;
                break
            }
        }
        if ret == 0 {
            panic!("No match found");
        }
        if ret >= 97 {
            ret = ret - 96;
        } else {
            ret = ret - 38;
        }
        splits.push(ret);
    }
    println!("Part 2 result: {}", splits.iter().sum::<i32>());
}