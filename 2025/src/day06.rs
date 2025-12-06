#[aoc(day6, part1)]
pub fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let ops = lines
        .last()
        .unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let mut cols = ops
        .iter()
        .map(|&c| if c == "+" { 0 } else { 1 })
        .collect::<Vec<u64>>();
    for line in lines.iter().take(lines.len() - 1) {
        for (i, x) in line
            .split_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u64>().unwrap())
            .enumerate()
        {
            if ops[i] == "+" {
                cols[i] += x;
            } else {
                cols[i] *= x;
            }
        }
    }
    cols.into_iter().sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let l = lines.len() - 1;
    let ops = lines
        .last()
        .unwrap()
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let grid = lines
        .iter()
        .take(l)
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut x = 0;
    let mut total = 0;
    let mut current_op = 0;
    let mut current_number = if ops[current_op] == "+" { 0 } else { 1 };
    while x < grid[0].len() {
        if let Some(n) = read_number(&grid, x, l) {
            if ops[current_op] == "+" {
                current_number += n;
            } else {
                current_number *= n;
            }
        } else {
            total += current_number;
            current_op += 1;
            current_number = if ops[current_op] == "+" { 0 } else { 1 };
        }
        x += 1;
    }
    total + current_number
}

fn read_number(grid: &[Vec<char>], x: usize, l: usize) -> Option<u64> {
    let mut digits = Vec::new();
    for row in grid.iter().take(l) {
        let c = row[x];
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap() as u64);
        } else {
            continue;
        }
    }
    if digits.is_empty() {
        None
    } else {
        Some(digits.into_iter().fold(0u64, |acc, d| acc * 10 + d))
    }
}

#[test]
fn part1_test() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part1(input), 4277556);
}

#[test]
fn part2_test() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    assert_eq!(part2(input), 3263827);
}
