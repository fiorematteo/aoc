#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>();
    ids.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|&(start, _)| start);
    let mut i = 0;
    while i < ranges.len() - 1 {
        let (a, b) = (ranges[i], ranges[i + 1]);
        if b.0 <= a.1 {
            if b.1 > a.1 {
                ranges[i + 1].0 = a.1 + 1;
            } else {
                ranges.remove(i + 1);
                continue;
            }
        }
        i += 1;
    }
    ranges.into_iter().map(|(start, end)| end - start + 1).sum()
}

#[test]
fn part1_test() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part1(input), 3);
}

#[test]
fn part2_test() {
    let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    assert_eq!(part2(input), 14);

    let input = "1-10
4-8
3-5

1
5
8
11
17
32";
    assert_eq!(part2(input), 10);
}
