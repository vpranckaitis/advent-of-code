use std::collections::VecDeque;

use crate::intcode;

const FILENAME: &str = "files/21.in";

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let mut program = intcode::read(FILENAME);

    let mut input: VecDeque<i64> = vec![
        "NOT A T\n",
        "OR T J\n",
        "NOT B T\n",
        "OR T J\n",
        "NOT C T\n",
        "OR T J\n",
        "AND D J\n",
        "WALK\n"
    ].iter()
        .flat_map(|s| s.bytes())
        .map(|b| b as i64)
        .collect();

    let mut res = 0;

    intcode::run(
        &mut program,
        &mut || input.pop_front().unwrap(),
        &mut |val| if val > 255 { res = val } else { print!("{}", char::from(val as u8)) });

    res
}

fn solve2() -> i64 {
    let mut program = intcode::read(FILENAME);

    let mut input: VecDeque<i64> = vec![
        "OR F J\n",
        "OR I J\n",
        "AND E J\n",
        "OR H J\n",
        "AND D J\n",
        "NOT T T\n",
        "AND A T\n",
        "AND B T\n",
        "AND C T\n",
        "NOT T T\n",
        "AND T J\n",
        "RUN\n"
    ].iter()
        .flat_map(|s| s.bytes())
        .map(|b| b as i64)
        .collect();

    let mut res = 0;

    intcode::run(
        &mut program,
        &mut || input.pop_front().unwrap(),
        &mut |val| if val > 255 { res = val } else { print!("{}", char::from(val as u8)) });

    res
}