use std::collections::HashMap;
use text_io::read;

fn is_point_valid(n: i64, m: i64, cur_i: i64, cur_j: i64) -> bool {
    cur_i < n && cur_j < m
}

fn find_next_position(n: i64, m: i64, cur_i: i64, cur_j: i64, map: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    let mut next_position: Vec<(i64, i64)> = vec![];
    if !is_point_valid(n, m, cur_i+1, cur_j) {
        next_position.push((cur_i+1, cur_j));
    } else if map[(cur_i + 1) as usize][cur_j as usize] == '^' {
        next_position.push((cur_i+1, cur_j+1)); // There are no ^^ patterns, only ^.^ patterns
        next_position.push((cur_i+1, cur_j-1));
    } else {
        next_position.push((cur_i+1, cur_j));
    }
    next_position
}

// I spend two days trying to make a formula between the answer and the number of ^ shapes
// I am a better coder than a combinatorics guy.
// The first solution that I wrote _main, brute force one is pretty, ill keep it
// half of my time writing rust code is me fighting with the compiler.
// This forces me to take a pen and paper and be sure that the logic is correct.
// In a way the handicap of fighting with the compiler is helpful.
fn count_ways(n: i64, m: i64, cur_i: i64, cur_j: i64, map: &Vec<Vec<char>>, memory: &mut HashMap<(i64, i64), i64>) -> i64 {
    let ans;
    if !memory.contains_key(&(cur_i, cur_j)) {
        println!("i = {:?}, j = {:?}", cur_i, cur_j);
        if !is_point_valid(n, m, cur_i+1, cur_j) {
            ans = 1;
        } else if map[(cur_i + 1) as usize][cur_j as usize] == '.' {
            ans = count_ways(n, m, cur_i+1, cur_j, map, memory);
        } else {
            ans = 
            count_ways(n, m, cur_i+1, cur_j-1, map, memory)
            + 
            count_ways(n, m, cur_i+1, cur_j+1, map, memory)
        }
    } else {
        ans = memory.get(&(cur_i, cur_j)).unwrap().to_owned();
    }
    memory.insert((cur_i, cur_j), ans);
    ans
}

fn main() {
    let n: i64 = 142;
    let m: i64 = 141;
    let mut map: Vec<Vec<char>> = vec![];

    for _ in 0..n {
        let line: String = read!();
        map.push(line.chars().collect());
    }

    for i in 0..n {
        println!("{:?}", map[i as usize]);
    }

    let mut memory: HashMap<(i64, i64), i64>= HashMap::new();
    let mut s_i: i64 = 0;
    let mut s_j: i64 = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i as usize][j as usize] == 'S' {
                s_i = i;
                s_j = j;
                break;
            }
        }
    }

    println!("ans = {}", count_ways(n, m, s_i+1, s_j, &map, &mut memory));
}

fn _main() {
    // brute force gods here we come
    let n: i64 = 142;
    let m: i64 = 141;
    let mut map: Vec<Vec<char>> = vec![];

    for _ in 0..n {
        let line: String = read!();
        map.push(line.chars().collect());
    }

    for i in 0..n {
        println!("{:?}", map[i as usize]);
    }

    let mut particle_keepr: Vec<(i64, i64)> = vec![];
    let mut s_i: i64 = 0;
    let mut s_j: i64 = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i as usize][j as usize] == 'S' {
                s_i = i;
                s_j = j;
                break;
            }
        }
    }
    particle_keepr.push((s_i, s_j)); // Find S and push it to particle keeper

    let mut winner_particle: Vec<(i64, i64)> = vec![];
    let mut idx: i64 = 0;
    while !particle_keepr.is_empty() {
        let particle_coordinates = particle_keepr.pop().unwrap();
        println!("{}.current_particle = {:?}", idx, particle_coordinates);
        idx+=1;
        if !is_point_valid(n, m, particle_coordinates.0, particle_coordinates.1) {
            winner_particle.push(particle_coordinates);
        } else {
            for particle in find_next_position(n, m, particle_coordinates.0, particle_coordinates.1, &map) {
                particle_keepr.push(particle);
            }
        }
    }

    println!("ans = {}", winner_particle.len());
}
