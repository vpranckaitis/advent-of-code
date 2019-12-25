use std::collections::VecDeque;

use crate::intcode::{ self, Program };
use std::io::{stdin, Read};

const FILENAME: &str = "files/25.in";

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let mut program = intcode::read(FILENAME);

    let z = vec![
        "space heater",
        "semiconductor",
        "planetoid",
        "hypercube",
        "spool of cat6",
        "sand",
        "festive hat",
        "dark matter"
    ];

    for i in 0..(1 << z.len()) {
        for j in 0..z.len() {
            if i & (1 << j) > 0 {
                println!("take {}", z[j]);
            }
        }
        println!("west");
        for j in 0..z.len() {
            if i & (1 << j) > 0 {
                println!("drop {}", z[j]);
            }
        }
    }

    let mut input = stdin().bytes();

    // traverse by hand, paste the output above once at Security checkpoint

    intcode::run(
        &mut program,
        &mut || input.next().unwrap().unwrap() as i64,
        &mut |val| print!("{}", char::from(val as u8))
    );

    0
}

fn solve2() -> i64 {
    let program = intcode::read(FILENAME);

    0
}