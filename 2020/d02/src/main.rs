#![crate_name = "d02"]
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    args.next();

    // part 1
    let mut valid = 0;
    let mut valid2 = 0;
    for arg in args {
        let lines = file_to_vec(arg)?;
        for line in lines {
            // let three = line.split(" ").collect::<Vec<&str>>();
            let three = line.split_whitespace().collect::<Vec<&str>>();
            let mm = three[0].split("-").collect::<Vec<&str>>();
            let lo = mm[0].parse::<usize>().unwrap();
            let hi = mm[1].parse::<usize>().unwrap();
            let ch = three[1].replace(":", "");
            let c = three[2].matches(&ch).count();
            if c >= lo && c <= hi {
                valid = valid + 1;
            }

            let mut hits = 0;
            if three[2].chars().nth(lo-1).is_some() {
                if three[2].chars().nth(lo-1).unwrap() == ch.chars().nth(0).unwrap() {
                    hits = hits + 1
                }
            }
            if three[2].chars().nth(hi-1).is_some() {
                if three[2].chars().nth(hi-1).unwrap() == ch.chars().nth(0).unwrap() {
                    hits = hits + 1
                }
            }
            if hits == 1 {
                println!("{}", line);
                valid2 = valid2 + 1;
            }

        }
        println!("Valid passwords: {}", valid);  // 524
        println!("Valid2 passwords: {}", valid2); // 426
    }

    Ok(())
}


fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}
