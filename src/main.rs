//! # Advent of Code
//!
//! **Use the links below to jump to a specific year**
//!
//! - [**AOC**](https://StevenBlack.github.io/aoc/aoc/index.html)
//! - [**2020**](https://StevenBlack.github.io/aoc/aoc/2020/doc/aoc_2020/index.html)
//! - [**2022**](https://StevenBlack.github.io/aoc/aoc/2021/doc/aoc_2022/index.html)
//!
//! # Introduction
//!
//! Advent of Code (AOC) is crated by [Eric Wastl](http://was.tl/) and is an
//! annual advent calendar of small programming puzzles. They can be solved
//! in any language (you don't really need code at all to solve some of them)
//! and its intended for people of all skill levels. There are a total of
//! 25 exercises per year, one per day starting on December 1st and ending on
//! December 25th (Christmas).
//!
//! You can find all of the advent of code exercises on the official website
//! here:
//!
//! <https://adventofcode.com/events>
//!
//! # Overview
//!
//! I'm using AOC as an opportunity to sharpen my rust skills and get more
//! familiar with the language and toolchain.
//!
//! This AOC crate serves as a dummy top-level create that links to each of the
//! other years.
//!
//! The repository is structured as follows:
//!
//! - **Each year is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)**
//! - **Each day is a [cargo crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)**
//!
//! Cheers & happy coding!
//!
//! ### Prerequisites
//!
//! - install rust using [rustup](https://www.rust-lang.org/tools/install) (this
//! should install the cargo toolchain as well). using the nightly build is
//! recommended
//!
//! ## Running the code
//!
//! Any day of the AOC can be run by going into that day's directory and running:
//!
//! ```
//! # e.g. if you want to run 2020/day10
//! $ cd 2020/day10
//! $ cargo run --release
//! ```
//!
//! **NOTE:** everyone's input for the advent of code is different. I've committed
//! the input for my account in this repository and the answers for my input in
//! the documentation, however, feel free to run the solutions using your own
//! input. Each crate reads from an "input.txt" file in their respective
//! directories.
//!
//! # Documentation
//!
//! **Bonus:** I've added the description of each day's challenge to the crate
//! description so you can view all of the challenges and all of the
//! functions and their docstrings that I used.
//!
//! To build the documentation for each workspace, simply run the following
//! command in the year's workspace:
//!
//! ```
//! cargo doc --workspace --open
//! ```
//!
//! The documentation should automatically open up in your browser. Each of the
//! AOC days should appear in the left-hand sidebar as its own crate that you can
//! choose to view.
//!
//! ## README
//!
//! **NOTE: this readme is generated using [cargo-readme](https://github.com/livioribeiro/cargo-readme).**
//!
//! To edit the README, please edit the [`src/main.rs`](src/main.rs) file and
//! regenerate the README using the following command:
//!
//! ```
//! cargo readme > README.md
//! ```
//!
//! # Tests
//!
//! Many crates also have some runnable tests that you can run using:
//!
//! ```
//! cargo test
//! ```
//!
//! Alternatively, run tests for the entire workspace using:
//!
//! ```
//! cargo test --all
//! ```
//!

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Hello, world!");
}
