use std::collections::{HashSet, VecDeque};

const N: usize = 8;

fn bfs(x: usize, y: usize, map: &[Vec<i32>]) -> [u32; N] {
    let current_number = map[y][x];
    assert!((0..=7).contains(&current_number));
    let mut dist = [0; N];
    let mut visited = HashSet::new();
    visited.insert((y, x));
    let mut queue = VecDeque::new();
    queue.push_front((0, (y, x)));
    while let Some((depth, (y, x))) = queue.pop_front() {
        if map[y][x] >= 0 {
            if dist[map[y][x] as usize] == 0 {
                dist[map[y][x] as usize] = depth;
            }
            if dist.iter().filter(|&&x| x == 0).count() == 1 {
                break;
            }
        }
        for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;
            if map[ny][nx] == -2 {
                continue;
            }
            if !visited.insert((ny, nx)) {
                continue;
            }
            queue.push_back((depth + 1, (ny, nx)));
        }
    }
    dist
}

fn build_dist(input: &str) -> [[u32; N]; N] {
    let map: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '#' => -2,
                    '.' => -1,
                    d if d.is_ascii_digit() => d.to_digit(10).unwrap() as i32,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let mut dist = [[0_u32; N]; N];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] >= 0 {
                dist[map[y][x] as usize] = bfs(x, y, &map);
            }
        }
    }
    dist
}

fn solve_dp(dist: [[u32; N]; N]) -> [[u32; N]; 1 << N] {
    let mut dp = [[u32::MAX; N]; 1 << N];
    dp[1][0] = 0;
    for mask in 0..=(1 << N) - 1 {
        for i in 0..N {
            let current = dp[mask][i];
            if current == u32::MAX {
                continue;
            }
            for j in 0..N {
                if mask & (1 << j) != 0 {
                    continue;
                }
                let new_mask = mask | (1 << j);
                dp[new_mask][j] = dp[new_mask][j].min(current + dist[i][j]);
            }
        }
    }

    dp
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u32 {
    let dist = build_dist(input);
    let dp = solve_dp(dist);
    (0..N).map(|i| dp[(1 << N) - 1][i]).min().unwrap()
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> u32 {
    let dist = build_dist(input);
    let dp = solve_dp(dist);
    (0..N)
        .map(|i| dp[(1 << N) - 1][i] + dist[i][0])
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {}
}
