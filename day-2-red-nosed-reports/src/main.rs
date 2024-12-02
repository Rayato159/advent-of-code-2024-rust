use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-2-red-nosed-reports-input.txt")
        .expect("Should have been able to read the file");

    let mut numbers_2d_list: Vec<Vec<i32>> = Vec::new();

    for content in contents.lines() {
        let numbers: Vec<i32> = content
            .split_whitespace()
            .map(|element| element.parse::<i32>().unwrap())
            .collect();

        numbers_2d_list.push(numbers);
    }

    println!("Part 1: {}", part1(&numbers_2d_list));
    println!("Part 2: {}", part2(&numbers_2d_list));
}

fn part1(numbers_2d_list: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for numbers in numbers_2d_list.iter() {
        let mut direction = 0;
        let mut is_safe = true;

        for i in 0..numbers.len() - 1 {
            let diff = numbers[i] - numbers[i + 1];
            if diff.abs() < 1 || diff.abs() > 3 {
                is_safe = false;
                break;
            }

            if direction == 0 {
                direction = if diff > 0 { 1 } else { -1 };
            } else if (direction == 1 && diff <= 0) || (direction == -1 && diff >= 0) {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            result += 1;
        }
    }

    result
}

fn part2(numbers_2d_list: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for numbers in numbers_2d_list.iter() {
        let mut is_safe = safe_checking(numbers);

        if !is_safe {
            for i in 0..numbers.len() {
                let new_numbers = remove_number(i, numbers);
                is_safe = safe_checking(&new_numbers);

                if is_safe {
                    break;
                }
            }
        }

        if is_safe {
            result += 1;
        }
    }

    result
}

fn safe_checking(numbers: &Vec<i32>) -> bool {
    let mut direction = 0;
    let mut is_safe = true;

    for i in 0..numbers.len() - 1 {
        let diff = numbers[i] - numbers[i + 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            is_safe = false;
            break;
        }

        if direction == 0 {
            direction = if diff > 0 { 1 } else { -1 };
        } else if (direction == 1 && diff <= 0) || (direction == -1 && diff >= 0) {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn remove_number(index_to_remove: usize, old_numbers: &Vec<i32>) -> Vec<i32> {
    let new_number = old_numbers
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != index_to_remove)
        .map(|(_, number)| *number)
        .collect();

    new_number
}
