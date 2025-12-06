use text_io::read;
use std::cmp::max;

/*
The part two of day 5 really felt like a step down from the original part.
 */
fn main() {
    let n: usize = read!();
    
    let mut ranges: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let line: String = read!();
        let range: Vec<&str> = line.split('-').collect();
        ranges.push(
            (
                range[0].parse().unwrap(),
                range[1].parse().unwrap()
            )
        );
    }

    ranges.sort();
    println!("sorted range = {:?}", &ranges);

    let mut compressed: Vec<(i64, i64)> = vec![];
    compressed.push(ranges[0]);
    let mut idx: usize = 0;

    for i in 1..ranges.len() {
        if ranges[i].0 >= compressed[idx].0 && ranges[i].0 <= compressed[idx].1 {
            compressed[idx] = (
                compressed[idx].0,
                max(
                    ranges[i].1,
                    compressed[idx].1
                )
            )
        } else {
            compressed.push(ranges[i]);
            idx += 1;
        }
    }

    println!("{:?}", compressed);
    let mut ans: i64 = 0;
    for val in compressed {
        ans += val.1 - val.0 + 1;
    }
    println!("ans = {}", ans);
}
