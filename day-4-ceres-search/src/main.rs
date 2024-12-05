use std::fs;

fn main() {
    let contents = fs::read_to_string("./day-4-ceres-search-input.txt")
        .expect("Should have been able to read the file");
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let bounds = (grid.len(), grid[0].len()); // (y, x)

    let mut part_1_result = 0;
    let xmas_vector: Vec<char> = "XMAS".chars().collect();
    let xmas_directions = [
        (0, 1),   // left-to-right
        (0, -1),  // right-to-left
        (1, 0),   // top-to-bottom
        (-1, 0),  // bottom-to-top
        (1, 1),   // top-left to bottom-right
        (-1, -1), // bottom-right to top-left
        (1, -1),  // top-right to bottom-left
        (-1, 1),  // bottom-left to top-right
    ];

    let mut part_2_result = 0;
    for y in 0..bounds.0 {
        for x in 0..bounds.1 {
            let origin = (y, x);
            part_1_result +=
                count_word_occurrences_part1(&grid, origin, &xmas_directions, &xmas_vector, bounds);
            if is_xmas_pattern(&grid, (y, x), bounds) {
                part_2_result += 1;
            }
        }
    }

    println!("Part1: {}", part_1_result);
    println!("Part2: {}", part_2_result);
}

fn count_word_occurrences_part1(
    grid: &Vec<Vec<char>>,
    origin: (usize, usize),
    directions: &[(isize, isize)],
    target_vector: &Vec<char>,
    bounds: (usize, usize),
) -> usize {
    directions
        .iter()
        .filter(|&&direction| {
            matches_sequence_part1(grid, origin, direction, target_vector, bounds)
        })
        .count()
}

fn matches_sequence_part1(
    grid: &Vec<Vec<char>>,
    origin: (usize, usize),
    direction: (isize, isize),
    target_vector: &Vec<char>,
    bounds: (usize, usize),
) -> bool {
    let (mut y, mut x) = (origin.0 as isize, origin.1 as isize);
    let (dy, dx) = direction;

    for &symbol in target_vector {
        if y < 0 || x < 0 || y >= bounds.0 as isize || x >= bounds.1 as isize {
            return false;
        }
        if grid[y as usize][x as usize] != symbol {
            return false;
        }
        y += dy;
        x += dx;
    }

    true
}

fn is_xmas_pattern(grid: &Vec<Vec<char>>, center: (usize, usize), bounds: (usize, usize)) -> bool {
    let (cy, cx) = center;

    // Ensure the center is 'A'
    if grid[cy][cx] != 'A' {
        return false;
    }

    let top_left = (cy as isize - 1, cx as isize - 1);
    let top_right = (cy as isize - 1, cx as isize + 1);
    let bottom_left = (cy as isize + 1, cx as isize - 1);
    let bottom_right = (cy as isize + 1, cx as isize + 1);

    if !is_valid_point(top_left, bounds)
        || !is_valid_point(top_right, bounds)
        || !is_valid_point(bottom_left, bounds)
        || !is_valid_point(bottom_right, bounds)
    {
        return false;
    }

    // Check for the 'MAS' or 'SAM' patterns
    let corners = [
        grid[top_left.0 as usize][top_left.1 as usize],
        grid[top_right.0 as usize][top_right.1 as usize],
        grid[bottom_left.0 as usize][bottom_left.1 as usize],
        grid[bottom_right.0 as usize][bottom_right.1 as usize],
    ];

    // Ensure top-left and bottom-right form 'MAS' or 'SAM'
    let top_bottom = [corners[0], grid[cy][cx], corners[3]];
    let bottom_top = [corners[2], grid[cy][cx], corners[1]];

    is_mas(&top_bottom) && is_mas(&bottom_top)
}

fn is_valid_point(point: (isize, isize), bounds: (usize, usize)) -> bool {
    let (y, x) = point;
    y >= 0 && x >= 0 && y < bounds.0 as isize && x < bounds.1 as isize
}

fn is_mas(pattern: &[char; 3]) -> bool {
    pattern == &['M', 'A', 'S'] || pattern == &['S', 'A', 'M']
}
