use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::fs;
use regex::Regex;

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

    // now, check validity, part 1
    let mut validpassports = 0;
    for x in &vp {
        if x.len() == 8 {
            validpassports = validpassports + 1;
        } else if x.len() == 7 && (x.get_key_value(&"cid") == None) {
            validpassports = validpassports + 1;
        }
    }
    println!("Valid passports (Part 1) {}", validpassports);

    // now, check validity, part 2
    let mut validpassports = 0;
    let reqkeys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid = true;
    for x in &vp {
        valid = true;
        // do the keys test here
        'keystest: for k in &reqkeys {
            if !x.contains_key(k) {
                valid = false;
                break 'keystest;
            }
        }
        if valid {
            // now test the integer range values
            if checkrange(x,"byr", 1920u16, 2002u16) &&
               checkrange(x,"iyr", 2010u16, 2020u16) &&
               checkrange(x,"eyr", 2020u16, 2030u16) {
            } else {
                valid = false;
            }
        }
        if valid {
            // ckeck the passport id length
            if x["pid"].len() != 9 {
                valid = false;
            }
        }
        if valid {
            // ckeck the height
            // hgt (Height) - a number followed by either cm or in:
            let re = Regex::new(r"^\d{2,3}(cm|in)").unwrap();
            if !re.is_match(x["hgt"]) {
                valid = false;
            } else {
                let (cdigits, units) = x["hgt"].split_at(x["hgt"].len() -2);
                let hgt = cdigits.parse::<usize>().unwrap();
                if units == "cm" {
                    // If cm, the number must be at least 150 and at most 193.
                    if hgt < 150 || hgt > 193 {
                        valid = false;
                    }
                } else {
                    // If in, the number must be at least 59 and at most 76.
                    if hgt < 59 || hgt > 76 {
                        valid = false;
                    }
                }
            }
        }
        if valid {
            // ckeck the hair color
            let hc = x["hcl"];
            if !(hc.len() == 7 && hc.chars().next().unwrap().to_string() == "#".to_string()) {
                let l6:String = hc.chars().rev().take(6).collect();
                valid = false;
            }
        }
        if valid {
            // ckeck the eye color
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x["ecl"]) {
                valid = false;
            }
        }

        if valid {
            validpassports = validpassports + 1;
        }
    }
    println!("Valid passports (Part 2) {}", validpassports);

    Ok(())
}

fn checkrange(p: &HashMap<&str, &str>, key: &str, lo:u16, hi:u16) -> bool {
    let val: u16 = p[key].parse::<u16>().unwrap();
    if !(val >= lo && val <= hi) {
        return false;
    }
    true
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