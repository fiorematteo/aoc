use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> String {
    let l = input.lines().next().unwrap().len();
    let mut freq: Vec<HashMap<char, u32>> = vec![HashMap::new(); l];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            *(freq[i].entry(c).or_default()) += 1;
        }
    }
    freq.into_iter()
        .map(|m| m.into_iter().map(|(a, b)| (b, a)).max().unwrap())
        .map(|(_, c)| c)
        .collect()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> String {
    let l = input.lines().next().unwrap().len();
    let mut freq: Vec<HashMap<char, u32>> = vec![HashMap::new(); l];
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            *(freq[i].entry(c).or_default()) += 1;
        }
    }
    freq.into_iter()
        .map(|m| m.into_iter().map(|(a, b)| (b, a)).min().unwrap())
        .map(|(_, c)| c)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(
            super::part1(
                "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"
            ),
            "easter"
        );
    }
}
