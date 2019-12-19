use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const FILENAME: &str = "files/18.in";
type Point = (i32, i32);
type Edge = (char, i32);

fn solve1() -> i32 {
    let (g, p) = read();

    let g2 = adjacency_list(&g, &p);

    shortest_path(vec!['@'], &g2)
}

fn solve2() -> i32 {
    let (mut g, mut p) = read();

    let at = p.iter()
        .filter(|(_, &c)| c == '@')
        .map(|(&p, _)| p)
        .next()
        .unwrap();
    p.remove(&at);

    let mut markers = vec!['1', '2', '3', '4'];
    for i in -1..2 {
        for j in -1..2 {
            let pos = (at.0 + i, at.1 + j);
            if i.abs() + j.abs() == 2 {
                p.insert(pos, markers.pop().unwrap());
            } else {
                g.insert(pos, false);
            }
        }
    }

    let g2 = adjacency_list(&g, &p);

    shortest_path(vec!['1', '2', '3', '4'], &g2)
}

const INF: i32 = 1e9 as i32;

fn shortest_path(start: Vec<char>, g: &HashMap<char, Vec<Edge>>) -> i32 {
    type State = (Vec<char>, i32);
    let mut dist: HashMap<State, i32> = HashMap::new();
    dist.insert((start.clone(), 0), 0);
    let mut q: BinaryHeap<Reverse<(i32, State)>> = BinaryHeap::new();
    q.push(Reverse((0, (start, 0))));

    while let Some(Reverse((d, state))) = q.pop() {
        if d <= *dist.get(&state).unwrap_or(&INF) {
            let (u, z) = state;
            for (i, u1) in u.iter().enumerate() {
                for &(v, c) in g.get(&u1).unwrap() {
                    if !v.is_alphabetic() || v.is_lowercase() || z & get_bit(v) > 0 {
                        let new_d = d + c;
                        let new_z = if v.is_lowercase() { z | get_bit(v) } else { z };
                        let mut new_pos = u.clone();
                        new_pos[i] = v;
                        let new_state = (new_pos, new_z);
                        if new_d < *dist.entry(new_state.clone()).or_insert(INF) {
                            dist.insert(new_state.clone(), new_d);
                            q.push(Reverse((new_d, new_state)));
                        }
                    }
                }
            }
        }
    }

    let mut end_z = 0;
    for &c in g.keys() {
        end_z |= get_bit(c);
    }

    dist.iter()
        .filter(|(&(_, z), _)| end_z == z)
        .map(|(_, &v)| v)
        .min()
        .unwrap_or(INF)
}

fn get_bit(c: char) -> i32 {
    if !c.is_alphabetic() {
        0
    } else {
        1 << (to_int(c.to_lowercase().next().unwrap()) - to_int('a'))
    }
}

fn to_int(c: char) -> i32 {
    i32::from(c.to_string().bytes().next().unwrap())
}

fn adjacency_list(g: &HashMap<Point, bool>, p: &HashMap<Point, char>) -> HashMap<char, Vec<Edge>> {
    let mut g2: HashMap<char, Vec<Edge>> = HashMap::new();
    for (&pos, &c) in p.iter() {
        let mut dist: HashMap<Point, i32> = HashMap::new();
        bfs(pos, &g, &p, &mut dist);
        for (&pos1, &c1) in p.iter() {
            if c != c1 {
                if let Some(&d) = dist.get(&pos1) {
                    g2.entry(c).or_default().push((c1, d));
                }
            }
        }
    }
    g2
}

fn bfs(start: Point, g: &HashMap<Point, bool>, p: &HashMap<Point, char>,
       dist: &mut HashMap<Point, i32>) {
    let mut q: VecDeque<Point> = VecDeque::new();
    q.push_back(start);
    dist.insert(start, 0);

    const DIRS: [Point; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some(u) = q.pop_front() {
        if u == start || !p.contains_key(&u) {
            let du = *dist.get(&u).unwrap();
            for &dir in DIRS.iter() {
                let v = add(u, dir);
                if *g.get(&v).unwrap() && !dist.contains_key(&v) {
                    q.push_back(v);
                    dist.insert(v, du + 1);
                }
            }
        }
    }
}

fn add((x1, y1): Point, (x2, y2): Point) -> Point {
    (x1 + x2, y1 + y2)
}

fn read() -> (HashMap<Point, bool>, HashMap<Point, char>) {
    let file = File::open(FILENAME).expect("No file");
    let reader = BufReader::new(file);

    let chars: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let mut g: HashMap<Point, bool> = HashMap::new();
    let mut p: HashMap<Point, char> = HashMap::new();
    for (y, v) in chars.iter().enumerate() {
        for (x, &c) in v.iter().enumerate() {
            let pos = (x as i32, y as i32);
            if c != '#' && c != '.' {
                p.insert(pos, c);
            }
            g.insert(pos, c != '#');
        }
    }

    (g, p)
}