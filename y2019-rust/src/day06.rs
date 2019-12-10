use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

type Graph = HashMap<String, Vec<String>>;

fn solve1() -> i32 {
    let input = read().expect("Wrong input");
    let mut g: Graph = HashMap::new();
    for (u, v) in input {
        if !g.contains_key(&u) {
            g.insert(u.clone(), vec![]);
        }
        g.get_mut(&u).unwrap().push(v);
    }
    dfs1(&g, &String::from("COM"), 0)
}

fn solve2() -> i32 {
    let input = read().expect("Wrong input");
    let mut g: Graph = HashMap::new();
    for (u, v) in input {
        if !g.contains_key(&u) {
            g.insert(u.clone(), vec![]);
        }
        if !g.contains_key(&v) {
            g.insert(v.clone(), vec![]);
        }
        g.get_mut(&u).unwrap().push(v.clone());
        g.get_mut(&v).unwrap().push(u.clone());
    }
    dfs2(&g, &String::from("YOU"), &String::from(""), &String::from("SAN"), 0).unwrap() - 2
}

fn dfs1(g: &Graph, u: &String, d: i32) -> i32 {
    let traverse: i32 = g.get(u).unwrap_or(&vec![]).iter()
        .map(|v| dfs1(g, v, d + 1))
        .sum();
    traverse + d
}

fn dfs2(g: &Graph, u: &String, par: &String, target: &String, d: i32) -> Option<i32> {
    if *u == *target {
        Some(d)
    } else {
        g.get(u).unwrap_or(&vec![]).iter()
            .filter(|v| *v != par)
            .flat_map(|v| dfs2(g, v, u, target, d + 1))
            .next()
    }
}

fn read() -> Result<Vec<(String, String)>, String> {
    let file = File::open("files/06.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .collect::<Result<Vec<String>, _>>()
        .map_err(|_| String::from("Failed reading file"))?
        .iter()
        .map(|l| {
            let split: Vec<_> = l.split(')').collect();
            match split[..] {
                [u, v] => Ok((String::from(u), String::from(v))),
                _ => Err(String::from("Unexpected line")),
            }
        })
        .collect()
}