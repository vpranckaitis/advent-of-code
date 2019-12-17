use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i32 {
    let mut p = read();
    let l = p.len();
    p.resize(2 * l, 0);

    for _ in 0..1000 {
        simulate_step(&mut p, l);
    }

    energy(&p[0..l], &p[l..(2*l)])
}

fn solve2() -> i64 {
    let mut p = read();
    let l = p.len();
    p.resize(2 * l, 0);

    let mut cyc: Vec<i64> = p.iter().map(|_| 0).collect();
    let mut hist: Vec<Vec<i32>> = p.iter().map(|_| vec![]).collect();

    while cyc.iter().any(|x| *x == 0) {
        for j in 0..cyc.len() {
            if cyc[j] == 0 {
                hist[j].push(p[j]);
                if hist[j].len() % 2 == 0 {
                    let d = hist[j].len() / 2;
                    let mut good = true;
                    let mut k = 0;
                    while good && k < d {
                        good = hist[j][k] == hist[j][k + d];
                        k += 1;
                    }
                    if good {
                        cyc[j] = d as i64;
                    }
                }
            }
        }
        simulate_step(&mut p, l);
    }

    cyc.iter().fold(1, |acc, &x| (acc / gcd(acc, x)) * x)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn simulate_step(p: &mut Vec<i32>, l: usize) {
    for i in 0..l {
        for j in ((i % 3)..l).step_by(3) {
            p[i + l] += (p[j] - p[i]).signum();
        }
    }

    for i in 0..l {
        p[i] += p[i + l];
    }
}

fn energy(p: &[i32], v: &[i32]) -> i32 {
    let mut res: i32 = 0;
    for i in (0..p.len()).step_by(3) {
        let calc = |xs: &[i32]| -> i32 {
            xs.iter()
                .skip(i)
                .take(3)
                .map(|x| x.abs())
                .sum()
        };
        let a: i32 = calc(p);
        let b: i32 = calc(v);
        res += a * b;
    }
    res
}

fn read() -> Vec<i32> {
    let file = File::open("files/12.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|l| {
            let s = l.unwrap();
            let s = &s[1..(s.len() - 1)];
            s.split(", ")
                .map(|x| x[2..].parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}