use text_io::read;

fn is_i_j_valid_index(i: i32, j: i32, n: i32) -> bool {
    i > -1 && j > -1 && i < n && j < n
}

fn count_neighbor(i: usize, j: usize, n: usize, map: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for val in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
        if is_i_j_valid_index(
            i as i32 + val.0, 
            j as i32 + val.1, 
            n as i32
        ) {
            if map[ (i as i32 + val.0) as usize ][ (j as i32 + val.1) as usize ] == '@' {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    let n: usize = read!(); // Its a square map (atleast for my test case)
    let mut map: Vec<Vec<char>> = vec![];

    for _ in 0..n {
        let line: String = read!();
        map.push(line.chars().collect());
    }

    let mut ans: i32 = 0;
    for i in 0..n {
        for j in 0..n {
            if map[i][j] == '@' && count_neighbor(i, j, n, &map) < 4 {
                ans += 1;
                // print!("x");
            } else {
                // print!("{}", &map[i][j]);
            }
        }
        // println!();
    }
    println!("ans = {}", ans);
}
