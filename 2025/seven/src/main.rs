use text_io::read;

// animating this simulation will be a fun project to do. you should be able to store printer fn value as a frame for the video

fn printer(n: i32, map: &Vec<Vec<char>>) {
    for i in 0..n {
        let ln: String = map[i as usize].iter().collect();
        println!("{}", ln);
    }
}

fn main() {
    let n: i32 = 16;
    let m: i32 = 15;
    let mut map: Vec<Vec<char>> = vec![];

    for _ in 0..n {
        let line: String = read!();
        map.push(line.chars().collect());
    }

    for i in 0..n {
        println!("{:?}", map[i as usize]);
    }

    for i in 1..n {
        for j in 0..m {
            if map[(i - 1) as usize][j as usize] == 'S' || map[(i - 1) as usize][j as usize] == '|' {
                if map[i as usize][j as usize] == '.' {
                    map[i as usize][j as usize] = '|';
                } else if map[i as usize][j as usize] == '^' {
                    if j - 1 > -1 && map[i as usize][(j - 1) as usize] == '.' {
                        map[i as usize][(j - 1) as usize] = '|';
                    }
                    if j + 1 < m && map[i as usize][(j + 1) as usize] == '.' {
                        map[i as usize][(j + 1) as usize] = '|';
                    }
                }
            }
            // printer(n, &map);
        }
    }

    printer(n, &map);
    let mut ans: i32 = 0;
    for i in 1..n {
        for j in 0..m {
            if map[i as usize][j as usize] == '^' && (map[(i - 1) as usize][j as usize] == '|' || map[(i - 1) as usize][j as usize] == 'S') {
                ans += 1;
            }
        }
    }
    println!("ans = {}", ans);

}
