use text_io::read;

fn checker(num: i64) -> bool {
    let n_as_str = num.to_string();
    let n: i64 = n_as_str.len() as i64;
    for sst in 1..n {
        if n % sst != 0 {
            continue;
        }
        let mut accumulator: Vec<&str> = vec![];
        for mult in 0..(n/sst) {
            accumulator.push(&n_as_str[(mult as usize) * (sst as usize) .. (mult as usize + 1) * (sst as usize)]);
        }
        let mut flag = true;
        for idx in 1..accumulator.len() {
            if accumulator[idx-1] != accumulator[idx] {
                flag = false;
                break;
            }
        }
        if flag == true {
            return true;
        }
    }
    return false;
}

fn main() {
    let line: String = read!();
    let ranges: Vec<&str> = line.split(',').collect();
    
    let mut ans: i64 = 0;
    for range in ranges {
        let limits: Vec<&str> = range.split('-').collect();
        let lower: i64 = limits[0].parse().unwrap();
        let upper: i64 = limits[1].parse().unwrap();
        for n in lower..=upper {
            if checker(n) {
                println!("{}", n);
                ans += n
            }
        }
    }
    println!("ans = {}", ans);
}
