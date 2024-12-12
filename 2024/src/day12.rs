use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut visited: HashSet<Pair> = HashSet::new();
    visited.insert((0, 0).into());
    let mut region_area = 1;
    let mut region_perimeter = 0;
    let mut total = 0;
    let mut queue: Vec<Pair> = vec![(0, 0).into()];
    let in_bounds =
        |p: Pair| p.y >= 0 && p.y < grid.len() as i32 && p.x >= 0 && p.x < grid[0].len() as i32;
    loop {
        if let Some(coords) = queue.pop() {
            for dir in [
                Pair { x: 0, y: -1 },
                Pair { x: 0, y: 1 },
                Pair { x: -1, y: 0 },
                Pair { x: 1, y: 0 },
            ] {
                let next = coords + dir;
                if in_bounds(next)
                    && grid[next.y as usize][next.x as usize]
                        == grid[coords.y as usize][coords.x as usize]
                {
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(next);
                    region_area += 1;
                    queue.push(next);
                } else {
                    region_perimeter += 1;
                }
            }
            continue;
        }

        // new region
        total += region_perimeter * region_area;
        region_area = 1;
        region_perimeter = 0;

        let mut done = true;
        for (y, x) in (0..grid.len()).flat_map(|y| (0..grid[0].len()).map(move |x| (y, x))) {
            let pair = (y as i32, x as i32).into();
            if !visited.contains(&pair) {
                queue.push(pair);
                visited.insert(pair);
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
    total
}
#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let mut visited: HashSet<Pair> = HashSet::new();
    visited.insert((0, 0).into());
    let mut region_area = 1;
    let mut region_sides = 0;
    let mut total = 0;
    let mut queue: Vec<Pair> = vec![(0, 0).into()];
    let in_bounds =
        |p: Pair| p.y >= 0 && p.y < grid.len() as i32 && p.x >= 0 && p.x < grid[0].len() as i32;
    let is_neighbour = |a: Pair, b: Pair| {
        in_bounds(a) && grid[a.y as usize][a.x as usize] == grid[b.y as usize][b.x as usize]
    };
    loop {
        if let Some(coords) = queue.pop() {
            for dir in [
                Pair { x: 0, y: -1 },
                Pair { x: 0, y: 1 },
                Pair { x: -1, y: 0 },
                Pair { x: 1, y: 0 },
            ] {
                let next = coords + dir;
                if is_neighbour(next, coords) {
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(next);
                    region_area += 1;
                    queue.push(next);
                }
            }
            let up = is_neighbour(coords + Pair { x: 0, y: -1 }, coords);
            let down = is_neighbour(coords + Pair { x: 0, y: 1 }, coords);
            let left = is_neighbour(coords + Pair { x: -1, y: 0 }, coords);
            let right = is_neighbour(coords + Pair { x: 1, y: 0 }, coords);
            let top_left = is_neighbour(coords + Pair { x: -1, y: -1 }, coords);
            let top_right = is_neighbour(coords + Pair { x: 1, y: -1 }, coords);
            let bottom_left = is_neighbour(coords + Pair { x: -1, y: 1 }, coords);
            let bottom_right = is_neighbour(coords + Pair { x: 1, y: 1 }, coords);

            // in corners
            region_sides += (up && right && !top_right) as u32
                + (up && left && !top_left) as u32
                + (down && right && !bottom_right) as u32
                + (down && left && !bottom_left) as u32;
            // out corners (diagonals are separated)
            region_sides += (!up && !right) as u32
                + (!up && !left) as u32
                + (!down && !right) as u32
                + (!down && !left) as u32;
            continue;
        }

        // new region
        total += region_sides * region_area;
        region_area = 1;
        region_sides = 0;

        let mut done = true;
        for (y, x) in (0..grid.len()).flat_map(|y| (0..grid[0].len()).map(move |x| (y, x))) {
            let pair = (y as i32, x as i32).into();
            if !visited.contains(&pair) {
                queue.push(pair);
                visited.insert(pair);
                done = false;
                break;
            }
        }
        if done {
            break;
        }
    }
    total as _
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pair {
    x: i32,
    y: i32,
}

impl Add for Pair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Pair {
    fn from(value: (i32, i32)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

#[test]
fn test_part1() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part1(input), 140);
}

#[test]
fn test_part2() {
    let input = "AAAA
BBCD
BBCC
EEEC";
    assert_eq!(part2(input), 80);
}

#[test]
fn test_e_part2() {
    let input = "EE
EX
EE
EX
EE";
    assert_eq!(part2(input), 8 * 12 + 2 * 4);

    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
    assert_eq!(part2(input), 236);
}

#[test]
fn test_diagonal_part2() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
    assert_eq!(part2(input), 368);
}
