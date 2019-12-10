use std::fs::File;
use std::io::{BufRead, BufReader};

use Command::*;
use Param::*;
use State::*;
use std::convert::TryInto;
use std::cmp::max;

type Program = Vec<i32>;

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const AMP_COUNT: usize = 5;

fn solve1() -> i32 {
    let program = read();
    generate_phases(&vec![0, 1, 2, 3, 4], &mut vec![], 0, &program)
}

fn solve2() -> i32 {
    let program = read();
    generate_phases(&vec![5, 6, 7, 8, 9], &mut vec![], 0, &program)
}

fn run_loop(d: &Program, phases: [i32; AMP_COUNT]) -> i32 {
    let mut programs = [d.clone(), d.clone(), d.clone(), d.clone(), d.clone()];
    let mut input = [vec![], vec![], vec![], vec![], vec![]];
    for i in 0..AMP_COUNT {
        input[i].push(phases[i]);
    }
    input[0].push(0);
    let mut states = [Run(0); AMP_COUNT];

    let mut pid = 0;
    loop {
        states[pid] = match states[pid] {
            Run(ip) => {
                run(&mut programs[pid], ip)
            },
            Input(p, ip) if input[pid].len() > 0 => {
                set(&mut programs[pid], p, input[pid].remove(0));
                Run(ip)
            },
            Output(output, ip) => {
                input[(pid + 1) % AMP_COUNT].push(output);
                Run(ip)
            }
            _ => {
                pid = (pid + 1) % AMP_COUNT;
                states[pid]
            },
        };
        let all_halted = states.iter().all(|st| if let Halted = st { true } else { false });
        if all_halted {
            break;
        }
    }
    *input[0].last().unwrap()
}

fn generate_phases(range: &Vec<usize>, phases: &mut Vec<i32>, depth: usize,
                   program: &Program) -> i32 {
    if depth == 5 {
        return run_loop(program, [phases[0], phases[1], phases[2], phases[3], phases[4]]);
    }
    let mut mx = -1000000000;
    for &i in range {
        if !phases.contains(&(i as i32)) {
            phases.push(i as i32);
            mx = max(mx, generate_phases(range, phases, depth + 1, program));
            phases.pop();
        }
    }
    mx
}

fn run(d: &mut Program, mut p: usize) -> State {
    while let command = parse_command(&d, p) {
        match command {
            Arith(f, a, b, c) => set(d, c, f(&get(&d, a), &get(&d, b))),
            In(c) => return Input(c, p + get_length(&command)),
            Out(a) => return Output(get(&d, a), p + get_length(&command)),
            Jump(pred, a, b) => if pred(&get(&d, a)) {
                p = get(&d, b) as usize - get_length(&command);
            },
            Comp(comp, a, b, c) => set_bool(d, c, comp(&get(&d, a), &get(&d, b))),
            Halt => break,
        };
        p += get_length(&command);
    };
    Halted
}

fn set(d: &mut Vec<i32>, p: Param, v: i32) {
    match p {
        Addr(a) => d[a] = v,
        Immed(..) => panic!(),
    };
}


fn set_bool(d: &mut Vec<i32>, p: Param, v: bool) {
    match p {
        Addr(a) => d[a] = if v { 1 } else { 0 },
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
enum State {
    Run(usize),
    Input(Param, usize),
    Output(i32, usize),
    Halted
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Param {
    Addr(usize),
    Immed(i32)
}

#[derive(Copy, Clone)]
enum Command {
    Arith(fn(&i32, &i32) -> i32, Param, Param, Param),
    In(Param),
    Out(Param),
    Jump(fn(&i32) -> bool, Param, Param),
    Comp(fn(&i32, &i32) -> bool, Param, Param, Param),
    Halt,
}

fn get_length(c: &Command) -> usize {
    match c {
        Arith(..) | Comp(..) => 4,
        In(..) | Out(..) => 2,
        Jump(..) => 3,
        Halt => 1,
    }
}

fn parse_command(d: &Program, p: usize) -> Command {
    //println!("{}", p);
    //println!("{:?}", &d[p..(p + 4)]);
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
                1 => Arith(|a, b| a + b, p1, p2, p3),
                2 => Arith(|a, b| a * b, p1, p2, p3),
                7 => Comp(|a, b| a < b, p1, p2, p3),
                8 => Comp(|a, b| a == b, p1, p2, p3),
                _ => panic!(),
            }
        },
        3 => In(parse_param(m1, d[p + 1])),
        4 => Out(parse_param(m1, d[p + 1])),
        5 | 6 => {
            let p1 = parse_param(m1, d[p + 1]);
            let p2 = parse_param(m2, d[p + 2]);
            match c {
                5 => Jump(|a| *a != 0, p1, p2),
                6 => Jump(|a| *a == 0, p1, p2),
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
    let file = File::open("files/07.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}