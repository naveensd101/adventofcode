use text_io::read;

fn main() {
    let n: i32 = read!();
    let mut final_answer: i128 = 0; // thankyou for existing
    for _ in 0..n {
        let line: String = read!();
        let mut answer: Vec<(char, usize)> = vec![];

        let m: usize = line.len();
        // I have never really used usize in any other languages
        for idx in (0..12).rev() {
            answer.push((
                line.chars().nth(m - 1 - idx).unwrap(), 
                m - 1 - idx
            ));
        }
        println!("{:?}", line);

        let mut max_int = answer[0].0;
        let mut max_idx = answer[0].1;
        for i in (0..answer[0].1).rev() {
            if line.chars().nth(i).unwrap() >= max_int {
                max_int = line.chars().nth(i).unwrap();
                max_idx = i;
            }
        }
        answer[0] = (max_int, max_idx);

        for i in 1..12 {
            let l = answer[i-1].1 + 1;
            let r = answer[i].1;

            let mut max_int = answer[i].0;
            let mut max_idx = answer[i].1;
            for idx in (l..=r).rev() {
                if line.chars().nth(idx).unwrap() >= max_int {
                    max_int = line.chars().nth(idx).unwrap();
                    max_idx = idx;
                }
            }
            answer[i] = (max_int, max_idx);
        }

        let mut string_answer = String::new();
        for val in answer {
            string_answer.push(val.0);
        }
        println!("{:?}", string_answer);
        let small_ans: i128 = string_answer.parse().unwrap();
        final_answer += small_ans;
    }
    println!("final ans = {}", final_answer);
}
