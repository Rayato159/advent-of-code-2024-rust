use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-3-mull-it-over-input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", part1(contents.clone()));
    println!("Part 2: {}", part2(contents.clone()));
}

fn part1(contents: String) -> i32 {
    let pattern = Regex::new(r"mul\(\d+,\d+\)").expect("Invalid regex");

    let mul_zip_str_list: Vec<&str> = pattern
        .find_iter(&contents)
        .map(|mat| mat.as_str())
        .collect();

    let mut result = 0;

    for mul in mul_zip_str_list.iter() {
        let mut split = mul.split(",");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        let first_num: i32 = first[4..].parse().unwrap();
        let second_num: i32 = second[..second.len() - 1].parse().unwrap();
        result += first_num * second_num;
    }

    result
}

fn part2(contents: String) -> i32 {
    let pattern = r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).expect("Invalid regex");

    let mut is_enabled = true;
    let mut result = 0;

    for caps in re.captures_iter(&contents) {
        let matched_str = caps.get(0).unwrap().as_str();

        if matched_str == "do()" {
            is_enabled = true;
        } else if matched_str == "don't()" {
            is_enabled = false;
        } else if matched_str.starts_with("mul(") {
            if is_enabled {
                let x: i32 = caps
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Invalid number");
                let y: i32 = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Invalid number");
                result += x * y;
            }
        }
    }

    result
}
