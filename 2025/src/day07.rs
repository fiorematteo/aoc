use std::collections::HashSet;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (y, x)))
        .unwrap();
    assert!(grid[start.0][start.1] == 'S');
    let mut queue = vec![start];
    let mut splitter_set = HashSet::new();
    while let Some((y, x)) = queue.pop() {
        if grid[y][x] == '^' {
            if splitter_set.contains(&(y, x)) {
                continue;
            }
            splitter_set.insert((y, x));
            assert!(x > 0);
            queue.push((y, x - 1));

            assert!(x < grid[0].len() - 1);
            queue.push((y, x + 1));
        } else if y < grid.len() - 1 {
            queue.push((y + 1, x));
        }
    }
    splitter_set.len()
}


#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut timeline_grid = grid
        .iter()
        .map(|row| row.iter().map(|&c| if c == 'S' { 1 } else { 0 }).collect())
        .collect::<Vec<Vec<usize>>>();
    for y in 0..grid.len() - 1 {
        for x in 0..grid[0].len() {
            if timeline_grid[y][x] == 0 {
                continue;
            }
            if grid[y][x] == '^' {
                assert!(x > 0);
                timeline_grid[y + 1][x - 1] += timeline_grid[y][x];
                assert!(x < grid[0].len() - 1);
                timeline_grid[y + 1][x + 1] += timeline_grid[y][x];
            } else {
                timeline_grid[y + 1][x] += timeline_grid[y][x];
            }
        }
    }
    timeline_grid[grid.len() - 1].iter().sum()
}

#[test]
fn part1_test() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part1(input), 21);

    let input = "..S..
..^..
.^.^.
..^..
..^..
.....";
    assert_eq!(part1(input), 4);
}

#[test]
fn part2_test() {
    let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    assert_eq!(part2(input), 40);
}
