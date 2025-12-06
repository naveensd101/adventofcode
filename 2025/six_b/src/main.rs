use text_io::read;

fn main() {
    let mut lines: Vec<Vec<char>> = vec![];
    let line_count: usize = 4;
    for _ in 0..line_count {
        let line: String = read!("{}\n");
        lines.push(
            line.chars().collect()
        );
    }
    let line: String = read!("{}\n");
    let symbols: Vec<char> = line.chars().collect();

    let mut ans: i64 = 0;
    let mut numbers: Vec<i64> = vec![];
    for i in (0..symbols.iter().len()).rev() {
        let mut number: Vec<char> = vec![];
        for j in 0..line_count {
            if lines[j][i] != ' ' {
                number.push(lines[j][i]);
            }
        }
        if !number.is_empty() {
            let tmp: String = number.iter().collect();
            numbers.push(tmp.parse().unwrap());
            number.clear();
        }

        if symbols[i] == '*' {
            let mut tmp_ans: i64 = 1;
            while !numbers.is_empty() {
                tmp_ans *= numbers.pop().unwrap();
            }
            ans += tmp_ans;
        } else if symbols[i] == '+' {
            let mut tmp_ans: i64 = 0;
            while !numbers.is_empty() {
                tmp_ans += numbers.pop().unwrap();
            }
            ans += tmp_ans;
        }
    }

    println!("ans = {}", ans);
}
