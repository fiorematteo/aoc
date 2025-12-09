#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
    let mut largest_area = 0;
    for &a in &tiles {
        for &b in &tiles {
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            largest_area = largest_area.max(area);
        }
    }
    largest_area
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let mut tiles = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
    tiles.push(tiles[0]);
    let mut largest_area = 0;
    for &a in &tiles {
        for &b in &tiles {
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            if area > largest_area && !contains_edges(a, b, &tiles) {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn contains_edges(a1: (usize, usize), a2: (usize, usize), tiles: &[(usize, usize)]) -> bool {
    let (a_min, a_max) = (
        (a1.0.min(a2.0), a1.1.min(a2.1)),
        (a1.0.max(a2.0), a1.1.max(a2.1)),
    );
    for pair in tiles.windows(2) {
        let &[b1, b2] = pair else { unreachable!() };
        let (b_min, b_max) = (
            (b1.0.min(b2.0), b1.1.min(b2.1)),
            (b1.0.max(b2.0), b1.1.max(b2.1)),
        );
        // aabb
        if a_min.0 < b_max.0 && a_max.0 > b_min.0 && a_min.1 < b_max.1 && a_max.1 > b_min.1 {
            return true;
        }
    }
    false
}

#[test]
fn part1_test() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    assert_eq!(part1(input), 50);
}

#[test]
fn part2_test() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    assert_eq!(part2(input), 24);
}
