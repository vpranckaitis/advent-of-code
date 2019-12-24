use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::intcode::{self, Program};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

type Point = (i32, i32);

const FILENAME: &str = "files/11.in";

fn solve1() -> i32 {
    let mut program = intcode::read(FILENAME);
    get_colors(&mut program, 0).len() as i32
}

fn solve2() -> String {
    let mut program = intcode::read(FILENAME);
    let colors = get_colors(&mut program, 1);

    let minx = colors.keys().map(|p| p.0).min().unwrap();
    let maxx = colors.keys().map(|p| p.0).max().unwrap();
    let miny = colors.keys().map(|p| p.1).min().unwrap();
    let maxy = colors.keys().map(|p| p.1).max().unwrap();

    let mut res = String::new();

    for y in (miny..(maxy + 1)).rev() {
        for x in minx..(maxx + 1) {
            let color = *colors.get(&(x, y)).unwrap_or(&0);
            res.push(if color == 1 { '#' } else { '.' });
        }
        res.push('\n');
    }

    res
}

fn turn(dir: Point, z: i64) -> Point {
    let (dx, dy) = dir;
    if z == 0 {
        (-dy, dx)
    } else {
        (dy, -dx)
    }
}

fn add((x1, y1): Point, (x2, y2): Point) -> Point {
    (x1 + x2, y1 + y2)
}

fn get_colors(program: &mut Program, initial: i64) -> HashMap<Point, i64> {
    let pos = Rc::new(RefCell::new((0, 0)));
    let mut dir = (0, 1);
    let colors: Rc<RefCell<HashMap<Point, i64>>> = Rc::new(RefCell::new(HashMap::new()));
    (*colors.borrow_mut()).insert(*pos.borrow(), initial);

    let mut is_painting = true;

    intcode::run(
        program,
        &mut || *(*colors.borrow()).get(&*pos.borrow()).unwrap_or(&0),
        &mut |val| {
            if is_painting {
                (*colors.borrow_mut()).insert(*pos.borrow(), val);
            } else {
                dir = turn(dir, val);
                let new_pos = add(*pos.borrow(), dir);
                *pos.borrow_mut() = new_pos;
            }
            is_painting = !is_painting;
        }
    );

    (*colors).replace(HashMap::new())
}