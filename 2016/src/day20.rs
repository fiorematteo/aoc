fn try_merge(a: &(usize, usize), b: &(usize, usize)) -> Option<(usize, usize)> {
    if a.1 + 1 >= b.0 {
        Some((a.0.min(b.0), b.1.max(a.1)))
    } else {
        None
    }
}

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|&(_, x)| x);
    ranges.sort_by_key(|&(x, _)| x);
    ranges.into_iter().fold(Vec::new(), |mut acc, item| {
        if let Some(last) = acc.pop() {
            if let Some(m) = try_merge(&last, &item) {
                acc.push(m);
            } else {
                acc.push(last);
                acc.push(item);
            }
        } else {
            acc.push(item);
        }
        acc
    })
}

#[aoc(day20, part1)]
pub fn part1(ranges: &[(usize, usize)]) -> usize {
    ranges[0].1 + 1
}

#[aoc(day20, part2)]
pub fn part2(ranges: &[(usize, usize)]) -> usize {
    let mut out = 0;
    for i in 0..ranges.len() - 1 {
        let (l, r) = (ranges[i], ranges[i + 1]);
        out += r.0 - l.1 - 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            super::part1(&generator(
                "5-8
0-2
4-7"
            )),
            3
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(&generator(
                "5-8
0-2
4-7"
            )),
            1
        );
    }
}
