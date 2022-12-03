#![crate_name = "d03"]

use common::{*};
use byte_set::ByteSet;

/// Creates a [`ByteSet`] from a sequence of [`u8`]s.
///
/// `byte_set!` allows `ByteSet`s to be defined with the same syntax as [`vec!`]
/// or array expressions.
///
/// # Examples
///
/// This can be used within a `const` context:
///
/// ```
/// # use byte_set::{byte_set, ByteSet};
/// const SET: ByteSet = byte_set!(1, 2, 3, b'a', b'b', b'c');
///
/// assert!(SET.contains(b'a'));
/// ```
///
/// [`ByteSet`]: struct.ByteSet.html
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
/// [`vec!`]: https://doc.rust-lang.org/std/macro.vec.html
#[macro_export]
macro_rules! byte_set {
    ($($byte:expr,)*) => {
        $crate::ByteSet::new() $(.inserting($byte))*
    };
    ($($byte:expr),*) => {
        $crate::byte_set!($($byte,)*)
    };
}

fn main() {
    println!("Advent of code 2022 day 3");
    one();
    two();
}

fn one() {
    // let xx = str_to_set("abcdeabcde");
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    for s in vec {
        let (first, second) = s.split_at(s.len()/2);
        let mut set = ByteSet::new();
        for byte in first.as_bytes() {
            set.insert(byte.clone());
        }
        let mut ret: i32 = 0;
        for byte in second.as_bytes() {
            // if set.get(byte).is_some() {
            if set.contains(byte.to_owned()) {
                ret = byte.to_owned() as i32;
                break
            }
        }
        if ret == 0 {
            panic!("No match found");
        }
        if ret >= 97 {
            ret = ret - 96;
        } else {
            ret = ret - 38;
        }
        splits.push(ret);
    }
    println!("Part 1 result: {}", splits.iter().sum::<i32>());
}

fn two() {
    let input = file_to_string("input.txt".to_string());
    let vec = input.lines().collect::<Vec<&str>>();
    let mut splits = vec!();
    let groups: Vec<&[&str]> = vec.chunks(3).collect();
    for group in groups {
        let mut set = ByteSet::new();
        let mut set2 = ByteSet::new();
        let mut ret: i32 = 0;
        for byte in group[0].as_bytes() {
            set.insert(byte.clone());
        }
        for byte in group[1].as_bytes() {
            if set.contains(byte.to_owned()) {
                set2.insert(byte.clone());
            }
        }
        for byte in group[2].as_bytes() {
            if set2.contains(byte.to_owned()) {
                ret = byte.to_owned() as i32;
                break
            }
        }
        if ret == 0 {
            panic!("No match found");
        }
        if ret >= 97 {
            ret = ret - 96;
        } else {
            ret = ret - 38;
        }
        splits.push(ret);
    }
    println!("Part 2 result: {}", splits.iter().sum::<i32>());
}