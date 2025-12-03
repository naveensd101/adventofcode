use text_io::read;

fn main() {
    let n: i32 = read!();
    let mut ans: i32 = 0;
    for _ in 0..n {
        let line: String = read!();
        println!("{:?}", line);
        let sz: i32 = line.len() as i32;
        let mut max_num: i32 = -1;
        for i in 0..sz {
            for j in i+1..sz {
                let mut num_str: String = String::new();
                num_str.push(line.chars().nth(i as usize).unwrap());
                num_str.push(line.chars().nth(j as usize).unwrap());
                let num: i32 = num_str.parse().unwrap();
                if max_num < num {
                    max_num = num;
                }
            }
        }
        println!("{:?}", max_num);
        ans += max_num;
    }
    println!("ans = {}", ans)
}
