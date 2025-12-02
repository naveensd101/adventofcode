use text_io::read;

// Go ahead judge me
fn calculator(cur: i32, parity: char, value: i32) -> (i32, i32) {
    let mut ans: i32 = 0;
    let mut next = cur;
    for _ in  0..value {
        if parity == 'L' {
            next -= 1;
        }
        if parity == 'R' {
            next += 1;
        }
        if next == 100 {
            next = 0;
        }
        if next == -1 {
            next = 99
        }
        if next == 0 {
            ans += 1;
        }
    }
    println!("{}: {}", next, ans);
    (next, ans)
}
fn main() {
    let n: i32 = read!();
    let mut cur: i32 = 50;
    let mut ans: i32 = 0;
    for _ in 0..n {
        let line: String = read!();
        let parity: char = line.chars().next().unwrap();
        let value: i32 = (&line[1..]).to_string().parse().expect("wanted i32");

        let tmp: (i32, i32)= calculator(cur, parity, value);
        cur = tmp.0;
        ans += tmp.1;
    }
    println!("ANS = {}", ans);
}
