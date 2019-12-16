use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const T: i64 = 100;

fn solve1() -> String {
    let mut p = read();

    let pattern = [0, 1, 0, -1];

    for _ in 0..T {
        let mut new_p: Vec<i64> = vec![];
        for i in 0..p.len() {
            let mut val = 0;
            for j in 0..p.len() {
                let idx = ((j + 1) / (i + 1)) % pattern.len();
                val += p[j] * pattern[idx];
            }
            new_p.push(val.abs() % 10);
        }
        p = new_p;
    }
    p.iter()
        .take(8)
        .map(|x| x.to_string())
        .collect()
}

fn solve2() -> String {
    let mut p = read();

    let skip = p.iter()
        .take(7)
        .fold(0, |acc, x| acc * 10 + *x);

    const MOD: i64 = 10;

    println!();

    let pl = p.len();
    for _ in 1..10000 {
        for i in 0..pl {
            p.push(p[i]);
        }
    }

    let pattern = [0, 1, 0, -1];

    for _ in 0..T {
        let mut partial = vec![0];
        for i in 0..p.len() {
            partial.push(partial[i] + p[i]);
        }

        let mut minv: i64 = 1e9 as i64;
        let mut maxv: i64 = 0;

        let mut new_p: Vec<i64> = vec![];
        for i in 0..p.len() {
            let mut val: i64 = 0;
            let mut j = 1;
            for l in (i..partial.len()).step_by(i + 1) {
                let rr = min(l + i + 1, partial.len() - 1);
                let segment = partial[rr] - partial[l];
                val += pattern[j % 4] * segment;
                minv = min(minv, val);
                maxv = max(maxv, val);
                j += 1;
            }
            new_p.push(val.abs() % MOD);
        }
        p = new_p;
    }
    p.iter()
        .skip(skip as usize)
        .take(8)
        .map(|x| x.to_string())
        .collect()
}

fn read() -> Vec<i64> {
    let file = File::open("files/16.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|l| -> Vec<i64> {
            l.unwrap().chars().into_iter()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}