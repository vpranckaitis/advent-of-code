use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const W: i32 = 25;
const H: i32 = 6;

fn solve1() -> i32 {
    let input = read();
    let layers = parse_layers(&input);

    let min_zeros = layers.iter()
        .map(|(k, v)| {
            let n: i32 = v.iter().filter(|&x| *x == 0).count() as i32;
            (k, n)
        })
        .min_by_key(|(_, n)| *n)
        .unwrap()
        .0;

    let ones = layers.get(min_zeros).unwrap()
        .iter()
        .filter(|&x| *x == 1)
        .count();
    let twos = layers.get(min_zeros).unwrap()
        .iter()
        .filter(|&x| *x == 2)
        .count();
    (ones * twos) as i32
}

fn solve2() -> String {
    let input = read();
    let layers = parse_layers(&input);

    let mut img = vec![2; (W * H) as usize];

    let max_layer = layers.keys().max().unwrap();

    for i in (0..(max_layer + 1)).rev() {
        for (i, &v) in layers.get(&i).unwrap().iter().enumerate() {
            if v < 2 {
                img[i] = v;
            }
        }
    }

    let mut res = String::new();
    for j in 0..H {
        for i in 0..W {
            res.push(match img[(j * W + i) as usize] {
                0 => ' ',
                1 => '#',
                _ => ' ',
            });
        }
        res.push('\n');
    }
    res
}

fn parse_layers(xs: &Vec<i32>) -> HashMap<i32, Vec<i32>> {
    let mut layers: HashMap<i32, Vec<i32>> = HashMap::new();
    xs.iter()
        .enumerate()
        .map(|(i, v)| (i as i32 / (W * H), v))
        .for_each(|(i, &v)| layers.entry(i).or_insert(vec![]).push(v));
    layers
}

fn read() -> Vec<i32> {
    let file = File::open("files/08.in").expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect()
}