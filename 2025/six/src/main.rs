use text_io::read;

fn custom_read() -> Vec<String> {
    let line: String = read!("{}\n");
    let new_line = line.trim();
    let mut lst: Vec<&str> = new_line.split(' ').collect();
    lst.retain(|x| !x.is_empty());

    let mut result: Vec<String> = vec![];
    for elem in lst {
        result.push(String::from(elem));
    }
    result
}

fn main() {
    let line_count = 4;
    let mut lines: Vec<Vec<String>> = vec![];
    for _ in 0..line_count {
        lines.push(custom_read());
    }
    let symbols = custom_read();

    let mut ans: i64 = 0;
    for i in 0..symbols.len() {
        if symbols[i] == "+" {
            let mut tmp_ans = 0;
            for j in 0..line_count {
                tmp_ans += lines[j][i].parse::<i64>().unwrap();
            }
            ans += tmp_ans;
        } else {
            let mut tmp_ans = 1;
            for j in 0..line_count {
                tmp_ans *= lines[j][i].parse::<i64>().unwrap();
            }
            ans += tmp_ans;
        }
    }

    println!("ans = {}", ans);
}
