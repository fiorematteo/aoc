use std::cmp;

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    solve1(input, 1000)
}

pub fn solve1(input: &str, count: usize) -> usize {
    let boxes = input
        .lines()
        .map(|line| {
            let l = line
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (l[0], l[1], l[2])
        })
        .collect::<Vec<_>>();
    let mut pairs = vec![];
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            let dx = boxes[i].0 - boxes[j].0;
            let dy = boxes[i].1 - boxes[j].1;
            let dz = boxes[i].2 - boxes[j].2;
            let d = dx.pow(2) + dy.pow(2) + dz.pow(2);
            pairs.push((i, j, d));
        }
    }
    pairs.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
    pairs.dedup();

    let mut edges = vec![vec![]; boxes.len()];
    for pair in pairs.into_iter().take(count * 2).map(|(a, b, _)| (a, b)) {
        edges[pair.0].push(pair.1);
    }

    let mut components = get_components(&edges)
        .drain(..)
        .filter(|c| c.len() > 1)
        .collect::<Vec<_>>();

    components.sort_by_key(|a| cmp::Reverse(a.len()));
    components[0].len() * components[1].len() * components[2].len()
}

fn dfs_lite(adj: &[Vec<usize>], visited: &mut [bool], s: usize) {
    visited[s] = true;

    for &i in &adj[s] {
        if !visited[i] {
            dfs_lite(adj, visited, i);
        }
    }
}

fn dfs(adj: &[Vec<usize>], visited: &mut [bool], s: usize, res: &mut Vec<usize>) {
    visited[s] = true;
    res.push(s);

    for &i in &adj[s] {
        if !visited[i] {
            dfs(adj, visited, i, res);
        }
    }
}

fn get_components(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let v = adj.len();
    let mut visited = vec![false; v];
    let mut res = vec![];
    for i in 0..v {
        if !visited[i] {
            let mut component = vec![];
            dfs(adj, &mut visited, i, &mut component);
            res.push(component);
        }
    }
    res
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let boxes = input
        .lines()
        .map(|line| {
            let l = line
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (l[0], l[1], l[2])
        })
        .collect::<Vec<_>>();
    let mut pairs = vec![];
    for i in 0..boxes.len() {
        for j in 0..boxes.len() {
            if i == j {
                continue;
            }
            let dx = boxes[i].0 - boxes[j].0;
            let dy = boxes[i].1 - boxes[j].1;
            let dz = boxes[i].2 - boxes[j].2;
            let d = dx.pow(2) + dy.pow(2) + dz.pow(2);
            pairs.push((i, j, d));
        }
    }
    pairs.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
    pairs.dedup();

    let mut edges = vec![vec![]; boxes.len()];
    for pair in pairs.into_iter().map(|(a, b, _)| (a, b)) {
        edges[pair.0].push(pair.1);

        let mut visited = vec![false; boxes.len()];
        dfs_lite(&edges, &mut visited, 0);
        if visited.iter().all(|&x| x) {
            return (boxes[pair.0].0 * boxes[pair.1].0) as usize;
        }
    }
    unreachable!()
}

#[test]
fn part1_test() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(solve1(input, 10), 40);
}

#[test]
fn part2_test() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(part2(input), 25272);
}
