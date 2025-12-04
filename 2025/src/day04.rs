#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if is_removable(&map, y, x) {
                total += 1;
            }
        }
    }
    total
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut total = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if is_removable(&map, y, x) {
                    total += 1;
                    removed = true;
                    map[y][x] = '.';
                }
            }
        }
    }
    total
}

fn is_removable(map: &[Vec<char>], y: usize, x: usize) -> bool {
    if map[y][x] != '@' {
        return false;
    }
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny < 0 || ny >= map.len() as isize || nx < 0 || nx >= map[0].len() as isize {
                continue;
            }
            if map[ny as usize][nx as usize] == '@' {
                count += 1;
            }
        }
    }
    count < 4
}

#[test]
fn part1_test() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part1(input), 13);
}

#[test]
fn part2_test() {
    let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    assert_eq!(part2(input), 43);
}
