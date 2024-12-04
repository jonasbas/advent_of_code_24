use std::fs;

use regex::Regex;

pub fn part_one() {
    let content = fs::read_to_string("src/day_3/input.txt").unwrap();

    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Regex error.");

    let result: i32 = reg
        .captures_iter(&content)
        .map(|capture| {
            let one = capture[1].parse::<i32>().unwrap();
            let two = capture[2].parse::<i32>().unwrap();

            one * two
        })
        .sum();

    println!("* Part one: {}", result);
}

pub fn part_two() {
    let content = fs::read_to_string("src/day_3/input.txt").unwrap();

    let all_reg = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").expect("Regex error.");
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Regex error.");

    let mut is_enabled = true;

    let result: i32 = all_reg
        .find_iter(&content)
        .map(|x| {
            let s = x.as_str();
            if s.starts_with("mul") {
                if is_enabled {
                    let capture = reg.captures(s).unwrap();
                    let one = capture[1].parse::<i32>().unwrap();
                    let two = capture[2].parse::<i32>().unwrap();

                    return one * two;
                } else {
                    return 0;
                }
            } else if s.starts_with("don't()") {
                is_enabled = false;
                return 0;
            } else {
                is_enabled = true;
                return 0;
            }
        })
        .sum();

    println!("* Part two: {}", result);
}
