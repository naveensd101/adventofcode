#![allow(dead_code)]
use std::i32;
use std::cmp::min;
use std::collections::HashMap;
use text_io::read;

#[derive(Debug)]
struct Manual {
    required_config: i32,
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>
}

fn does_vector_have_negative_number(arr: &Vec<i32>) -> bool {
    for el in arr {
        if *el < 0 {
            return true
        }
    }
    false
}

fn are_all_vals_zero(arr: &Vec<i32>) -> bool {
    for el in arr {
        if *el != 0 {
            return false
        }
    }
    true
}

fn reader() -> Manual {
    let mut manual: Manual = Manual { required_config: 0, buttons: vec![], joltage: vec![] };
    loop {
        let word: String = read!(); // read! will read stdin untill it finds a white space
        let ch: char = word.chars().next().unwrap();
        
        if ch == '[' {
            continue;
        } else if ch == '(' {
            // word will be something like (0, 1, 3)
            let csv: String = word[1..word.len()-1].to_owned();
            let num_vector: Vec<i32> = csv.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            manual.buttons.push(num_vector);
        } else if ch == '{' {
            let csv: String = word[1..word.len()-1].to_owned();
            let num_vector: Vec<i32> = csv.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            manual.joltage = num_vector;
            break;
        }
    }

    manual
}

fn f(manual: &Manual, memory: &mut HashMap<Vec<i32>, i32>) -> i32 {
    println!("{:?}", manual);
    println!("{:?}", memory);
    let jolts = manual.joltage.clone();
    if are_all_vals_zero(&jolts) {
        return 0;
    }
    if does_vector_have_negative_number(&jolts) {
        return i32::MAX/2;
    }

    if memory.contains_key(&manual.joltage) {
        return *memory.get(&manual.joltage).unwrap();
    }

    let mut min_ans = i32::MAX;
    for button in manual.buttons.clone() {
        let mut new_jolts = manual.joltage.clone(); //make a copy of joltages
        for idx in button {
            new_jolts[idx as usize] -= 1;
        }
        min_ans = min(min_ans, 
            1 + f(
                &Manual { required_config: 0, buttons: manual.buttons.clone(), joltage: new_jolts },
                memory
            )
        )
    }
    memory.insert(manual.joltage.clone(), min_ans);
    min_ans
}

fn main() {
    let mut memory: HashMap<Vec<i32>, i32> = HashMap::new();

    let n: usize = read!();
    let mut ans: i32 = 0;
    for i in 0..n {
        println!("{i}");
        ans += f(&reader(), &mut memory);
    }
    println!("{:?}", ans);
}
