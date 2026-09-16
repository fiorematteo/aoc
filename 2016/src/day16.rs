use itertools::Itertools;

fn checksum(data: &str) -> String {
    data.chars()
        .chunks(2)
        .into_iter()
        .map(|mut x| {
            let a = x.next().unwrap();
            let b = x.next().unwrap();
            if a == b {
                '1'
            } else {
                '0'
            }
        })
        .collect()
}

fn solve(input: &str, disk_len: usize) -> String {
    let mut data = input.to_string();
    while data.len() < disk_len {
        data = rev_comp(data);
    }
    let mut check = checksum(&data[..disk_len]);
    while check.len().is_multiple_of(2) {
        check = checksum(&check);
    }
    check
}

fn rev_comp(data: impl Into<String>) -> String {
    let data = data.into();
    let rev: String = data
        .chars()
        .rev()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();
    data + "0" + &rev
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    solve(input, 272)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    solve(input, 35651584)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(checksum("11"), "1");
        assert_eq!(checksum("00"), "1");
        assert_eq!(checksum("10"), "0");
        assert_eq!(checksum("01"), "0");
        assert_eq!(checksum("1100"), "11");
        assert_eq!(checksum("0010"), "10");
        assert_eq!(checksum("1001"), "00");
        assert_eq!(checksum("0111"), "01");
        assert_eq!(rev_comp("1"), "100");
        assert_eq!(rev_comp("0"), "001");
        assert_eq!(rev_comp("11111"), "11111000000");
        assert_eq!(rev_comp("111100001010"), "1111000010100101011110000");
        assert_eq!(solve("110010110100", 12), "100");
        assert_eq!(solve("10000", 20), "01100");
    }

    #[test]
    fn part2() {}
}
