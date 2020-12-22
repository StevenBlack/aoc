use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;

fn main() -> io::Result<()> {

    let mut args = std::env::args();
    args.next();

    let mut passp: Vec<String> = vec![];

    for arg in args {
        // read into a vec, blamk ;imnes are included here
        let data = file_to_vec(arg);

        // split into passports
        let mut buffer: String = "".to_string();
        for d in data.unwrap() {
            if d.len() == 0  {
                passp.push(buffer.trim().to_string());
                buffer.clear();
            } else {
                buffer.push(" ".chars().next().unwrap());
                buffer.push_str(d.as_str());
            }
        }
        // empty the buffer
        passp.push(buffer);
    }

    // now make a vec of passports
    let mut vp: Vec<HashMap<&str, &str>> = vec![];
    for v in &mut passp {
        let mut p = HashMap::new();
        // p.insert("valid", "no");
        let x: Vec<&str> = v.as_str().split(" ").collect::<Vec<&str>>();
        for kv in x {
            let k_v = kv.split(':').collect::<Vec<&str>>();
            if k_v.len() == 2 {
                p.insert(k_v[0], k_v[1]);
            }
        }
        vp.push(p);
    }

    // now, check validity
    let mut validpassports = 0;
    for x in &vp {
        if x.len() == 8 {
            validpassports = validpassports + 1;
        } else if x.len() == 7 && (x.get_key_value(&"cid") == None) {
            validpassports = validpassports + 1;
        }
    }
    println!("Valid passports: {}", validpassports);
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