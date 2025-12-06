use text_io::read;
use std::cmp::max;

/*
This question was the most interesting by far in 2025's advent of code for me.
I forced myself to not brute force my way out of this.
It also made me read about segment trees, a data structure
that I avoided during my bachelor's.
 */

fn binary_search(num: i64, lst: &Vec<(i64, i64)>) -> bool {
    let mut l: usize = 0;
    let mut r: usize = lst.len() - 1;
    for idx in vec![l, r] {
        if num >= lst[idx].0 && num <= lst[idx].1 {
            return true;
        }
    }

    while r - l > 1 {
        let mid = (l + r) / 2;
        for idx in vec![l, r, mid] {
            if num >= lst[idx].0 && num <= lst[idx].1 {
                return true;
            }
        }
        if num >= lst[mid].0 {
            l = mid;
        } else {
            r = mid;
        }
    }
    false
}

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let mut ranges: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        let line: String = read!();
        let range: Vec<&str> = line.split('-').collect();
        let left: i64 = range[0].parse().unwrap();
        let right: i64 = range[1].parse().unwrap();

        ranges.push((left, right));
    }
    ranges.sort();

    let mut compressed: Vec<(i64, i64)> = vec![];
    compressed.push(ranges[0]);
    let mut idx: usize = 0;

    for i in 1..ranges.len() {
        if ranges[i].0 >= compressed[idx].0 && ranges[i].0 <= compressed[idx].1 {
            compressed[idx] = (compressed[idx].0, max(ranges[i].1, compressed[idx].1));
        } else {
            compressed.push(ranges[i]);
            idx += 1;
        }
    }

    println!("{:?}", &compressed);
    let mut ans: i32 = 0;
    for _ in 0..m {
        let num: i64 = read!();
        if binary_search(num, &compressed) {
            ans += 1;
            // println!("{}: true", num);
        } else {
            // println!("{}: false", num);
        }
    }
    println!("ans = {}", ans);

}