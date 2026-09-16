use itertools::Itertools;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let [a, b, c] = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (a, b, c)
        })
        .filter(|&(x, y, z)| (x + y > z) && (x + z > y) && (z + y > x))
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let iter = input.lines().map(|line| {
        let [a, b, c] = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        (a, b, c)
    });
    iter.clone()
        .map(|(x, _, _)| x)
        .chain(iter.clone().map(|(_, y, _)| y))
        .chain(iter.map(|(_, _, z)| z))
        .chunks(3)
        .into_iter()
        .filter_map(|mut chunk| Some((chunk.next()?, chunk.next()?, chunk.next()?)))
        .filter(|&(x, y, z)| (x + y > z) && (x + z > y) && (z + y > x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("5 10 25"), 0);
    }
}
