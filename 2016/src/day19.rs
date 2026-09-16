#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let mut elfs: Vec<_> = (0..input.parse::<usize>().unwrap())
        .map(Option::Some)
        .collect();
    let mut elfs_left = elfs.len();
    let mut i = 0;
    while elfs_left > 1 {
        i = (i + 1) % elfs.len();
        while elfs[i].is_none() {
            i = (i + 1) % elfs.len();
        }
        elfs[i] = None;
        elfs_left -= 1;
        while elfs[i].is_none() {
            i = (i + 1) % elfs.len();
        }
    }
    elfs.into_iter().find(|e| e.is_some()).unwrap().unwrap() + 1
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let n = input.parse::<usize>().unwrap();
    let mut next: Vec<usize> = (0..n).map(|i| (i + 1) % n).collect();
    let mut current = 0;
    let mut opposite = n / 2 - 1;
    let mut remaining = n;
    while remaining > 1 {
        next[opposite] = next[next[opposite]];
        remaining -= 1;
        current = next[current];
        if remaining % 2 == 0 {
            opposite = next[opposite];
        }
    }
    current + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("5"), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("5"), 2);
    }
}
