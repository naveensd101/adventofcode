use text_io::read;

fn moder_100(n: i32) -> i32 {
    match n {
        ..0 => moder_100(n + 100),
        0.. => n % 100
    }
}

fn main() {
    let n: i32 = read!();
    let mut ans: i32 = 0;
    let mut cur: i32 = 50;
    for _ in 0..n {
        let line: String = read!();
        let c: char = line.chars().next().unwrap();
        let num: i32 = (&line[1..]).to_string().parse().expect("wanted i32");

        match c {
            'L' => {
                cur -= num;
                cur = moder_100(cur);
            },
            'R' => {
                cur += num;
                cur = moder_100(cur);
            },
            _ => {
                panic!("Char not expected");
            }
        }

        // println!("cur = {cur}");
        if cur == 0 {
            ans += 1;
        }
    }

    println!("ans = {ans}");
}

