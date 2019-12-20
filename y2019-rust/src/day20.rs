use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    println!("{}\n{}", solve1(), solve2());
}

const FILENAME: &str = "files/20.in";
type State = (usize, i32);

fn solve1() -> i32 {
    let q = read();

    let (g, start, finish) = build_graph(&q, 0);

    bfs((start, 0), (finish, 0), &g)
}

fn solve2() -> i32 {
    let q = read();

    let (g, start, finish) = build_graph(&q, 1);

    bfs((start, 0), (finish, 0), &g)
}

fn build_graph(q: &Vec<Vec<char>>, depth: i32) -> (Vec<Vec<State>>, usize, usize) {
    let dirs = [(-1, 0), (1 , 0), (0, -1), (0, 1)];

    let n = q.len();
    let m = q[0].len();

    let mut g: Vec<Vec<State>> = vec![];
    g.resize(n * m, vec![]);

    let mut h: HashMap<String, Vec<State>> = HashMap::new();

    for y in 1..(n - 1) {
        for x in 1..(m - 1) {
            let u = y * m + x;
            for &(dx, dy) in dirs.iter() {
                let x1 = ((x as i32) + dx) as usize;
                let y1 = ((y as i32) + dy) as usize;

                if q[y][x] == '.' && q[y1][x1] == '.' {
                    g[u].push((y1 * m + x1, 0));
                }

                let x2 = ((x as i32) - dx) as usize;
                let y2 = ((y as i32) - dy) as usize;
                if q[y][x].is_alphabetic() && q[y1][x1].is_alphabetic() && q[y2][x2] == '.' {
                    let hh = if x == 1 || x == m - 2 || y == 1 || y == n - 2 {
                        depth
                    } else {
                        -depth
                    };
                    let s = format!("{}{}", q[min(y, y1)][min(x, x1)], q[max(y, y1)][max(x, x1)]);
                    h.entry(s).or_default().push((y2 * m + x2, hh));
                }
            }
        }
    }

    for r in h.values() {
        if let [stu, stv] = r[..] {
            g[stu.0].push(stv);
            g[stv.0].push(stu);
        }
    }

    (g, h.get("AA").unwrap()[0].0, h.get("ZZ").unwrap()[0].0)
}


fn bfs(start: State, end: State, g: &Vec<Vec<State>>) -> i32 {
    const INF: i32 = 1e9 as i32;

    let mut q: VecDeque<State> = VecDeque::new();
    q.push_back(start);
    let mut dist: HashMap<State, i32> = HashMap::new();
    dist.insert(start, 0);

    while let Some(st) = q.pop_front() {
        if st == end {
            break;
        }
        let (u, h) = st;
        let d = *dist.get(&st).unwrap();
        for &(v, hh) in g[u].iter() {
            let h1 = h + hh;
            let st1 = (v, h1);
            if h1 >= 0 && *dist.entry(st1).or_insert(INF) == INF {
                q.push_back(st1);
                dist.insert(st1, d + 1);
            }
        }
    }

    *dist.get(&end).unwrap_or(&INF)
}


fn read() -> Vec<Vec<char>> {
    let file = File::open(FILENAME).expect("No file");
    let reader = BufReader::new(file);

    let mut chars: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let len = chars.iter()
        .map(|v| v.len())
        .max()
        .unwrap();

    for v in chars.iter_mut() {
        v.resize(len, ' ');
    }

    chars
}