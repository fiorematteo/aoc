use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_left(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        };
    }
    fn turn(&mut self, rot: char) {
        match rot {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            _ => unreachable!(),
        }
    }
    fn walk(&self, x: i32) -> (i32, i32) {
        match self {
            Dir::Up => (x, 0),
            Dir::Right => (0, x),
            Dir::Down => (-x, 0),
            Dir::Left => (0, -x),
        }
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut d = Dir::Up;
    let mut position = (0, 0);
    for x in input.split(", ") {
        let mut xs = x.chars();
        let rot = xs.next().unwrap();
        let count = xs.collect::<String>().parse::<i32>().unwrap();
        d.turn(rot);
        let delta = d.walk(count);
        position.0 += delta.0;
        position.1 += delta.1;
    }
    position.0.abs() + position.1.abs()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut d = Dir::Up;
    let mut position = (0_i32, 0_i32);
    let mut seen = HashSet::new();
    for x in input.split(", ") {
        let mut xs = x.chars();
        let rot = xs.next().unwrap();
        let count = xs.collect::<String>().parse::<i32>().unwrap();
        d.turn(rot);
        let delta = d.walk(count);
        for (axis, distance) in [(0, delta.0), (1, delta.1)] {
            let step = distance.signum();
            for _ in 0..distance.abs() {
                if axis == 0 {
                    position.0 += step;
                } else {
                    position.1 += step;
                }

                if !seen.insert(position) {
                    return position.0.abs() + position.1.abs();
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("R2, L3"), 5);
        assert_eq!(super::part1("R2, R2, R2"), 2);
        assert_eq!(super::part1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("R8, R4, R4, R8"), 4);
    }
}
