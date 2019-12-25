use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const FILENAME: &str = "files/22.in";

const N: i128 = 119315717514047;
const K: i128 = 101741582076661;

fn solve1() -> i32 {
    let q = read();

    let mut v: Vec<i32> = vec![];
    for i in 0..10007 {
        v.push(i);
    }

    v = shuffle(v, &q);

    v.iter()
        .enumerate()
        .filter(|(_, &x)| x == 2019)
        .map(|(i, _)| i)
        .next()
        .unwrap() as i32
}

fn solve2() -> i128 {
    let q = read();

    let (mut p, dir, step) = repeated_shuffle(&q, K);

    let mut v = vec![];
    for i in 0..2021 {
        v.push(p);
        p = (p + dir * step + N) % N;
    }

    v[2020]
}

fn shuffle(mut v: Vec<i32>, q: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut z: i64 = 1;
    for &(r, val) in q.iter() {
        if r == 0 {
            v.reverse();
        } else if r == 1 {
            let pos = if val >= 0 {
                val as usize % v.len()
            } else {
                (v.len() as i32 + (val % v.len() as i32)) as usize
            };
            let mut v1 = vec![];
            for i in pos..v.len() {
                v1.push(v[i]);
            }
            for i in 0..pos {
                v1.push(v[i]);
            }
            v = v1;
        } else {
            z *= val as i64;
            let mut v1 = vec![];
            v1.resize(v.len(), -1);
            let mut pos = 0;
            for &x in v.iter() {
                v1[pos] = x;
                pos = (pos + val as usize) % v.len();
            }
            v = v1;
        }
    }
    v
}

fn repeated_shuffle(q: &Vec<(i32, i32)>, k: i128) -> (i128, i128, i128) {
    if k == 0 {
        (0, 1, 1)
    } else if k % 2 == 0 {
        let (mut p, mut d, mut s) = repeated_shuffle(q, k / 2);
        p = (p * (1 + d * s + N)) % N;
        d = d * d;
        s = (s * s) % N;
        (p, d, s)
    } else {
        let (mut p, mut d, mut s) = repeated_shuffle(q, k - 1);
        shuffle_fast(q, &mut p, &mut d, &mut s);
        (p, d, s)
    }
}

fn shuffle_fast(q: &Vec<(i32, i32)>, p: &mut i128, dir: &mut i128, step: &mut i128) {
    for &(r, val32) in q.iter() {
        let val = val32 as i128;
        if r == 0 || r == 1 {
            let shift = (*dir * val * *step + 10000*N) % N;
            *p = (*p + shift + N) % N;
            if r == 0 {
                *dir *= -1;
            }
        } else {
            let mut i = 1;
            while (i * N + 1) % val != 0 {
                i += 1;
            }
            let step1 = (i * N + 1) / val;
            *step = (*step * step1) % N;
        }
    }
}

fn read() -> Vec<(i32, i32)> {
    let file = File::open(FILENAME).expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| {
            let ll = l.unwrap();
            let v: Vec<_> = ll.split(" ").collect();
            if v[0] == "deal".to_string() {
                if v[1] == "into".to_string() {
                    (0, -1)
                } else {
                    (2, v[3].parse::<i32>().unwrap())
                }
            } else {
                (1, v[1].parse::<i32>().unwrap())
            }
        })
        .collect()
}