#![crate_name = "d04"]

use common::{*};

fn main() {
    println!("Advent of code 2022 day 4");
    one();
    two();
}

fn one() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();

    println!("Part 1 result: {:?}", vec);
}

fn two() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();

    println!("Part 2 result: {:?}", vec);
}
