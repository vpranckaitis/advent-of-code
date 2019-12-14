use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

fn solve1() -> i64 {
    let p = read();
    make("ORE", "FUEL", 1, &p)
}

fn solve2() -> i64 {
    let p = read();

    let mx: i64 = 1000000000000;

    let mut l: i64 = 1;
    let mut r: i64 = mx;

    while l + 1 < r {
        let m = (l + r) / 2;
        if make("ORE", "FUEL", m, &p) <= mx {
            l = m;
        } else {
            r = m;
        }
    }
    l
}

fn make(from: &str, to: &str, n_to: i64, recipes: &Vec<(Vec<(i64, String)>, (i64, String))>) -> i64 {
    let r: HashMap<String, (i64, Vec<_>)> = recipes.iter()
        .map(|(xs, (cnt, res))| (res.clone(), (*cnt, xs.clone())))
        .collect();
    let mut need: HashMap<String, i64> = HashMap::new();

    need.insert(to.to_string(), n_to, );

    loop {
        if let Some((next, n)) = need.iter().find(|&(s, x)| s != from && *x > 0) {
            let (cnt, rr) = r.get(next).unwrap();
            let k = n / cnt + if n % cnt == 0 { 0 } else { 1 };
            *need.entry(next.clone()).or_insert(0) -= k * cnt;
            for (n_used, used) in rr {
                *need.entry(used.clone()).or_insert(0) += k * n_used;
            }
        } else {
            break;
        }
    }
    *need.get(from).unwrap()
}

fn read() -> Vec<(Vec<(i64, String)>, (i64, String))> {
    let file = File::open("files/14.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|l| {
            let z: Vec<Vec<_>> = l.unwrap().split(" => ")
                .map(|p| {
                    let pp: Vec<(i64, String)> = p.split(", ")
                        .map(|q| {
                            let qq: Vec<_> = q.split(" ").collect();
                            (qq[0].parse::<i64>().unwrap(), qq[1].to_string())
                        })
                        .collect();
                    pp
                })
                .collect();
            (z[0].clone(), z[1][0].clone())
        })
        .collect()
}