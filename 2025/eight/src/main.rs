use std::{collections::HashMap, hash::Hash};

use text_io::read;

fn dist(pt1: (i32, i32, i32), pt2: (i32, i32, i32)) -> f64 {
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
    let mut points: Vec<(i32, i32, i32)> = vec![];

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

    for i in 0..n {
        println!("{}. {:?}", i, points[i]);
    }
    for i in 0..10 {
        println!("{}. {}, {:?}", i, dists[i].0, dists[i].1);
    }

    let mut group_map: HashMap<usize, i32> = HashMap::new();
    let mut group_num = 0;
    let mut connection_counter = 0;
    let mut dist_idx: usize = 0;
    while connection_counter < 10 {
        // connect the junction boxes present at dist_idx
        let dist = dists[dist_idx];
        dist_idx += 1;

        if group_map.contains_key(&dist.1.0) && group_map.contains_key(&dist.1.1) {
            continue;
        } else if group_map.contains_key(&dist.1.0) {
            connection_counter += 1;
            group_map.insert(
                dist.1.1,
                // I am just fighting with the compiler, 
                // I barely understand what to_owned is doing under the hood
                // Ill eventually learn it ¯\_(ツ)_/¯
                group_map.get(&dist.1.0).unwrap().to_owned()
            );
        } else if group_map.contains_key(&dist.1.1) {
            connection_counter += 1;
            group_map.insert(
                dist.1.0,
                group_map.get(&dist.1.1).unwrap().to_owned()
            );
        } else {
            connection_counter += 1;
            group_map.insert(
                dist.1.0,
                group_num
            );
            group_map.insert(
                dist.1.1,
                group_num
            );
            group_num += 1;
        }
    }

    let mut reverse_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for vals in group_map {
        if let Some(lst) = reverse_map.get(&vals.1) {
            (*lst).push(vals.0)
        }
    }
}
