use std::collections::HashMap;

use crate::intcode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

type Point = (i64, i64);

const FILENAME: &str = "files/13.in";

fn solve1() -> i64 {
    let mut program = intcode::read(FILENAME);

    let mut out = vec![];
    let mut pos: HashMap<Point, i64> = HashMap::new();

    intcode::run(
        &mut program,
        &mut || 0,
        &mut |val| {
            out.push(val);
            match out[..] {
                [x, y, v] => {
                    pos.insert((x, y), v);
                    out.clear();
                },
                _ => (),
            };
        }
    );

    pos.values()
        .filter(|x| **x == 2)
        .count() as i64
}

fn solve2() -> i64 {
    let mut program = intcode::read(FILENAME);

    let mut out = vec![];
    let x_paddle: Rc<RefCell<i64>> = Rc::new(RefCell::new(0));
    let x_ball: Rc<RefCell<i64>> = Rc::new(RefCell::new(0));
    let mut score = -1;

    program.mem[0] = 2;

    intcode::run(
        &mut program,
        &mut || (*x_ball.borrow() - *x_paddle.borrow()).signum(),
        &mut |val| {
            out.push(val);
            match out[..] {
                [-1, _, s] => {
                    score = s;
                },
                [x, _, 3] => {
                    *x_paddle.borrow_mut() = x;
                },
                [x, _, 4] => {
                    *x_ball.borrow_mut() = x;
                },
                _ => { },
            };
            if out.len() == 3 {
                out.clear();
            }
        }
    );

    score
}