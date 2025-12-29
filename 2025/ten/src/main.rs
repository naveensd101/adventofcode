#![allow(dead_code)]
use text_io::read;

#[derive(Debug)]
struct Manual {
    required_config: i32,
    buttons: Vec<i32>
}

fn reader() -> Manual {
    let mut manual: Manual= Manual{required_config: 0, buttons: vec![]};
    loop {
        let line: String = read!();
        let ch: char = line.chars().next().unwrap();

        if ch == '{' {
            break;
        } else if ch == '[' {
            // line will be something line [.##.]
            // enumerate through the line without the first and last chars which are brackets
            for (idx, val) in line[1..line.len()-1].chars().enumerate() {
                if val == '#' {
                    manual.required_config += 1<<idx;
                }
            }
        }
    }

    manual
}

fn main() {
    let n: usize = read!();
    for _ in 0..n {
        println!("{:?}", reader());
    }
}
