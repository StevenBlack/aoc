// common utils to support AOC puzzles

mod common {
    use std::io::BufReader;
    use std::io::BufRead;
    use std::io;
    use std::fs;
    use std::io::prelude::*;
    use std::fs::File;

    // read a file to string
    pub fn file_to_string(filename: String) -> String {
        let mut file = File::open(filename).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        contents
    }

    pub fn string_to_vec(param: String) -> Vec<String> {
        param.lines().map(|l| l.to_string()).collect()
    }

    pub fn string_split_to_vec(param: String, splt: String) -> Vec<String> {
        param.split(splt.as_str()).map(|l| l.to_string()).collect()
    }

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

    // read a file into Vec<Vec<String>> (words)
    pub fn file_to_vec_of_words_vec(filename: String) -> Vec<Vec<String>> {
        // let v:Vec<&str> = "Hello, world!".split_whitespace().collect();
        // let v:Vec<String> = "Hello, world! I'm having a great day, you?".split_whitespace().map(|w| w.to_string()).collect();
        let readresult = file_to_vec(filename);
        let readvec = match readresult {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let vecvec :Vec<Vec<String>> = readvec.iter().map(|l| l.split_whitespace().map(|l| l.to_string()).collect()).collect();
        vecvec
    }
}
#[cfg(test)]
mod tests {
    use super::common::*;

    #[test]
    fn check_string_to_vec() {
        let myvec = string_to_vec("line 1\nLine 2".to_string());
        assert!(myvec.len() == 2);
    }

    #[test]
    fn input_readable_by_file_to_string() {
        let mystring = file_to_string("input2.txt".to_string());
        assert!(mystring.len() > 0);
    }

    #[test]
    #[should_panic]
    fn bad_input_file_to_string() {
       let _ = file_to_string("badinput.txt".to_string());
    }

    #[test]
    fn convert_strin_to_vec() {
        let my_string = file_to_string("input2.txt".to_string());
        let my_vec=string_to_vec(my_string);
        assert!(my_vec.len() > 0);
    }

    #[test]
    fn convert_string_split_to_vec() {
        let my_string = file_to_string("input2.txt".to_string());
        let my_vec=string_split_to_vec(my_string, 'r'.to_string());
        assert!(my_vec.len() > 0);
    }

    #[test]
    #[should_panic]
    fn convert_strin_to_vec_panic() {
        let my_string = file_to_string("badinput.txt".to_string());
        let my_vec=string_to_vec(my_string);
        assert!(my_vec.len() > 0);
    }

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
    fn input_readable_by_file_to_vec_of_words_vec() {
        let my_vec = file_to_vec_of_words_vec("input2.txt".to_string());
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
    use common::*;
    {
        let my_vec = string_to_vec("L1\nL2\nL3".to_string());
        println!("string_to_vec");
        println!("===========");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
        println!("");
    }

    {
        let my_string = file_to_string("input2.txt".to_string());
        println!("file_to_string");
        println!("==============");
        println!("vec.len() = {:?}", my_string.len());
        println!("");
    }
    {
        let my_vec = file_to_vec("input.txt".to_string()).unwrap();
        println!("file_to_vec");
        println!("===========");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
        println!("");
    }
    {
        let my_vec = file_to_vec_of_char_vec("input.txt".to_string());
        println!("file_to_vec_of_char_vec");
        println!("=======================");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
        println!("");
    }
    {
        let my_vec = file_to_vec_of_words_vec("input2.txt".to_string());
        println!("file_to_vec_of_words_vec");
        println!("========================");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next().unwrap());
        println!("vec[0..3] is {:?}", my_vec[0..3].to_vec());
        println!("");
    }
    {
        let my_string = file_to_string("input2.txt".to_string());
        let my_vec = string_split_to_vec(my_string, "r".to_owned());

        println!("string_split_to_vec");
        println!("===================");
        println!("vec.len() = {:?}", my_vec.len());
        println!("vec[0] is {:?}", my_vec.iter().next());
        println!("vec[0..10] is {:?}", my_vec[0..3].to_vec());
        println!("");
    }
}
