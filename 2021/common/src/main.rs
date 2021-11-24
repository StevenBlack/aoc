// common utils to support AOC puzzles
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

// read a file into Result<Vec<String>>
pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    if fs::metadata(filename.clone()).is_err() {
        panic!("Bad file {}", filename);
    }
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

// read a file into Vec<Vec<char>>
pub fn file_to_vec_of_char_vec(filename: String) -> Vec<Vec<char>> {
    let readresult = file_to_vec(filename);
    let readvec = match readresult {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    readvec.iter().map(|l| l.chars().collect()).collect()
}

// read a file into Vec<Vec<String>>
pub fn file_to_vec_of_words_vec(filename: String) -> Vec<Vec<char>> {
    let readresult = file_to_vec(filename);
    let readvec = match readresult {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    readvec.iter().map(|l| l.chars().collect()).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_readable_by_file_to_vec() {
        let my_vec = file_to_vec("input.txt".to_string()).unwrap();
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn input_readable_by_file_to_vec_of_char_vec() {
        let my_vec = file_to_vec_of_char_vec("input.txt".to_string());
        assert!(my_vec.len() > 0);
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_vec() {
       let _ = file_to_vec("badinput.txt".to_string());
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_vec_of_char_vec() {
       let _ = file_to_vec_of_char_vec("badinput.txt".to_string());
    }
}

// just for fun
fn main() {
    {
        let my_vec = file_to_vec("input.txt".to_string()).unwrap();
        println!("file_to_vec");
        println!("===========");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
    }
    {
        let my_vec = file_to_vec_of_char_vec("input.txt".to_string());
        println!("file_to_vec_of_char_vec");
        println!("=======================");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
    }
}
