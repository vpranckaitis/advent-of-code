use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i32 {
    let xs = read();
    let mut res = 0;
    for x in xs {
        res += x / 3 - 2;
    }

    res
}

fn solve2() -> i32 {
    let xs = read();
    let mut res = 0;
    for mut x in xs {
        while x > 0 {
            x = max(x / 3 - 2, 0);
            res += x;
        }
    }

    res
}

fn read() -> Vec<i32> {
    let file = File::open("files/01.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.map(|s| s.parse::<i32>().expect("Could not parse")).expect(""))
        .collect()
}