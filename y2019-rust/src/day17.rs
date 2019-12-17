use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

use Command::*;
use Comparison::*;
use Operation::*;
use Param::*;
use Pred::*;
use State::*;

type Point = (i64, i64);
type Mem = Vec<i64>;
#[derive(Clone)]
struct Program { mem: Mem, base: i64 }

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let mem = read();
    let (map, _, _, _) = run(&mut Program { mem, base: 0 }, &"".to_string());

    let dirs: [(isize, isize); 5] = [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)];

    let mut res: i64 = 0;
    for y in 1..(map.len() - 1) {
        for x in 1..(map[y].len() - 1) {
            let is_cross = dirs.iter()
                .all(|&(dy, dx)| map[((y as isize) + dy) as usize][((x as isize) + dx) as usize]);
            if is_cross {
                res += (y * x) as i64;
            }
        }
    }
    res
}

fn solve2() -> i64 {
    let mut mem = read();
    let (map, mut pos, mut dir, _) = run(&mut Program { mem: mem.clone(), base: 0 }, &"".to_string());

    let mut route: Vec<String> = vec![];
    let mut steps = 0;
    loop {
        let forw = add(pos, dir);
        let right_dir = turn(dir, true);
        let right = add(pos, right_dir);
        let left_dir = turn(dir, false);
        let left = add(pos, left_dir);

        let (new_pos, new_dir) = if test(&map, forw) {
            steps += 1;
            (forw, dir)
        } else {
            if steps > 0 {
                route.push(steps.to_string());
                steps = 0;
            }
            if test(&map, right) {
                route.push("R".to_string());
                (pos, right_dir)
            } else if test(&map, left) {
                route.push("L".to_string());
                (pos, left_dir)
            } else {
                break;
            }
        };
        pos = new_pos;
        dir = new_dir;
    }

    println!("{}", route.iter().flat_map(|s| s.chars()).collect::<String>());
    /*
    A=L8R10L8R8
    B=L12R8R8
    A=L8R10L8R8
    C=L8R6R6R10L8
    C=L8R6R6R10L8
    A=L8R10L8R8
    B=L12R8R8
    C=L8R6R6R10L8
    B=L12R8R8
    B=L12R8R8
    */

    let commands = format!("{}\n{}\n{}\n{}\n{}\n",
                           "A,B,A,C,C,A,B,C,B,B",
                           "L,8,R,10,L,8,R,8",
                           "L,12,R,8,R,8",
                           "L,8,R,6,R,6,R,10,L,8",
                           "n");
    mem[0] = 2;
    let (_, _, _, dust) = run(&mut Program { mem, base: 0 }, &commands);

    dust
}

fn test(map: &Vec<Vec<bool>>, (x, y): Point) -> bool {
    let n = map.len() as i64;
    let m = map[0].len() as i64;
    inside((x, y), m, n) && map[y as usize][x as usize]
}

fn inside((x, y): Point, m: i64, n: i64) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

fn add((a, b): Point, (x, y): Point) -> Point {
    (a + x, b + y)
}

fn turn((dx, dy): Point, q: bool) -> Point {
    if q {
        (-dy, dx)
    } else {
        (dy, -dx)
    }
}

fn run(program: &mut Program, input: &String) -> (Vec<Vec<bool>>, Point, Point, i64) {
    let mut state = Run(0);

    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 0);
    let mut map = vec![vec![]];
    let mut dust: i64 = -1;

    let mut input_pos = 0;

    loop {
        state = match state {
            Run(ip) => {
                run_till_interrupt(program, ip)
            },
            Input(p, ip) => {
                let c = input.chars().skip(input_pos).next().unwrap();
                input_pos += 1;
                set(program, p, c as i64);
                Run(ip)
            },
            Output(val, ip) => {
                let c = char::from(val as u8);
                if val > 255 {
                    dust = val;
                } else if val == 10 {
                    map.push(vec![]);
                } else {
                    map.last_mut().unwrap().push(c != '.');
                    if c != '#' && c != '.' {
                        pos = (map.last().unwrap().len() as i64 - 1, map.len() as i64 - 1);
                        dir = match c {
                            '<' => (-1, 0),
                            '^' => (0, -1),
                            '>' => (1, 0),
                            'v' => (0, 1),
                            _ => dir,
                        }
                    }
                }
                Run(ip)
            }
            Halted => break,
        };
    }
    while let Some(r) = map.last_mut() {
        if r.is_empty() {
            map.pop();
        } else {
            break;
        }
    }
    (map, pos, dir, dust)
}

fn run_till_interrupt(d: &mut Program, mut p: usize) -> State {
    while let command = parse_command(&d.mem, p) {
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
    let file = File::open("files/17.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}