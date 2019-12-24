use std::collections::VecDeque;

use crate::intcode::{self, Program};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

type Point = (i64, i64);

const FILENAME: &str = "files/17.in";

fn solve1() -> i64 {
    let mut program = intcode::read(FILENAME);

    let (map, _, _) = get_map(&mut program);

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
    let mut program = intcode::read(FILENAME);

    let (map, mut pos, mut dir) = get_map(&mut program.clone());

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

    let mut input: VecDeque<_> = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        "A,B,A,C,C,A,B,C,B,B",
        "L,8,R,10,L,8,R,8",
        "L,12,R,8,R,8",
        "L,8,R,6,R,6,R,10,L,8",
        "n"
    ).bytes()
        .map(|b| b as i64)
        .collect();

    let mut res = -1;
    program.mem[0] = 2;

    intcode::run(
        &mut program,
        &mut || input.pop_front().unwrap(),
        &mut |val| {
            if val > 255 {
                res = val;
            } else {
                print!("{}", char::from(val as u8));
            }
        }
    );

    res
}

fn get_map(program: &mut Program) -> (Vec<Vec<bool>>, Point, Point) {
    let mut pos: Point = (0, 0);
    let mut dir: Point = (0, 0);
    let mut map = vec![vec![]];

    intcode::run(
        program,
        &mut || 0,
        &mut |val| {
            if val == 10 {
                map.push(vec![]);
            } else {
                let c = char::from(val as u8);
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
        }
    );


    while map.last().map(|r| r.is_empty()).unwrap_or(false) {
        map.pop();
    }

    (map, pos, dir)
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