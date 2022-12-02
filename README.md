# Advent of Code using Rust

## introduction

Thank you to #hkennyv https://github.com/hkennyv/aoc for the structural and 
organization inspiration.

Advent of Code (AOC) is crated by [Eric Wastl](http://was.tl/) and is an
annual advent calendar of small programming puzzles. They can be solved
in any language (you don't really need code at all to solve some of them)
and its intended for people of all skill levels. There are a total of
25 exercises per year, one per day starting on December 1st and ending on
December 25th (Christmas).

You can find all of the advent of code exercises on the official website
here:

<https://adventofcode.com/events>

## Overview

I'm using AOC as an opportunity to sharpen my rust skills and get more
familiar with the language and toolchain.

This AOC crate serves as a dummy top-level create that links to each of the
other years.

The repository is structured as follows:

- **Each year is a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)**
- **Each day is a [cargo crate](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)**

Cheers & happy coding!

## How to use

This repository runs locally only. Refer to https://github.com/hkennyv/aoc which supports
using Replit too.

### Running locally

In order to run locally, you'll have to install the rust toolchain
using rustup. See below.

#### Prerequisites

- install rust using [rustup](https://www.rust-lang.org/tools/install) (this
should install the cargo toolchain as well). using the nightly build is
recommended

### Running the code

Any day of the AOC can be run by going into that day's directory and running:

```rust
$ cd 2020/day10
$ cargo run --release
```

**NOTE:** everyone's input for the advent of code is different. I've committed
the input for my account in this repository and the answers for my input in
the documentation, however, feel free to run the solutions using your own
input. Each crate reads from an "input.txt" file in their respective
directories.

## Documentation

**Bonus:** I've added the description of each day's challenge to the crate
description so you can view all of the challenges and all of the
functions and their docstrings that I used.

To build the documentation for each workspace, simply run the following
command in the year's workspace:

```rust
cargo doc --workspace --open
```

The documentation should automatically open up in your browser. Each of the
AOC days should appear in the left-hand sidebar as its own crate that you can
choose to view.

### README

**NOTE: this readme is generated using [cargo-readme](https://github.com/livioribeiro/cargo-readme).**

To edit the README, please edit the [`src/main.rs`](src/main.rs) file and
regenerate the README using the following command:

```rust
cargo readme > README.md
```

## Tests

Many crates also have some runnable tests that you can run using:

```rust
cargo test
```

Alternatively, run tests for the entire workspace using:

```rust
cargo test --all
```


License: MIT
