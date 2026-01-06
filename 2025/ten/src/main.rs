#![allow(dead_code)]
use text_io::read;

#[derive(Debug)]
struct Manual {
    required_config: i32,
    buttons: Vec<i32>
}

fn min_config(manual: Manual) -> i32 {
    let power_set_of_buttons = power_set(manual.buttons);
    let mut min_ans = i32::MAX;
    for subset in power_set_of_buttons {
        let mut xorer = 0;
        let subset_len = subset.len();
        for num in subset {
            xorer ^= num;
        }
        if xorer == manual.required_config {
            min_ans = min_ans.min(subset_len as i32);
        }
    }
    min_ans
}

fn power_set<T: Copy>(array: Vec<T>) -> Vec<Vec<T>> {
    let mut power_set = vec![];
    let n: usize = array.len();

    for i in 0.. 1 << n {
        let mut subset = vec![];
        for j in 0 ..n {
            if i & (1 << j) != 0 {
                subset.push(array[j]);
            }
        }
        power_set.push(subset);
    }

    power_set
}

fn reader() -> Manual {
    let mut manual: Manual= Manual{required_config: 0, buttons: vec![]};
    loop {
        let word: String = read!();
        let ch: char = word.chars().next().unwrap();

        if ch == '{' {
            break;
        } else if ch == '[' {
            // line will be something line [.##.]
            // enumerate through the line without the first and last chars which are brackets
            for (idx, val) in word[1..word.len()-1].chars().enumerate() {
                if val == '#' {
                    manual.required_config += 1<<idx;
                }
            }
        } else if ch == '(' {
            // word will be something like (0,1,3)
            let csv: String = word[1..word.len()-1].to_owned();
            let num_vector: Vec<i32> = csv.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut button = 0;
            for num in num_vector {
                button += 1 << num;
            }
            manual.buttons.push(button);
        }
    }

    manual
}

fn main() {
    let n: usize = read!();
    let mut ans: i32 = 0;
    for _ in 0..n {
        ans += min_config(reader());
    }
    println!("ans = {}", ans);
}

#[cfg(test)]
mod tests {
    use super::power_set;

    #[test]
    fn test_power_set() {
        let set = vec![1, 2, 3];
        let mut expected_power_set = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ];
        expected_power_set.sort();

        let mut result = power_set(set);
        result.sort();

        assert_eq!(result, expected_power_set);
    }
}
