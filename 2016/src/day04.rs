use std::collections::HashMap;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut out = 0;
    for line in input.lines() {
        let line = line.split("-").collect::<Vec<_>>();
        let (room_id, checksum) = line.last().unwrap().split_once("[").unwrap();
        let room_id = room_id.parse::<usize>().unwrap();
        let checksum = checksum.strip_suffix("]").unwrap();

        let mut map = HashMap::new();
        for c in line[0..line.len() - 1].iter().flat_map(|s| s.chars()) {
            let count = map.entry(c).or_default();
            *count += 1;
        }
        let mut map = map
            .into_iter()
            .map(|(c, x): (char, i32)| (-x, c))
            .collect::<Vec<_>>();
        map.sort();
        if checksum
            .chars()
            .zip(map.into_iter())
            .all(|(x, (_, y))| x == y)
        {
            out += room_id
        }
    }
    out
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    for line in input.lines() {
        let line = line.split("-").collect::<Vec<_>>();
        let (room_id, _) = line.last().unwrap().split_once("[").unwrap();
        let room_id = room_id.parse::<usize>().unwrap();
        for word in &line[0..line.len() - 1] {
            for c in word.chars() {
                print!("{}", rotate(c, room_id));
            }
            print!(" ");
        }
        println!(" {}", room_id);
    }
    0
}

fn rotate(c: char, x: usize) -> char {
    let alpha = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let i = alpha.iter().position(|&cc| cc == c).unwrap();
    alpha[(i + x) % alpha.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
    }
}
