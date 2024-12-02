use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-1-historian-hysteria-input.txt")
        .expect("Should have been able to read the file");

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let list_size = contents.lines().count();

    for content in contents.lines() {
        let mut split = content.split_whitespace();

        let left = split.next().unwrap().parse::<i32>().unwrap();
        left_list.push(left);

        let right = split.next().unwrap().parse::<i32>().unwrap();
        right_list.push(right);
    }

    left_list.sort_by(|a, b| a.cmp(b));
    right_list.sort_by(|a, b| a.cmp(b));

    println!("Part 1: {}", part1(&left_list, &right_list, list_size));
    println!("Part 2: {}", part2(&left_list, &right_list));
}

fn part1(left_list: &Vec<i32>, right_list: &Vec<i32>, list_size: usize) -> i32 {
    let mut distance = 0;

    for i in 0..list_size {
        distance += (left_list[i] - right_list[i]).abs();
    }

    distance
}

fn part2(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left_list_map: HashMap<i32, i32> = left_list.iter().map(|&n| (n, 0)).collect();

    for right in right_list.iter() {
        if let Some(n) = left_list_map.get_mut(right) {
            *n += 1;
        }
    }

    for left_map in left_list_map.iter() {
        result += left_map.0 * left_map.1;
    }

    result
}
