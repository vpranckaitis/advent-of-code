use std::collections::{HashMap, HashSet};

use crate::intcode::{ self, Program };
use std::rc::Rc;
use std::cell::RefCell;

type Point = (i64, i64);

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const FILENAME: &str = "files/15.in";

fn solve1() -> i64 {
    let mut program = intcode::read(FILENAME);
    let (tank_pos, dist) = traverse_map(&mut program);
    *dist.get(&tank_pos).unwrap()
}

fn solve2() -> i64 {
    let mut program = intcode::read(FILENAME);
    let (tank_pos, dist) = traverse_map(&mut program);
    let mut unreached_positions: HashSet<Point> = dist.iter()
        .map(|(k, _)| *k)
        .collect();
    let mut fill = vec![tank_pos];
    let mut k = 0;
    while !unreached_positions.is_empty() {
        let mut newly_filled: Vec<Point> = vec![];
        for p in fill {
            for dir in 1..5 {
                let adjacent = step(p, dir);
                if unreached_positions.remove(&adjacent) {
                    newly_filled.push(adjacent);
                }
            }
        }
        fill = newly_filled;
        k += 1;
    }
    k
}

fn step((x, y): Point, dir: i64) -> Point {
    match dir {
        1 => (x, y + 1),
        2 => (x, y - 1),
        3 => (x - 1, y),
        4 => (x + 1, y),
        _ => panic!(),
    }
}

fn opposite_dir(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!(),
    }
}

fn traverse_map(program: &mut Program) -> (Point, HashMap<Point, i64>) {
    let mut pos: Point = (0, 0);
    let mut tank_pos: Point = (0, 0);

    let mut dist: HashMap<Point, i64> = HashMap::new();
    dist.insert(pos, 0);

    let dir = Rc::new(RefCell::new(vec![0 as i64]));
    let previous = Rc::new(RefCell::new(vec![0 as i64]));

    let last_dir: Rc<RefCell<i64>> = Rc::new(RefCell::new(0));

    let backtrack = Rc::new(RefCell::new(false));

    intcode::run(
        program,
        &mut || {
            if let (Some(d), Some(b)) = ((*dir).borrow_mut().last_mut(), (*previous).borrow().last()) {
                *d += 1;
                while *d == *b {
                    *d += 1;
                }
                *last_dir.borrow_mut() = if *d == 5 {
                    *backtrack.borrow_mut() = true;
                    *b
                } else {
                    *backtrack.borrow_mut() = false;
                    *d
                };
            }
            *last_dir.borrow()
        },
        &mut |val| {
            if val != 0 {
                let new_pos = step(pos, *last_dir.borrow());
                let new_dist = dist.get(&pos).unwrap() + 1;
                if *backtrack.borrow() {
                    *backtrack.borrow_mut() = false;
                    (*dir.borrow_mut()).pop();
                    (*previous.borrow_mut()).pop();
                } else if dist.get(&new_pos) == None || *dist.get(&new_pos).unwrap() > new_dist {
                    dist.insert(new_pos, new_dist);
                    (*dir.borrow_mut()).push(0);
                    (*previous.borrow_mut()).push(opposite_dir(*last_dir.borrow()));
                } else {
                    (*dir.borrow_mut()).push(4);
                    (*previous.borrow_mut()).push(opposite_dir(*last_dir.borrow()));
                }
                pos = new_pos;
                if val == 2 {
                    tank_pos = new_pos;
                }
            }
        }
    );

    (tank_pos, dist)
}