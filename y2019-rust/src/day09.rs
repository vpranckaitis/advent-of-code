use std::fs::File;
use std::io::{BufRead, BufReader};

use Command::*;
use Param::*;
use Operation::*;
use Comparison::*;
use Pred::*;
use State::*;
use std::convert::TryInto;

type Mem = Vec<i64>;
struct Program { mem: Mem, base: i64 }

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let mem = read();
    let output = run(&mut Program { mem, base: 0 }, &mut vec![1]);
    match output[..] {
        [x] => x,
        _ => panic!(),
    }
}

fn solve2() -> i64 {
    let mem = read();
    let output = run(&mut Program { mem, base: 0 }, &mut vec![2]);
    match output[..] {
        [x] => x,
        _ => panic!(),
    }
}

fn run(program: &mut Program, input: &mut Vec<i64>) -> Vec<i64> {
    let mut state = Run(0);
    let mut output: Vec<i64> = vec![];
    loop {
        state = match state {
            Run(ip) => {
                run_till_interrupt(program, ip)
            },
            Input(p, ip) if input.len() > 0 => {
                set(program, p, input.remove(0));
                Run(ip)
            },
            Output(val, ip) => {
                output.push(val);
                Run(ip)
            }
            _ => state,
        };
        if let Halted = state {
            break;
        }
    }
    output
}

fn run_till_interrupt(d: &mut Program, mut p: usize) -> State {
    while let command = parse_command(&d.mem, p) {
        //println!("{:?}", command);
        match command {
            Arith(ff, a, b, c) => {
                let v1 = get(d, a);
                let v2 = get(d, b);
                let f = get_arith(ff);
                set(d, c, f(&v1, &v2))
            },
            In(c) => return Input(c, p + get_length(&command)),
            Out(a) => return Output(get(d, a), p + get_length(&command)),
            Jump(ff, a, b) => {
                let pred = get_pred(ff);
                if pred(&get(d, a)) {
                    p = get(d, b) as usize;
                } else {
                    p += get_length(&command);
                }
            },
            Comp(ff, a, b, c) => {
                let v1 = get(d, a);
                let v2 = get(d, b);
                let comp = get_comp(ff);
                set_bool(d, c, comp(&v1, &v2))
            },
            Base(a) => {
                let v = get(d, a);
                d.base += v;
            }
            Halt => break,
        };
        if let Jump(..) = command {
            // ignore
        } else {
            p += get_length(&command);
        }
    };
    Halted
}

fn set(d: &mut Program, p: Param, v: i64) {
    let pos = match p {
        Addr(a) => a,
        Rel(a) => d.base + a,
        Immed(..) => panic!(),
    };
    *get_memory_location(&mut d.mem, pos) = v;
}


fn set_bool(d: &mut Program, p: Param, v: bool) {
    set(d, p, if v { 1 } else { 0 });
}

fn get(d: &mut Program, p: Param) -> i64 {
    let pos = match p {
        Addr(a) => a,
        Rel(a) => d.base + a,
        Immed(val) => return val,
    };
    *get_memory_location(&mut d.mem, pos)
}

fn get_memory_location(mem: &mut Mem, pos: i64) -> &mut i64 {
    if pos < 0 {
        panic!();
    }
    if pos >= mem.len() as i64 {
        mem.resize((pos + 1) as usize, 0);
    }
    &mut mem[pos as usize]
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Run(usize),
    Input(Param, usize),
    Output(i64, usize),
    Halted
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Param {
    Addr(i64),
    Immed(i64),
    Rel(i64)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Sum, Product
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Comparison {
    Less, Eq
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pred {
    True, False
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Command {
    Arith(Operation, Param, Param, Param),
    In(Param),
    Out(Param),
    Jump(Pred, Param, Param),
    Comp(Comparison, Param, Param, Param),
    Base(Param),
    Halt,
}

fn get_arith(d: Operation) -> (fn(&i64, &i64) -> i64) {
    match d {
        Sum => |a, b| a + b,
        Product => |a, b| a * b,
    }
}

fn get_comp(d: Comparison) -> (fn(&i64, &i64) -> bool) {
    match d {
        Less => |a, b| a < b,
        Eq => |a, b| a == b,
    }
}

fn get_pred(d: Pred) -> (fn(&i64) -> bool) {
    match d {
        True => |&a| a != 0,
        False => |&a| a== 0,
    }
}

fn get_length(c: &Command) -> usize {
    match c {
        Arith(..) | Comp(..) => 4,
        In(..) | Out(..) | Base(..) => 2,
        Jump(..) => 3,
        Halt => 1,
    }
}

fn parse_command(d: &Mem, p: usize) -> Command {
    //println!("{}\n{:?}", p, &d[p..min(d.len(), p + 4)]);
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
                1 => Arith(Sum, p1, p2, p3),
                2 => Arith(Product, p1, p2, p3),
                7 => Comp(Less, p1, p2, p3),
                8 => Comp(Eq, p1, p2, p3),
                _ => panic!(),
            }
        },
        3 => In(parse_param(m1, d[p + 1])),
        4 => Out(parse_param(m1, d[p + 1])),
        5 | 6 => {
            let p1 = parse_param(m1, d[p + 1]);
            let p2 = parse_param(m2, d[p + 2]);
            match c {
                5 => Jump(True, p1, p2),
                6 => Jump(False, p1, p2),
                _ => panic!()
            }
        },
        9 => Base(parse_param(m1, d[p + 1])),
        99 => Halt,
        _ => {
            println!("{:?}", c);
            panic!()
        },
    }
}

fn parse_param(mode: i64, value: i64) -> Param {
    match mode {
        0 => Addr(value.try_into().unwrap()),
        1 => Immed(value),
        2 => Rel(value),
        _ => panic!(),
    }
}

fn read() -> Mem {
    let file = File::open("files/09.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}