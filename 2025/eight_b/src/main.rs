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

// writing disjoined set union for the first time
fn find_set(v: usize, parent: &Vec<usize>) -> usize {
    if v == parent[v] {
        v
    } else {
        find_set(parent[v], parent)
    }
}
fn union_set(n: usize, a: usize, b: usize, parent: &mut Vec<usize>, size: &mut Vec<usize>) -> bool {
    let s_a: usize = find_set(a, parent);
    let s_b: usize = find_set(b, parent);

    if s_a != s_b {
        parent[s_b] = s_a;
        size[s_a] += size[s_b];
    }

    size[s_a] == n
}

fn solution(n: usize, dists: Vec<(f64, (usize, usize))>, points: Vec<(i64, i64, i64)>) -> i64 {
    let mut parent: Vec<usize> = vec![0; n];
    let mut size: Vec<usize> = vec![1; n]; // all sets will have size as 1 when initialised
    for i in 0..n {
        // initilise all the vertexes to be a single elemented set
        parent[i] = i;
    }

    let mut ans = 0;
    for dist in dists {
        if union_set(n, dist.1.0, dist.1.1, &mut parent, &mut size) {
            ans = points[dist.1.0].0 * points[dist.1.1].0;
            break;
        } 
    }

    ans
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

    println!("ans = {}", solution(n, dists, points));
}
