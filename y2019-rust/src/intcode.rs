use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

use Command::*;
use Comparison::*;
use Operation::*;
use Param::*;
use Pred::*;
use State::*;

pub type Mem = Vec<i64>;
#[derive(Clone)]
pub struct Program { pub mem: Mem, state: State, base: i64 }

pub fn read(filename: &str) -> Program {
    let file = File::open(filename).expect("No file");
    let reader = BufReader::new(file);

    let mem: Mem = reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    Program { mem, state: Run(0), base: 0 }
}

pub fn run(program: &mut Program, input: &mut dyn FnMut() -> i64, output: &mut dyn FnMut(i64) -> ()) {
    while program.state != Halted {
        run_till_io(program, input, output);
    }
}

pub fn run_till_io(program: &mut Program, input: &mut dyn FnMut() -> i64, output: &mut dyn FnMut(i64) -> ()) {
    loop {
        program.state = match program.state {
            Run(ip) => {
                run_till_interrupt(program, ip)
            },
            Input(p, ip) => {
                set(program, p, input());
                Run(ip)
            },
            Output(val, ip) => {
                output(val);
                Run(ip)
            }
            Halted => Halted,
        };

        if let Run(_) = program.state {
            // continue
        } else {
            break;
        }
    }
}

fn run_till_interrupt(d: &mut Program, mut p: usize) -> State {
    loop {
        let command = parse_command(&d.mem, p);
        match command {
            Arith(f, a, b, c) => {
                let v1 = get(d, a);
                let v2 = get(d, b);
                set(d, c, f.apply(v1, v2))
            },
            In(c) => return Input(c, p + get_length(&command)),
            Out(a) => return Output(get(d, a), p + get_length(&command)),
            Jump(f, a, b) => {
                if f.test(get(d, a)) {
                    p = get(d, b) as usize;
                } else {
                    p += get_length(&command);
                }
            },
            Comp(f, a, b, c) => {
                let v1 = get(d, a);
                let v2 = get(d, b);
                set_bool(d, c, f.compare(v1, v2))
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


impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Sum => a + b,
            Product => a * b,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Comparison {
    Less, Eq
}

impl Comparison {
    fn compare(&self, a: i64, b: i64) -> bool {
        match self {
            Less => a < b,
            Eq => a == b,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pred {
    True, False
}

impl Pred {
    fn test(&self, a: i64) -> bool {
        match self {
            True => a != 0,
            False => a== 0,
        }
    }
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

fn get_length(c: &Command) -> usize {
    match c {
        Arith(..) | Comp(..) => 4,
        In(..) | Out(..) | Base(..) => 2,
        Jump(..) => 3,
        Halt => 1,
    }
}

fn parse_command(d: &Mem, p: usize) -> Command {
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