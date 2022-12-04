// common utils to support AOC puzzles

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;

// find the max and min of a vector of numbers
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaxMin<T> {
    pub max: (T, usize),
    pub min: (T, usize),
}

#[allow(dead_code)]
pub fn find_max_min<T: std::cmp::PartialOrd + Copy>(slice: &[T]) -> MaxMin<T> {
    let mut max = &slice[0];
    let mut min = &slice[0];

    let mut max_pos: usize = 0;
    let mut min_pos: usize = 0;

    for index in 1..slice.len() {
        if slice[index] < *min {
            min = &slice[index];
            min_pos = index;
        }
        if slice[index] > *max {
            max = &slice[index];
            max_pos = index;
        }
    }

    MaxMin {
        max: (*max, max_pos),
        min: (*min, min_pos),
    }
}

// read a file to string
#[allow(dead_code)]
pub fn file_to_string(filename: String) -> String {
    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

#[allow(dead_code)]
pub fn string_to_vec(param: String) -> Vec<String> {
    param.lines().map(|l| l.to_string()).collect()
}

#[allow(dead_code)]
pub fn string_split_to_vec(param: String, splt: String) -> Vec<String> {
    param.split(splt.as_str()).map(|l| l.to_string()).collect()
}

// read a file into Result<Vec<String>>
#[allow(dead_code)]
pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    if fs::metadata(filename.clone()).is_err() {
        panic!("Bad file {}", filename);
    }
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

// read a file into Vec<Vec<char>>
#[allow(dead_code)]
pub fn file_to_vec_of_char_vec(filename: String) -> Vec<Vec<char>> {
    let readresult = file_to_vec(filename);
    let readvec = match readresult {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    readvec.iter().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
fn file_to_vec_of_usize(filename: String) -> Vec<usize> {
    fs::read_to_string(filename)
        .expect("failed to read file")
        .lines()
        .map(|line: &str| line.parse::<usize>().expect("cannot parse a usize"))
        .collect()
}

// read a file into Vec<Vec<String>> (words)
#[allow(dead_code)]
pub fn file_to_vec_of_words_vec(filename: String) -> Vec<Vec<String>> {
    // let v:Vec<&str> = "Hello, world!".split_whitespace().collect();
    // let v:Vec<String> = "Hello, world! I'm having a great day, you?".split_whitespace().map(|w| w.to_string()).collect();
    let readresult = file_to_vec(filename);
    let readvec = match readresult {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let vecvec: Vec<Vec<String>> = readvec
        .iter()
        .map(|l| l.split_whitespace().map(|l| l.to_string()).collect())
        .collect();
    vecvec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_string_to_vec2() {
        let myvec = string_to_vec("line 1\nLine 2".to_string());
        assert!(myvec.len() == 2);
    }

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn check_string_to_vec() {
        let myvec = super::string_to_vec("line 1\nLine 2".to_string());
        assert!(myvec.len() == 2);
    }
}
