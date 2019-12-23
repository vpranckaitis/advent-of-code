use std::collections::VecDeque;

use crate::intcode::{ self, Program };

const FILENAME: &str = "files/23.in";

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let program = intcode::read(FILENAME);

    const N: usize = 50;

    let mut inputs: Vec<VecDeque<i64>> = vec![];
    let mut outputs: Vec<Vec<i64>> = vec![];
    inputs.resize(N, VecDeque::new());
    outputs.resize(N, vec![]);

    for i in 0..N {
        inputs[i].push_back(i as i64);
    }

    let mut programs: Vec<Program> = vec![];
    programs.resize(N, program);

    let mut res = -1;

    let mut k: usize = 0;
    while res == -1 {

        intcode::run_till_io(
            &mut programs[k],
            &mut || inputs[k].pop_front().unwrap_or(-1),
            &mut |val| outputs[k].push(val)
        );

        if let [d64, x, y] = outputs[k][..] {
            let d = d64 as usize;
            if d == 255 {
                res = y;
            } else {
                inputs[d].push_back(x);
                inputs[d].push_back(y);
            }
            outputs[k].clear();
        }
        k = (k + 1) % N;
    }

    res
}

fn solve2() -> i64 {
    let program = intcode::read(FILENAME);

    const N: usize = 50;

    let mut inputs: Vec<VecDeque<i64>> = vec![];
    let mut outputs: Vec<Vec<i64>> = vec![];
    inputs.resize(N, VecDeque::new());
    outputs.resize(N, vec![]);

    for i in 0..N {
        inputs[i].push_back(i as i64);
    }

    let mut programs: Vec<Program> = vec![];
    programs.resize(N, program);

    let mut nat_x = -1;
    let mut nat_y = -1;

    let mut last_idle_y = 0;

    let mut res = -1;

    let mut k: usize = 0;
    let mut idle_rounds = 0;
    while res == -1 {

        intcode::run_till_io(
            &mut programs[k],
            &mut || inputs[k].pop_front().unwrap_or(-1),
            &mut |val| outputs[k].push(val)
        );

        if let [d64, x, y] = outputs[k][..] {
            let d = d64 as usize;
            if d == 255 {
                nat_x = x;
                nat_y = y;
            } else {
                inputs[d].push_back(x);
                inputs[d].push_back(y);
            }
            outputs[k].clear();
        }

        k = (k + 1) % N;
        if k == 0 && inputs.iter().all(|v| v.is_empty()) {
            idle_rounds += 1;
            if idle_rounds == 10 {
                idle_rounds = 0;
                if last_idle_y == nat_y {
                    res = nat_y;
                } else {
                    inputs[0].push_back(nat_x);
                    inputs[0].push_back(nat_y);
                    last_idle_y = nat_y;
                }
            }
        }
    }

    res
}