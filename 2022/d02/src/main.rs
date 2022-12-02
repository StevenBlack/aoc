#![crate_name = "d02"]

use common::{*};

fn main() {
    day();
}

fn day() {
    println!("Advent of code 2022 day 2");

    // get the input data, trimming away blank lines.
    let input = file_to_string("input.txt".to_string());
    // split the input into groups for each elf.
    let elves = string_split_to_vec(input, "\n\n".to_string());

    // the vec of calories for each elf.
    let mut cals: Vec<u32> = vec!();

    for elf in elves {
        let mut elfstrvec = elf.split("\n").collect::<Vec<&str>>();
        // eliminate empty strings.
        elfstrvec.retain(|&x| x.len() > 0);
        cals.push(
            elfstrvec
            .iter()
            .map(|s| s.parse::<u32>()
            .unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .sum()
        );
    }

    let topelf = find_max_min(&cals).max;

    println!("Top elf is #{:?} with {} calories.", topelf.1 + 1, topelf.0);
    cals.sort();
    let clen = cals.len();
    println!("Top three elves total {:?}.", cals[clen-1] + cals[clen-2] + cals[clen-3]);
}
