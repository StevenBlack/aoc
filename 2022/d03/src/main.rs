#![crate_name = "d03"]

use common::{*};

fn main() {
    println!("Advent of code 2022 day 3");
    one();
    two();
}

fn one() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    for s in vec {
        let (first, second) = s.split_at(s.len()/2);
        let set = str_to_set(first);
        let mut ret: i32 = 0;
        for byte in second.as_bytes() {
            if set.contains(byte.to_owned()) {
                ret = byte.to_owned() as i32;
                break
            }
        }
        splits.push(score(ret as u32));
    }
    println!("Part 1 result: {}", splits.iter().sum::<u32>());
}

fn two() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    let groups: Vec<&[&str]> = vec.chunks(3).collect();
    for group in groups {
        let ret = str_to_set(group[0])
          .intersection(str_to_set(group[1]))
          .intersection(str_to_set(group[2])).first().unwrap();
        splits.push(score(ret as u32));
    }
    println!("Part 2 result: {}", splits.iter().sum::<u32>());
}

fn score(mut s: u32) -> u32 {
  if s >= 97 {
    s = s - 96;
  } else {
    s = s - 38;
  }
  s
}