use crate::intcode::{self, Program};

const FILENAME: &str = "files/19.in";

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let program = intcode::read(FILENAME);

    let mut count = 0;

    for x in 0..50 {
        for y in 0..50 {
            count += test(program.clone(), x, y);
        }
    }

    count
}

fn solve2() -> i64 {
    let program = intcode::read(FILENAME);

    let mut x = 0;
    let mut y = 100;

    loop {
        let val = test(program.clone(), x, y);
        if val == 1 {
            if y >= 100 && test(program.clone(), x + 99, y - 99) == 1 {
                return x * 10000 + y - 99;
            } else {
                y += 1;
            }
        } else {
            x += 1;
        }
    }
}

fn test(mut program: Program, x: i64, y: i64) -> i64 {
    let mut input = vec![y, x];
    let mut output = 0;
    intcode::run(
        &mut program,
        &mut || input.pop().unwrap(),
        &mut |val| output += val);
    output
}