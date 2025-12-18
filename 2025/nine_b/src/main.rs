use text_io::read;
use std::cmp::{max, min};

fn area(pt1: (i64, i64),  pt2: (i64, i64)) -> i64 {
    (max(pt1.0, pt2.0) - min(pt1.0, pt2.0) + 1)
    *
    (max(pt1.1, pt2.1) - min(pt1.1, pt2.1) + 1)
}
// the input is already in a hull edge list format
// right ray touches odd points in polygon if inside
fn is_ray_start_inside(ray_start: (i64, i64), n: usize, polygon: &Vec<(i64, i64)>) -> bool {
    for i in 0..n {
        let x1 = polygon[i].0;
        let y1 = polygon[i].1;
        let x2 = polygon[(i+1)%n].0;
        let y2 = polygon[(i+1)%n].1;
        let alpah = ray_start.0;
        let beta = ray_start.1;

        // if point is on the line then reutnr true
        if x1 == x2 && x1 == alpah && beta <= max(y1, y2) && beta >= min(y1, y2) {
            return true;
        } else if y1 == y2 && y1 == beta && alpah <= max(x1, x2) && alpah >= min(x1, x2) {
            return true;
        }

        // if x1 == x2 && x1 >= alpah && beta >= min(y1, y2) && beta <= max(y1, y2) {
        //     count += 1;
        // } else {
        //     continue;
        // }
    }
    // count % 2 != 0
    false
}

fn main() {
    let n: usize = read!();
    let mut polygon: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let line: String = read!();
        let nums: Vec<&str> = line.split(',').collect();
        polygon.push((
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap()
        ));
    }

    let mut max_area: i64 = 0;
    for i in 0..n {
        for j in i+1..n {
            if 
            is_ray_start_inside((polygon[j].0, polygon[i].1), n, &polygon) 
            &&
            is_ray_start_inside((polygon[i].0, polygon[j].1), n, &polygon)
            {
                let tmp_area = area(polygon[i], polygon[j]);
                if tmp_area > max_area {
                    dbg!(polygon[i]);
                    dbg!(polygon[j]);
                    dbg!(tmp_area);
                    max_area = tmp_area;
                }
            }
        }
    }

    dbg!(max_area);
}
