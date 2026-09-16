use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    size: usize,
    used: usize,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Hash)]
enum MatrixNode {
    Empty,
    Goal,
    Data,
    Wall,
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let nodes: Vec<_> = input
        .lines()
        .skip(2)
        .map(|line| {
            let mut parts = line.split_whitespace().skip(1);
            let size = parts.next().unwrap();
            let used = parts.next().unwrap();
            let size = size.strip_suffix("T").unwrap().parse().unwrap();
            let used = used.strip_suffix("T").unwrap().parse().unwrap();
            Node { size, used }
        })
        .collect();

    let mut count = 0;
    for a in &nodes {
        for b in &nodes {
            if a.used != 0 && a != b && a.used <= (b.size - b.used) {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    let mut matrix = Vec::new();
    let mut highest_x = 0;
    let mut highest_y = 0;

    let mut empty = (0, 0);

    for line in input.lines().skip(2) {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let used = parts.nth(1).unwrap();

        let (x, y) = name
            .split_once("node-x")
            .unwrap()
            .1
            .split_once("-y")
            .unwrap();

        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        let used: usize = used.strip_suffix("T").unwrap().parse().unwrap();
        while matrix.len() <= y {
            matrix.push(Vec::new());
        }
        while matrix[y].len() <= x {
            matrix[y].push(MatrixNode::Empty);
        }

        matrix[y][x] = if used == 0 {
            empty = (y, x);
            MatrixNode::Empty
        } else if used > 100 {
            MatrixNode::Wall
        } else {
            MatrixNode::Data
        };

        highest_x = highest_x.max(x);
        highest_y = highest_y.max(y);
    }
    for row in &mut matrix {
        row.resize(highest_x + 1, MatrixNode::Empty);
    }

    matrix[0][highest_x] = MatrixNode::Goal;

    assert_eq!(
        1,
        matrix
            .iter()
            .flat_map(|x| x.iter())
            .filter(|x| matches!(x, MatrixNode::Empty))
            .count()
    );

    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), empty, (0, highest_x))]);
    while let Some((Reverse(steps), empty, goal)) = queue.pop() {
        if goal == (0, 0) {
            return steps;
        }
        let (y, x) = empty;
        for (yy, xx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let yy = y as isize + yy;
            if yy < 0 || yy > highest_y as isize {
                continue;
            }
            let xx = x as isize + xx;
            if xx < 0 || xx > highest_x as isize {
                continue;
            }
            let (yy, xx) = (yy as usize, xx as usize);
            let nn = &matrix[yy][xx];
            if matches!(nn, MatrixNode::Wall) {
                continue;
            }
            let new_empty = (yy, xx);
            let new_goal = if new_empty == goal { empty } else { goal };
            let new_state = (new_empty, new_goal);
            if visited.insert(new_state) {
                queue.push((Reverse(steps + 1), new_empty, new_goal));
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part2() {
        let out = super::part2(
            "root@ebhq-gridcenter# df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%",
        );
        assert_eq!(out, 7)
    }
}
