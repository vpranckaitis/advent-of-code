use std::fs::File;
use std::io::{BufRead, BufReader};

use Command::*;
use Param::*;
use std::convert::TryInto;

type Program = Vec<i32>;

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i32 {
    let mut program = read();
    let output = run(&mut program, 1);
    match output.last() {
        Some(v) => *v,
        _ => panic!(),
    }
}

fn solve2() -> i32 {
    let mut program = read();
    let output = run(&mut program, 5);
    match output.last() {
        Some(v) => *v,
        _ => panic!(),
    }
}

fn run(d: &mut Program, input: i32) -> Vec<i32> {
    let mut p: usize = 0;
    let mut out = vec![];

    let mut command = parse_command(d, p);
    while command != Halt {
        match command {
            Add(a, b, c) => set(d, c, get(d, a) + get(d, b)),
            Mult(a, b, c) => set(d, c, get(d, a) * get(d, b)),
            In(c) => set(d, c, input),
            Out(a) => out.push(get(d, a)),
            JumpTrue(a, b) => if get(d, a) != 0 {
                p = get(d, b) as usize - get_length(&command);
            },
            JumpFalse(a, b) => if get(d, a) == 0 {
                p = get(d, b) as usize - get_length(&command);
            },
            LessThan(a, b, c) => set(d, c, if get(d, a) < get(d, b) { 1 } else { 0 }),
            Equals(a, b, c) => set(d, c, if get(d, a) == get(d, b) { 1 } else { 0 }),
            Halt => (),
        };
        p += get_length(&command);
        command = parse_command(d, p);
    }

    out
}

fn set(d: &mut Vec<i32>, p: Param, v: i32) {
    match p {
        Addr(a) => d[a] = v,
        Immed(..) => panic!(),
    };
}

fn get(d: &Program, p: Param) -> i32 {
    match p {
        Addr(a) => d[a],
        Immed(v) => v,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Param {
    Addr(usize),
    Immed(i32)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Command {
    Add(Param, Param, Param),
    Mult(Param, Param, Param),
    In(Param),
    Out(Param),
    JumpTrue(Param, Param),
    JumpFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    Halt,
}

fn get_length(c: &Command) -> usize {
    match c {
        Add(..) | Mult(..) | LessThan(..) | Equals(..) => 4,
        In(..) | Out(..) => 2,
        JumpTrue(..) | JumpFalse(..) => 3,
        Halt => 1,
    }
}

fn parse_command(d: &Program, p: usize) -> Command {
    let c = d[p] % 100;
    let m1 = d[p] / 100 % 10;
    let m2 = d[p] / 1000 % 10;
    let m3 = d[p] / 10000 % 10;
    match c {
        1 | 2 | 7 | 8 => {
            let p1 = parse_param(m1, d[p + 1]);
            let p2 = parse_param(m2, d[p + 2]);
            let p3 = parse_param(m3, d[p + 3]);
            match c {
                1 => Add(p1, p2, p3),
                2 => Mult(p1, p2, p3),
                7 => LessThan(p1, p2, p3),
                8 => Equals(p1, p2, p3),
                _ => panic!(),
            }
        },
        3 => In(parse_param(m1, d[p + 1])),
        4 => Out(parse_param(m1, d[p + 1])),
        5 | 6 => {
            let p1 = parse_param(m1, d[p + 1]);
            let p2 = parse_param(m2, d[p + 2]);
            match c {
                5 => JumpTrue(p1, p2),
                6 => JumpFalse(p1, p2),
                _ => panic!()
            }
        }
        99 => Halt,
        _ => {
            println!("{:?}", c);
            panic!()
        },
    }
}

fn parse_param(mode: i32, value: i32) -> Param {
    match mode {
        0 => Addr(value.try_into().unwrap()),
        1 => Immed(value),
        _ => panic!(),
    }
}

fn read() -> Program {
    let file = File::open("files/5.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}