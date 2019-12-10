pub fn solve() {
    let (l, r) = parse_input();
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for i in l..(r + 1) {
        let mut mn = 10;
        let mut a = i;
        let mut good = true;
        let mut repeats = [0; 10];
        while good && a > 0 {
            let b = a % 10;
            a /= 10;
            good = b <= mn;
            mn = b;
            repeats[b as usize] += 1;
        }
        if good {
            if repeats.iter().max().unwrap_or(&0) >= &2 {
                cnt1 += 1;
            }
            if repeats.iter().any(|x| x == &2) {
                cnt2 += 1;
            }
        }
    }
    println!("{}\n{}", cnt1, cnt2)
}

fn parse_input() -> (i32, i32) {
    (172851, 675869)
}