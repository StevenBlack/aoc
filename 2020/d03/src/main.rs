#![crate_name = "d03"]
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {

    let mut args = std::env::args();
    args.next();

    for arg in args {
        let pattern = file_to_vec_of_char_vec(arg);
        let mut field: Vec<Vec<char>> = vec![];
        for row in pattern {
            let mut cum= vec![];
            for _ in 0..100 {
                cum.push(&row[..])
            }
           field.push(cum.concat());
        }

        let runs = vec![1, 3, 5, 7, 1];
        let rises = vec![1, 1, 1, 1, 2];

        let mut iruns = runs.iter();
        let mut irises = rises.iter();

        let mut product = 1;
        let loops =runs.len();

        for _x in 0..loops {
            // the slide path
            let rise = irises.next().unwrap();
            let run = iruns.next().unwrap();

            // the origin
            let mut r: usize = 0;
            let mut c: usize = 0;

            let mut trees = 0;

            while r < (field.len() -1) {
                r = r + rise;
                c = c + run;
                // println!("{} {}", r, field[r][c]);
                if field[r][c] == "#".chars().next().unwrap() {
                    trees = trees + 1
                }
            }
            println!("Trees = {}", trees);
            product = product * trees;
        }
        println!("Product = {}", product);
    }

    Ok(())
}

fn file_to_vec_of_char_vec(filename: String) -> Vec<Vec<char>> {
    let readresult = file_to_vec(filename);
    let readvec = match readresult {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    readvec.iter().map(|l| l.chars().collect()).collect()
}

fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
