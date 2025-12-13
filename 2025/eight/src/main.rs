use std::collections::HashSet;

use text_io::read;

// unlike cpp rust gives me error for overflows, this is really handy
fn dist(pt1: (i64, i64, i64), pt2: (i64, i64, i64)) -> f64 {
    ((
        (pt1.0 - pt2.0) * (pt1.0 - pt2.0)
        +
        (pt1.1 - pt2.1) * (pt1.1 - pt2.1)
        +
        (pt1.2 - pt2.2) * (pt1.2 - pt2.2)
    ) as f64).sqrt()
}

fn main() {
    let n: usize = read!();
    let mut points: Vec<(i64, i64, i64)> = vec![];

    for _ in 0..n {
        let line: String = read!();
        let nums: Vec<&str> = line.split(',').collect();
        points.push((
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap(),
            nums[2].parse().unwrap(),
        ));
    }

    // list of tuple with (distance, (pt1 index, pt2 index))
    let mut dists: Vec<(f64, (usize, usize))> = vec![];
    for i in 0..n {
        for j in i+1..n {
            dists.push(
                (
                    dist(points[i], points[j]), 
                    (i, j)
                )
            )
        }
    }
    dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // for i in 0..n {
    //     println!("{}. {:?}", i, points[i]);
    // }
    // for i in 0..10 {
    //     println!("{}. {}, {:?}", i, dists[i].0, dists[i].1);
    // }

    // build adjacentcy matrix
    let mut adj: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for i in 0..1000 {
        adj[dists[i].1.0][dists[i].1.1] = true;
        adj[dists[i].1.1][dists[i].1.0] = true;
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut all_connected: Vec<HashSet<usize>> = vec![];
    for i in 0..n {
        if visited.contains(&i) {
            continue;
        }

        let mut current: Vec<usize> = vec![];
        current.push(i);
        let mut connected_component: HashSet<usize> = HashSet::new();
        while !current.is_empty() {
            let cur = current.pop().unwrap();
            if visited.contains(&cur) { continue; }
            visited.insert(cur);
            connected_component.insert(cur);
            for j in 0..n {
                if adj[cur][j] {
                    current.push(j);
                    connected_component.insert(j);
                }
            }
        }
        all_connected.push(connected_component);
    }

    println!("{:?}", all_connected);

    // (# of junction box, count of circuits with that count)
    let mut freq: Vec<usize> = vec![];
    for component in all_connected {
        freq.push(component.len());
    }

    freq.sort();
    freq.reverse();
    println!("{:?}", freq);
    let mut ans: usize = 1;
    for i in 0..3 {
        ans *= freq[i];
    }
    println!("ans = {ans}");
}
