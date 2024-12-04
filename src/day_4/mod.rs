use std::fs;

pub fn part_one() {
    let content = fs::read_to_string("src/day_4/input.txt").unwrap();
    let lines = content.lines();

    let char_matrix: Vec<Vec<char>> = lines
        .map(|line| {
            let char_list: Vec<char> = line.chars().collect();
            char_list
        })
        .collect();

    let mut result = 0;

    for y in 0..char_matrix.len() {
        for x in 0..char_matrix[y].len() {
            let value = char_matrix[y][x];
            if value == 'X' {
                result += check_part_one(&char_matrix, x, y, 'M', 'A', 'S');
            } else if value == 'S' {
                result += check_part_one(&char_matrix, x, y, 'A', 'M', 'X');
            }
        }
    }

    println!("* Part one: {}", result);
}

pub fn part_two() {
    let content = fs::read_to_string("src/day_4/input.txt").unwrap();
    let lines = content.lines();

    let char_matrix: Vec<Vec<char>> = lines
        .map(|line| {
            let char_list: Vec<char> = line.chars().collect();
            char_list
        })
        .collect();

    let mut result = 0;

    for y in 0..char_matrix.len() {
        for x in 0..char_matrix[y].len() {
            let value = char_matrix[y][x];
            if value == 'M' {
                result += check_part_two(&char_matrix, x, y, 'S')
            } else if value == 'S' {
                result += check_part_two(&char_matrix, x, y, 'M')
            }
        }
    }

    println!("* Part two: {}", result);
}

fn check_part_two(matrix: &Vec<Vec<char>>, x: usize, y: usize, check_one: char) -> i32 {
    let row = &matrix[y];
    let length = row.len();
    let height = matrix.len();

    // matrix not big enough
    if x + 2 >= length || y + 2 >= height {
        return 0;
    }

    let top_right = matrix[y][x + 2];
    let middle = matrix[y + 1][x + 1];
    let bottom_left = matrix[y + 2][x];
    let bottom_right = matrix[y + 2][x + 2];

    if (top_right != 'M' && top_right != 'S') || middle != 'A' {
        return 0;
    }

    if bottom_right != check_one {
        return 0;
    }

    if top_right == 'M' && bottom_left != 'S' {
        return 0;
    }

    if top_right == 'S' && bottom_left != 'M' {
        return 0;
    }

    1
}

fn check_part_one(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    check_one: char,
    check_two: char,
    check_three: char,
) -> i32 {
    let row = &matrix[y];
    let length = row.len();
    let height = matrix.len();

    let mut sum = 0;

    // check horizontal
    if length > x + 3 {
        let next_1 = row[x + 1];
        let next_2 = row[x + 2];
        let next_3 = row[x + 3];

        if next_1 == check_one && next_2 == check_two && next_3 == check_three {
            sum += 1;
        }
    }

    // check vertical
    if height > y + 3 {
        let next_1 = matrix[y + 1][x];
        let next_2 = matrix[y + 2][x];
        let next_3 = matrix[y + 3][x];

        if next_1 == check_one && next_2 == check_two && next_3 == check_three {
            sum += 1;
        }
    }

    // check diagonal right
    if height > y + 3 && length > x + 3 {
        let next_1 = matrix[y + 1][x + 1];
        let next_2 = matrix[y + 2][x + 2];
        let next_3 = matrix[y + 3][x + 3];

        if next_1 == check_one && next_2 == check_two && next_3 == check_three {
            sum += 1;
        }
    }

    // check diagonal left
    if height > y + 3 && x >= 3 {
        let next_1 = matrix[y + 1][x - 1];
        let next_2 = matrix[y + 2][x - 2];
        let next_3 = matrix[y + 3][x - 3];

        if next_1 == check_one && next_2 == check_two && next_3 == check_three {
            sum += 1;
        }
    }

    sum
}
