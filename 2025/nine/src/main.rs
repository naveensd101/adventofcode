use text_io::read;
use std::cmp::{max, min};

fn area(pt1: (i64, i64),  pt2: (i64, i64)) -> i64 {
    (max(pt1.0, pt2.0) - min(pt1.0, pt2.0) + 1)
    *
    (max(pt1.1, pt2.1) - min(pt1.1, pt2.1) + 1)
}

fn solution(n: usize, tiles: Vec<(i64, i64)>) -> i64 {
    let mut ans: i64 = 0;
    for i in 0..n {
        for j in i+1..n {
            ans = max(
                ans,
                area(tiles[i], tiles[j])
            )
        }
    }
    ans
}

fn main() {
    let n: usize = read!();
    let mut tiles: Vec<(i64, i64)> = vec![];

    for _ in 0..n {
        let line: String = read!();
        let nums: Vec<&str> = line.split(',').collect();
        tiles.push(
            (
                nums[0].parse().unwrap(),
                nums[1].parse().unwrap()
            )
        );
    }

    println!("ans = {}", solution(n, tiles))
}
