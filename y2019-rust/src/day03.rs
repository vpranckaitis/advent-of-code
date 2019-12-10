use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Dir { dx: i32, dy: i32, len: i32 }

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

pub fn solve1() -> i32 {
    let input = read().expect("Wrong input");
    let mut pos: Vec<HashSet<_>> = input
        .iter()
        .map(|ds| {
            let mut s: HashSet<(i32, i32)> = HashSet::new();
            let mut pos = (0, 0);
            ds.iter().for_each(|d| {
                for _ in 0..d.len {
                    pos = (pos.0 + d.dx, pos.1 + d.dy);
                    s.insert(pos);
                }
            });
            s
        })
        .collect();
    let pos1 = pos.remove(0);
    let pos2 = pos.remove(0);

    pos1.intersection(&pos2)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap_or(-1)
}

fn solve2() -> i32 {
    let input = read().expect("Wrong input");
    let mut pos: Vec<HashMap<_, _>> = input
        .iter()
        .map(|ds| {
            let mut s: HashMap<(i32, i32), i32> = HashMap::new();
            let mut pos = (0, 0);
            let mut steps = 0;
            ds.iter().for_each(|d| {
                for _ in 0..d.len {
                    pos = (pos.0 + d.dx, pos.1 + d.dy);
                    steps += 1;
                    if !s.contains_key(&pos) {
                        s.insert(pos, steps);
                    }
                }
            });
            s
        })
        .collect();

    let steps1 = pos.remove(0);
    let steps2 = pos.remove(0);

    let pos1 = steps1.keys()
        .collect::<HashSet<_>>();
    let pos2 = steps2.keys()
        .collect::<HashSet<_>>();

    pos1.intersection(&pos2)
        .map(|p| steps1.get(p).unwrap() + steps2.get(p).unwrap())
        .min()
        .unwrap_or(-1)
}


fn read() -> Result<Vec<Vec<Dir>>, String> {
    let file = File::open("files/03.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .take(2)
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| "Failed reading file")?
        .iter()
        .map(|l| parse_dirs(l))
        .collect()
}

fn parse_dirs(l: &String) -> Result<Vec<Dir>, String> {
    l.split(',')
        .map(|s| -> Result<_, String> {
            let len = s.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .map_err(|_| "Failed parsing length")?;
            match s.chars().next() {
                Some('R') => Ok(Dir { dx: 1, dy: 0, len }),
                Some('L') => Ok(Dir { dx: -1, dy: 0, len }),
                Some('U') => Ok(Dir { dx: 0, dy: 1, len }),
                Some('D') => Ok(Dir { dx: 0, dy: -1, len }),
                _ => Err("Failed parsing direction".to_string())
            }
        })
        .collect()
}