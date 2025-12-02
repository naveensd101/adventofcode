use text_io::read;

fn checker(n: i64) -> bool {
    let str: String = n.to_string();
    if str.len() % 2 != 0 {
        return false;
    }
    let mut flag: bool = true;
    for idx in 0..(str.len() / 2) {
        if str.chars().nth(idx) != str.chars().nth(str.len() / 2 + idx) {
            flag = false;
            break;
        }
    }
    flag
}

fn main() {
    let line: String = read!();

    let ranges: Vec<&str> = line.split(',').collect();

    let mut ans: i64 = 0;
    for range in ranges {
        let limits: Vec<&str> = range.split('-').collect();
        let lower: i64 = limits[0].parse().expect("i64 wanted");
        let upper: i64 = limits[1].parse().expect("i64 wanted");
        for n in lower..=upper {
            if checker(n) {
                println!("val = {}", n);
                ans += n;
            }
        }
    }

    println!("ans = {}", ans);
}
