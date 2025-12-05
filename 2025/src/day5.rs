#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().map(|line| solve(line, 2)).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    input.lines().map(|line| solve(line, 12)).sum()
}

fn solve(line: &str, target_len: usize) -> u64 {
    if line.len() <= target_len {
        return line
            .chars()
            .enumerate()
            .map(|(i, c)| c.to_digit(10).unwrap() as u64 * 10_u64.pow((line.len() - i - 1) as u32))
            .sum();
    }
    let (l, r) = line.split_at(1);
    let r = solve(r, target_len);
    let sr: String = r.to_string();
    let mut best = r;
    for i in 0..sr.len() {
        let mut missing_digit = String::from(&sr[..i]);
        missing_digit.push_str(&sr[i + 1..]);
        let new = l.parse::<u64>().unwrap() * 10_u64.pow((sr.len() - 1) as u32)
            + missing_digit.parse::<u64>().unwrap();
        best = best.max(new);
    }
    best
}

#[test]
fn part1_test() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part1(input), 357);
}

#[test]
fn part2_test() {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    assert_eq!(part2(input), 3121910778619);
}
