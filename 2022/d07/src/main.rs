#![crate_name = "d07"]

//! # Advent of code 2022
//! [day 5](https://adventofcode.com/2020/day/7)
//!

use common::file_to_string;
use std::collections::VecDeque;
use std::{cmp, fs, default};

fn main() {
    println!("Advent of code 2022 day 7");
    let input = file_to_string("input.txt".to_string());
    let part_one = one(input);
    println!("Part 1 result: {}", part_one);

    let part_two = two(read_input("input.txt"));
    println!("Part 2 result: {}", part_two);
}

struct File {
    pub name: String,
    pub size: u32,
}

impl Default for File {
    fn default() -> File {
        File {
          name: "x".to_string(),
          size: 0,
        }
    }
}

struct Folder {
    pub name: String,
    pub files: Vec<File>,
    pub folders: Vec<Folder>,
}

impl Default for Folder {
    fn default() -> Folder {
        Folder {
          name: ".".to_string(),
          files: vec!(),
          folders: vec!(),
        }
    }
}

impl Folder {
    pub fn add_file(&mut self, f:File) {
        self.files.push(f);
    }
    pub fn add_folder(&mut self, f:Folder) {
        self.folders.push(f);
    }
    pub fn get_size(&self) -> u32{
        self.files
            .iter()
            .map(|f| f.size)
            .sum::<u32>()
            +
            self.folders
            .iter()
            .map(|f| f.get_size())
            .sum::<u32>()
    }
}

#[test]
fn test_this() {
    let mut root = Folder { ..Default::default()};
    let file = File { name: "x.txt".to_string(), size: 100 };
    root.add_file(file);
    assert_eq!(root.get_size(), 100);
}

#[test]
fn test_this_deeper() {
    let mut root = Folder { ..Default::default()};
    let mut current_folder = *root;
    let mut sub = Folder { ..Default::default()};
    let file1 = File { name: "x.txt".to_string(), size: 100 };
    let file2 = File { name: "x.txt".to_string(), size: 100 };
    let file3 = File { name: "x.txt".to_string(), size: 100 };
    root.add_file(file1);
    sub.add_file(file2);
    sub.add_file(file3);
    root.add_folder(sub);
    assert_eq!(root.get_size(), 300);
}

fn one(input: String) -> u32 {
    let raw = input.lines().collect::<Vec<&str>>();
    let root = Folder { name: "/".to_string(), ..Default::default() };

    100
}

fn two(input: String) -> u32 {
    let raw = input.lines().collect::<Vec<&str>>();

    100

}

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("failed to read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input: String = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
            .to_string();
        assert_eq!(one(input), 95437);
    }

    #[test]
    #[ignore]
    fn test_two() {
        let input: String = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".to_string();
        assert_eq!(two(input), 100);
    }
}
