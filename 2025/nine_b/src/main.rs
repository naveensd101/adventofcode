use text_io::read;
use std::cmp::{max, min};
use geo::{Contains, Covers, Intersects};
use geo_types::{Polygon, LineString, line_string, polygon, Rect, coord};

fn area(pt1: (i64, i64), pt2: (i64, i64)) -> i64 {
    (max(pt1.0, pt2.0) - min(pt1.0, pt2.0) + 1)
    *
    (max(pt1.1, pt2.1) - min(pt1.1, pt2.1) + 1)
}
fn main() {
    let n: usize = read!();
    let mut polygon_points: Vec<(i64, i64)> = vec![];
    let mut polygon_points_copy: Vec<(f64, f64)> = vec![];
    for _ in 0..n {
        let line: String = read!();
        let nums: Vec<&str> = line.split(',').collect();
        polygon_points.push((
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap()
        ));
        polygon_points_copy.push((
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap()
        ));
    }
    let polygon = Polygon::new(
        LineString::from(polygon_points_copy), vec![]
    );

    let mut max_area = 0;
    for i in 0..n {
        for j in i+1..n {
            let big_x = max(polygon_points[i].0, polygon_points[j].0);
            let small_x = min(polygon_points[i].0, polygon_points[j].0);
            let big_y = max(polygon_points[i].1, polygon_points[j].1);
            let small_y = min(polygon_points[i].1, polygon_points[j].1);
            let rect = Rect::new(
                coord! {x: small_x as f64, y: small_y as f64},
                coord! {x: big_x as f64, y: big_y as f64},
            );

            if polygon.contains(&rect) {
                max_area = max(max_area, area((small_x, small_y), (big_x, big_y)));
                println!("{i}. {}", max_area);
            }
        }
    }

    println!("{}", max_area);
}

fn _main() {
    let polygon_points: Vec<(i64, i64)> = vec![
        (3, 2), (6, 2), (6, 4), (3, 4)
    ];
    let rectangle_points: Vec<(i64, i64)> = vec![
        (3, 2), (4, 2), (4, 3), (3, 3)
    ];

    let polygon = Polygon::new(
        LineString::from(polygon_points), vec![]
    );

    let line_segment = line_string![(x: 4, y: 3), (x: 5, y: 3)];
    // let line_segment = line_string![(x: 40, y: 30), (x: 50, y: 30)];
    println!("{:?}", polygon.intersects(&line_segment));
}