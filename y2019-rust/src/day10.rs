use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Ratio { p: i32, q:i32 }

impl Ratio {
    fn from(p: i32, q: i32) -> Ratio {
        let a = gcd(p.abs(), q.abs());
        Ratio { p: p / a, q: q / a }
    }
}

fn solve1() -> i32 {
    let g = read();
    let mut mx: i32 = 0;
    let n = g.len() as i32;
    let m = g[0].len() as i32;
    for y in 0..n {
        for x in 0..m {
            if g[y as usize][x as usize] {
                let mut r: HashSet<Ratio> = HashSet::new();
                for yy in 0..n {
                    for xx in 0..m {
                        if !(yy == y && xx == x) && g[yy as usize][xx as usize] {
                            r.insert(Ratio::from(y - yy, x - xx));
                        }
                    }
                }
                mx = max(mx, r.len() as i32);
            }
        }
    }
    mx
}

fn solve2() -> i32 {
    let g = read();

    let mut mx: i32 = 0;
    let n = g.len() as i32;
    let m = g[0].len() as i32;

    let mut ry = 0;
    let mut rx = 0;

    for y in 0..n {
        for x in 0..m {
            if g[y as usize][x as usize] {
                let mut r: HashSet<Ratio> = HashSet::new();
                for yy in 0..n {
                    for xx in 0..m {
                        if !(yy == y && xx == x) && g[yy as usize][xx as usize] {
                            r.insert(Ratio::from(y - yy, x - xx));
                        }
                    }
                }
                if r.len() as i32 > mx {
                    mx = r.len() as i32;
                    ry = y;
                    rx = x;
                }
            }
        }
    }

    let mut r: HashMap<Ratio, Vec<(i32, i32)>> = HashMap::new();
    for yy in 0..n {
        for xx in 0..m {
            if !(yy == ry && xx == rx) && g[yy as usize][xx as usize] {
                r.entry(Ratio::from(ry - yy, xx - rx)).or_insert(vec![]).push((xx - rx, ry - yy));
            }
        }
    }
    r.values_mut().for_each(|v| v.sort_by_cached_key(|(x, y)| {
        -(y.abs() + x.abs())
    }));

    let mut ord: Vec<Ratio> = vec![];
    r.keys().for_each(|k| ord.push(*k));
    ord.sort_by(|r1, r2| {
        let z1 = quarter(r1);
        let z2 = quarter(r2);
        if z1 != z2 {
            z1.cmp(&z2)
        } else {
            0.cmp(&sin_angle(r1, r2))
        }
    });

    let mut rem = 0;
    let mut i: usize = 0;
    let k = ord.len();
    loop {
        let v = r.get_mut(&ord[i]).unwrap();
        if !v.is_empty() {
            let d = v.pop().unwrap();
            rem += 1;
            if rem == 200 {
                return (d.0 + rx) * 100 + (ry - d.1);
            }
        }
        i = (i + 1) % k;
    }
}

fn sin_angle(r1: &Ratio, r2: &Ratio) -> i32 {
    r1.p * r2.q - r1.q * r2.p
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn quarter(r: &Ratio) -> i32 {
    let Ratio { p, q } = *r;
    if p > 0 && q >= 0 {
        1
    } else if q > 0 && p <= 0 {
        2
    } else if p < 0 && q <= 0 {
        3
    } else {
        4
    }
}

fn read() -> Vec<Vec<bool>> {
    let file = File::open("files/10.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| l.unwrap().chars().map(|c| c == '#').collect())
        .collect()
}