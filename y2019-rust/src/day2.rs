use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

pub fn solve1() -> i32 {
    let mut xs = read();

    xs[1] = 12;
    xs[2] = 2;

    run(&mut xs)
}

fn solve2() -> i32 {
    let xs = read();
    for i in 0..99 {
        for j in 0..99 {
            let mut cp = xs.clone();
            cp[1] = i;
            cp[2] = j;
            if run(&mut cp) == 19690720 {
                return 100 * i + j;
            }
        }
    }
    -1
}

fn run(xs: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    while xs[i] != 99 {
        let a = xs[i + 1] as usize;
        let b = xs[i + 2] as usize;
        let val = if xs[i] == 1 {
            xs[a] + xs[b]
        } else if xs[i] == 2 {
            xs[a] * xs[b]
        } else {
            panic!()
        };
        let c = xs[i + 3] as usize;
        xs[c] = val;
        i += 4;
    }

    xs[0]
}

fn read() -> Vec<i32> {
    let file = File::open("files/2.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}