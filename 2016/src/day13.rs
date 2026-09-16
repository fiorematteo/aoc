use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn is_wall(x: isize, y: isize) -> bool {
    // Find x*x + 3*x + 2*x*y + y + y*y.
    // Add the office designer's favorite number (your puzzle input).
    // Find the binary representation of that sum; count the number of bits that are 1.
    //     If the number of bits that are 1 is even, it's an open space.
    //     If the number of bits that are 1 is odd, it's a wall.
    let magic_number = 1364;
    let k = (x * x) + (3 * x) + (2 * x * y) + y + (y * y) + magic_number;
    k.count_ones() % 2 == 1
}

#[aoc(day13, part1)]
pub fn part1(_input: &str) -> usize {
    let start = (1, 1);
    let target = (31, 39); // x y
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    let mut dist = HashMap::new();

    while let Some((Reverse(d), (x, y))) = heap.pop() {
        if dist.contains_key(&(x, y)) {
            continue;
        }
        dist.insert((x, y), d);
        if (x, y) == target {
            return d;
        }
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            if x < 0 || y < 0 || is_wall(x, y) {
                continue;
            }
            heap.push((Reverse(d + 1), (x, y)));
        }
    }
    unreachable!()
}

#[aoc(day13, part2)]
pub fn part2(_input: &str) -> usize {
    let start = (1, 1);
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), start));
    let mut dist = HashMap::new();

    while let Some((Reverse(d), (x, y))) = heap.pop() {
        if dist.contains_key(&(x, y)) {
            continue;
        }
        dist.insert((x, y), d);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            if x < 0 || y < 0 || is_wall(x, y) || d == 50 {
                continue;
            }
            heap.push((Reverse(d + 1), (x, y)));
        }
    }
    dist.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("x"), 42)
    }

    #[test]
    fn part2() {}
}
