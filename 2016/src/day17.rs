use md5::compute;
use std::collections::VecDeque;

const DIRS: [(char, isize, isize); 4] = [('U', 0, -1), ('D', 0, 1), ('L', -1, 0), ('R', 1, 0)];

fn is_open(s: impl AsRef<str>, i: usize) -> bool {
    let hash = compute(s.as_ref());
    let s = format!("{:?}", hash);
    let c = s.chars().nth(i).unwrap();
    c == 'b' || c == 'c' || c == 'd' || c == 'e' || c == 'f'
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let prefix = input;
    let target = (3, 3);
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), String::new()));

    while let Some(((x, y), path)) = queue.pop_front() {
        if (x, y) == target {
            return path;
        }

        for (i, &(c, dx, dy)) in DIRS.iter().enumerate() {
            let nx = x + dx;
            let ny = y + dy;

            if !(0..4).contains(&nx) || !(0..4).contains(&ny) {
                continue;
            }

            if is_open(format!("{prefix}{path}"), i) {
                let mut next_path = path.clone();
                next_path.push(c);
                queue.push_back(((nx, ny), next_path));
            }
        }
    }

    unreachable!()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let prefix = input;
    let target = (3, 3);
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), String::new()));

    let mut longest_path = 0;

    while let Some(((x, y), path)) = queue.pop_front() {
        if (x, y) == target {
            longest_path = longest_path.max(path.len());
            continue;
        }

        for (i, &(c, dx, dy)) in DIRS.iter().enumerate() {
            let nx = x + dx;
            let ny = y + dy;

            if !(0..4).contains(&nx) || !(0..4).contains(&ny) {
                continue;
            }

            if is_open(format!("{prefix}{path}"), i) {
                let mut next_path = path.clone();
                next_path.push(c);
                queue.push_back(((nx, ny), next_path));
            }
        }
    }

    longest_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("ihgpwlah"), "DDRRRD");
        assert_eq!(super::part1("kglvqrro"), "DDUDRLRRUDRD");
        assert_eq!(super::part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }

    #[test]
    fn part2() {}
}
