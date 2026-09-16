use std::collections::BTreeSet;

fn is_tuple(s: &str, n: usize) -> bool {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        chars.take(n - 1).all(|x| x == c)
    } else {
        false
    }
}

pub fn solve(input: &str, compute: fn(&str) -> String) -> usize {
    let mut set = BTreeSet::new();
    let mut keys = BTreeSet::new();
    for i in 0.. {
        let input = format!("{input}{i}");
        let hash = compute(&input);
        set.retain(|(k, _)| k + 1000 >= i);
        for j in 0..hash.len() - 2 {
            let c = hash[j..j + 1].chars().next().unwrap();
            if is_tuple(&hash[j..j + 3], 3) {
                set.insert((i, c));
                break;
            }
        }
        for j in 0..hash.len() - 4 {
            let c = hash[j..j + 1].chars().next().unwrap();
            if is_tuple(&hash[j..j + 5], 5) {
                for &(k, cc) in &set {
                    if cc == c && k != i {
                        keys.insert(k);
                        continue;
                    }
                }
            }
        }
        if keys.len() >= 64 {
            if let Some(&k) = keys.iter().nth(63) {
                if i >= k + 1000 {
                    return k;
                }
            }
        }
    }
    0
}

fn repeated_hash(input: &str) -> String {
    let mut s = format!("{:?}", md5::compute(input));
    for _ in 0..2016 {
        s = format!("{:?}", md5::compute(s));
    }
    s
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, |s| format!("{:?}", md5::compute(s)))
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, repeated_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("abc"), 22728);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("abc"), 22551);
    }
}
