use std::{collections::HashMap, fs};

pub fn part_one() {
    let (mut list_one, mut list_two) = parse_input();

    list_one.sort();
    list_two.sort();

    let result: i32 = list_one
        .iter()
        .zip(list_two)
        .map(|(x, y)| i32::abs(x - y))
        .sum();

    println!("* Part one: {}", result);
}

pub fn part_two() {
    let (list_one, list_two) = parse_input();

    let mut counting_map: HashMap<i32, i32> = HashMap::new();

    list_one.iter().for_each(|value| {
        let mut count = 0;
        list_two.iter().for_each(|x| {
            if value == x {
                count += 1
            }
        });

        counting_map.insert(*value, count);
    });

    let result: i32 = counting_map.iter().map(|(key, value)| key * value).sum();
    println!("* Part two: {}", result);
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let content = fs::read_to_string("src/day_1/input.txt").unwrap();
    let lines = content.lines();

    let mut list_one = vec![];
    let mut list_two = vec![];

    lines.for_each(|line| {
        let tmp: Vec<&str> = line.split("   ").collect();

        let value_one = tmp[0].parse::<i32>().unwrap();
        let value_two = tmp[1].parse::<i32>().unwrap();

        list_one.push(value_one);
        list_two.push(value_two);
    });

    (list_one, list_two)
}
