use std::fs;

pub fn part_one() {
    let content = fs::read_to_string("src/day_2/input.txt").unwrap();
    let lines = content.lines();

    let result: i32 = lines
        .map(|line| {
            let values: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

            let line_check = check_line(&values);

            if line_check.is_none() {
                return 1;
            }

            0
        })
        .sum();

    println!("* Part one: {}", result);
}

pub fn part_two() {
    let content = fs::read_to_string("src/day_2/input.txt").unwrap();
    let lines = content.lines();

    let result: i32 = lines
        .map(|line| {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let first_check = check_line(&values);

            if first_check.is_none() {
                return 1;
            }

            // check every possible permutation, maybe there is a better way but i couldn't find it
            for i in 0..values.len() {
                let mut copy = values.clone();
                copy.remove(i);

                let second_check = check_line(&copy);

                if second_check.is_none() {
                    return 1;
                }
            }

            0
        })
        .sum();

    println!("* Part two: {}", result);
}

// function returns option because of an old implementation, but i can't be bothered to change it :p
fn check_line(values: &Vec<i32>) -> Option<i32> {
    let mut ascending = false;
    let mut descending = false;

    values
        .windows(2)
        .enumerate()
        .map(|(idx, window)| {
            let first = window[0];
            let second = window[1];

            let is_ascending = first > second;

            if is_ascending && descending {
                return idx as i32;
            }

            if !is_ascending && ascending {
                return idx as i32;
            }

            ascending = is_ascending;
            descending = !is_ascending;

            let difference_abs = i32::abs(first - second);
            if difference_abs > 3 || difference_abs < 1 {
                return idx as i32;
            }

            -1
        })
        .filter(|x| *x != -1)
        .min()
}
