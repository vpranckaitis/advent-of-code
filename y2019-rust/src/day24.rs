use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const FILENAME: &str = "files/24.in";

type Point = (i32, i32);

fn solve1() -> i32 {
    let mut a = read();

    let mut prev = HashSet::new();

    loop {
        if prev.contains(&a) {
            return a;
        }
        prev.insert(a);
        a = next(a);
    }
}

fn solve2() -> i32 {
    let a = read();

    let mut q = VecDeque::from(vec![0, 0, a, 0, 0]);

    for _ in 0..200 {
        q = next_recursive(q);
    }

    let mut res: i32 = 0;
    for a in q.iter() {
        res += a.count_ones() as i32;
    }

    res
}

fn next_recursive(q: VecDeque<i32>) -> VecDeque<i32> {
    let mut r: VecDeque<i32> = VecDeque::new();
    for t in 1..q.len() - 1 {
        let a = q[t - 1];
        let b = q[t];
        let c = q[t + 1];

        let mut cnt = count_adjacent(b);
        for i in 0..5 {
            cnt[get_bit((2, 1)) as usize] += if get(c, (i, 0)) { 1 } else { 0 };
            cnt[get_bit((2, 3)) as usize] += if get(c, (i, 4)) { 1 } else { 0 };

            cnt[get_bit((i, 0)) as usize] += if get(a, (2, 1)) { 1 } else { 0 };
            cnt[get_bit((i, 4)) as usize] += if get(a, (2, 3)) { 1 } else { 0 };
        }

        for j in 0..5 {
            cnt[get_bit((1, 2)) as usize] += if get(c, (0, j)) { 1 } else { 0 };
            cnt[get_bit((3, 2)) as usize] += if get(c, (4, j)) { 1 } else { 0 };

            cnt[get_bit((0, j)) as usize] += if get(a, (1, 2)) { 1 } else { 0 };
            cnt[get_bit((4, j)) as usize] += if get(a, (3, 2)) { 1 } else { 0 };
        }

        let mut d = b;
        for k in 0..25 {
            let cur = (b & 1 << k) > 0;
            let cnt_k = cnt[k as usize];
            if (cur && cnt_k != 1) || k == 12 {
                d &= !(1 << k);
            } else if !cur && (cnt_k == 1 || cnt_k == 2) {
                d |= 1 << k;
            }
        }
        r.push_back(d);
    }

    for i in 0..2 {
        if r[i] != 0 {
            r.push_front(0);
        }
        if r[r.len() - 1 - i] != 0 {
            r.push_back(0);
        }
    }

    r
}

fn next(a: i32) -> i32 {
    let cnt = count_adjacent(a);

    let mut r = a;
    for k in 0..25 {
        let cur = (a & 1 << k) > 0;
        let cnt_k = cnt[k as usize];
        if cur && cnt_k != 1 {
            r &= !(1 << k);
        } else if !cur && (cnt_k == 1 || cnt_k == 2) {
            r |= 1 << k;
        }
    }

    r
}

fn count_adjacent(q: i32) -> Vec<i32> {
    const DIR: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut cnt = Vec::new();
    cnt.resize(25, 0);
    for i in 0..5 {
        for j in 0..5 {
            let k = get_bit((i, j));
            for &(di, dj) in DIR.iter() {
                if get(q, (i + di, j + dj)) {
                    cnt[k as usize] += 1;
                }
            }
        }
    }
    cnt
}

fn get(q: i32, p: Point) -> bool {
    let (i, j) = p;
    if i < 0 || i >= 5 || j < 0 || j >= 5 {
        false
    } else {
        q & (1 << get_bit(p)) > 0
    }
}

fn get_bit((i, j): Point) -> i32 {
    i * 5 + j
}

fn read() -> i32 {
    let file = File::open(FILENAME).expect("No file");
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .enumerate()
        .map(|(i, c)| if c == '#' { 1 << i } else { 0 })
        .sum()
}