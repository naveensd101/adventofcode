#[allow(unused)]
fn find_sum_except_i_for_bulb(
    bulb_id: i32, 
    i: usize, 
    state: &Vec<Option<i32>>, 
    buttons: &Vec<Vec<i32>>
) -> Option<i32> {
    // state has got the clicks of each button in buttons list
    let mut sum: i32 = 0;
    for (idx, button) in buttons.iter().enumerate() {
        if idx == i {
            continue;
        }
        for bulb in button {
            if *bulb == bulb_id {
                if state[idx].is_none() {
                    return None;
                } else {
                    sum += state[idx].unwrap()
                }
            }
        }
    }
    Some(sum)
}

fn recursive_normalise(state: Vec<Option<i32>>, buttons: Vec<Vec<i32>>, bulb_joltage_req: Vec<i32>) -> Vec<Option<i32>> {
    let cur_state = state.to_owned();
    loop {
        let normalised = normalise(cur_state, &buttons, &bulb_joltage_req);
        if normalised == cur_state {
            return normalised;
        } else {
            cur_state = normalised.to_vec()
        }
    }
}

fn normalise(state: Vec<Option<i32>>, buttons: &Vec<Vec<i32>>, bulb_joltage_req: &Vec<i32>) -> Vec<Option<i32>> {
    // Goal: If any state position is collapsable based on the other set values, then collapse it
    let mut normalised_state: Vec<Option<i32>> = vec![];
    let start_state = state.to_owned();
    for (i, clicks) in start_state.iter().enumerate() {
        if clicks.is_none() {
            for bulb_id in buttons[i].iter() {
                let sum: Option<i32> = find_sum_except_i_for_bulb(*bulb_id, i, &start_state, &buttons);
                if sum.is_some() {
                    normalised_state.push(Some(bulb_joltage_req[*bulb_id as usize] - sum.unwrap()));
                    break;
                }
            }
        }
        if normalised_state.len() != (i+1) {
            normalised_state.push(state[i]);
        }
    }
    normalised_state
}

fn main() {
    let bulb_joltage_req: Vec<i32> = vec![3, 5, 4, 7];
    let buttons: Vec<Vec<i32>> = vec![
        vec![3],
        vec![1, 3],
        vec![2],
        vec![2, 3],
        vec![0, 2],
        vec![0, 1],
    ];
    let _n = buttons.len();
    let _m = bulb_joltage_req.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_finder_1() {
        let bulb_id = 0;
        let i = 4;
        let state = vec![
            None,
            None,
            None,
            None,
            None,
            Some(3)
        ];
        let buttons: Vec<Vec<i32>> = vec![
            vec![3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![0, 2],
            vec![0, 1],
        ];
        let result = find_sum_except_i_for_bulb(bulb_id, i, &state, &buttons);
        assert_eq!(Some(3), result);
    }

    #[test]
    fn test_sum_finder_2() {
        //x2 + x3 + x4 = 4
        //for bulb_id = 2
        let bulb_id = 2;
        let i = 4;
        let state = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/Some(4),
            /*4*/None,
            /*5*/None
        ];
        let buttons: Vec<Vec<i32>> = vec![
            /*0*/vec![3],
            /*1*/vec![1, 3],
            /*2*/vec![2],
            /*3*/vec![2, 3],
            /*4*/vec![0, 2],
            /*5*/vec![0, 1],
        ];
        let result = find_sum_except_i_for_bulb(bulb_id, i, &state, &buttons);
        assert_eq!(Some(7), result);
    }

    #[test]
    fn test_sum_finder_3() {
        //x2 + x3 + x4 = 4
        //for bulb_id = 2
        let bulb_id = 2;
        let i = 4;
        let state = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/None,
            /*4*/None,
            /*5*/None
        ];
        let buttons: Vec<Vec<i32>> = vec![
            /*0*/vec![3],
            /*1*/vec![1, 3],
            /*2*/vec![2],
            /*3*/vec![2, 3],
            /*4*/vec![0, 2],
            /*5*/vec![0, 1],
        ];
        let result = find_sum_except_i_for_bulb(bulb_id, i, &state, &buttons);
        assert_eq!(None, result);
    }

    #[test]
    fn test_normalise_1() {
        //x2 + x3 + x4 = 4
        let bulb_joltage_req: Vec<i32> = vec![3, 5, 4, 7];
        let state = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/Some(0),
            /*4*/None,
            /*5*/None
        ];
        let buttons: Vec<Vec<i32>> = vec![
            /*0*/vec![3],
            /*1*/vec![1, 3],
            /*2*/vec![2],
            /*3*/vec![2, 3],
            /*4*/vec![0, 2],
            /*5*/vec![0, 1],
        ];
        let result = normalise(state, buttons, bulb_joltage_req);
        let expected = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/Some(0),
            /*4*/Some(1),
            /*5*/None
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_normalise_2() {
        //x2 + x3 + x4 = 4
        let bulb_joltage_req: Vec<i32> = vec![3, 5, 4, 7];
        let state = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/None,
            /*4*/None,
            /*5*/None
        ];
        let buttons: Vec<Vec<i32>> = vec![
            /*0*/vec![3],
            /*1*/vec![1, 3],
            /*2*/vec![2],
            /*3*/vec![2, 3],
            /*4*/vec![0, 2],
            /*5*/vec![0, 1],
        ];
        let result = normalise(state, buttons, bulb_joltage_req);
        let expected = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/None,
            /*4*/None,
            /*5*/None
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_normalise_3() {
        //x2 + x3 + x4 = 4
        let bulb_joltage_req: Vec<i32> = vec![3, 5, 4, 7];
        let state = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(2),
            /*3*/None,
            /*4*/Some(1),
            /*5*/None
        ];
        let buttons: Vec<Vec<i32>> = vec![
            /*0*/vec![3],
            /*1*/vec![1, 3],
            /*2*/vec![2],
            /*3*/vec![2, 3],
            /*4*/vec![0, 2],
            /*5*/vec![0, 1],
        ];
        let result = normalise(state, buttons, bulb_joltage_req);
        let expected = vec![
            /*0*/None,
            /*1*/None,
            /*2*/Some(3),
            /*3*/None,
            /*4*/None,
            /*5*/None
        ];
        assert_eq!(expected, result);
    }
}