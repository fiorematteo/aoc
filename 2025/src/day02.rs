use fancy_regex::Regex;

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|interval| {
            let (f, t) = interval.split_once("-").unwrap();
            (f.parse::<u64>().unwrap(), t.parse::<u64>().unwrap())
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(u64, u64)]) -> u64 {
    let mut total = 0;
    for &(f, t) in input {
        for i in f..=t {
            let si = i.to_string();
            let m = si.len() / 2;
            if si[..m] == si[m..] {
                total += i;
            }
        }
    }
    total
}

#[aoc(day2, part2)]
pub fn part2(input: &[(u64, u64)]) -> u64 {
    let re = Regex::new(r"^(.*)\1+$").unwrap();
    let mut total = 0;
    for &(f, t) in input {
        for i in f..=t {
            let si = i.to_string();
            if re.is_match(&si).unwrap() {
                total += i;
            }
        }
    }
    total
}

#[test]
fn part1_test() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part1(&gen(input)), 1227775554);
}

#[test]
fn part2_test() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(part2(&gen(input)), 4174379265);
}
