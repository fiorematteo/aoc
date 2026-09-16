pub fn solve_part1(input: &str, start: &str) -> String {
    let mut password: Vec<_> = String::from(start).chars().collect();
    for line in input.lines() {
        // swap position X with position Y
        if line.starts_with("swap position") {
            let (x, y) = line
                .split_once("position ")
                .unwrap()
                .1
                .split_once(" with position ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (password[y], password[x]) = (password[x], password[y]);
        } else if line.starts_with("swap letter") {
            // swap letter X with letter Y
            let (x, y) = line
                .split_once("letter ")
                .unwrap()
                .1
                .split_once(" with letter ")
                .unwrap();
            let x = x.chars().next().unwrap();
            let y = y.chars().next().unwrap();
            let pp = password.clone();
            let x = pp.iter().position(|c| *c == x).unwrap();
            let y = pp.iter().position(|c| *c == y).unwrap();
            (password[y], password[x]) = (password[x], password[y]);
        } else if line.starts_with("rotate left") {
            // rotate left X steps
            let x = line
                .split_once("rotate left ")
                .unwrap()
                .1
                .split_once(" step")
                .unwrap()
                .0
                .parse::<usize>()
                .unwrap();
            password.rotate_left(x);
        } else if line.starts_with("rotate right") {
            // rotate right X steps
            let x = line
                .split_once("rotate right ")
                .unwrap()
                .1
                .split_once(" step")
                .unwrap()
                .0
                .parse::<usize>()
                .unwrap();
            password.rotate_right(x);
        } else if line.starts_with("rotate based") {
            // rotate based on position of letter X
            let x = line
                .split_once("letter ")
                .unwrap()
                .1
                .chars()
                .next()
                .unwrap();
            let x = password.iter().position(|c| *c == x).unwrap();
            let x = if x >= 4 { x + 2 } else { x + 1 };
            let x = x % password.len();
            password.rotate_right(x);
        } else if line.starts_with("reverse positions") {
            // reverse positions X through Y
            let (x, y) = line
                .split_once("positions ")
                .unwrap()
                .1
                .split_once(" through ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            assert!(y >= x);
            let pp = password.clone();
            let cc = pp[x..=y].iter().rev();
            for (&c, i) in cc.zip(x..=y) {
                password[i] = c;
            }
        } else if line.starts_with("move position") {
            // move position X to position Y
            let (x, y) = line
                .split_once("move position ")
                .unwrap()
                .1
                .split_once(" to position ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            let c = password.remove(x);
            password.insert(y, c);
        }
    }
    password.iter().collect()
}

pub fn solve_part2(input: &str, start: &str) -> String {
    let mut password: Vec<_> = String::from(start).chars().collect();
    for line in input.lines().rev() {
        // swap position X with position Y
        if line.starts_with("swap position") {
            let (x, y) = line
                .split_once("position ")
                .unwrap()
                .1
                .split_once(" with position ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            (password[y], password[x]) = (password[x], password[y]);
        } else if line.starts_with("swap letter") {
            // swap letter X with letter Y
            let (x, y) = line
                .split_once("letter ")
                .unwrap()
                .1
                .split_once(" with letter ")
                .unwrap();
            let x = x.chars().next().unwrap();
            let y = y.chars().next().unwrap();
            let pp = password.clone();
            let x = pp.iter().position(|c| *c == x).unwrap();
            let y = pp.iter().position(|c| *c == y).unwrap();
            (password[y], password[x]) = (password[x], password[y]);
        } else if line.starts_with("rotate left") {
            // rotate left X steps
            let x = line
                .split_once("rotate left ")
                .unwrap()
                .1
                .split_once(" step")
                .unwrap()
                .0
                .parse::<usize>()
                .unwrap();
            password.rotate_right(x);
        } else if line.starts_with("rotate right") {
            // rotate right X steps
            let x = line
                .split_once("rotate right ")
                .unwrap()
                .1
                .split_once(" step")
                .unwrap()
                .0
                .parse::<usize>()
                .unwrap();
            password.rotate_left(x);
        } else if line.starts_with("rotate based") {
            // rotate based on position of letter X
            let x = line
                .split_once("letter ")
                .unwrap()
                .1
                .chars()
                .next()
                .unwrap();

            let len = password.len();
            for r in 0..len {
                let mut candidate = password.clone();
                candidate.rotate_left(r);
                let idx = candidate.iter().position(|c| *c == x).unwrap();
                let rot = (idx + if idx >= 4 { 2 } else { 1 }) % len;
                if rot == r {
                    password.rotate_left(r);
                    break;
                }
            }
        } else if line.starts_with("reverse positions") {
            // reverse positions X through Y
            let (x, y) = line
                .split_once("positions ")
                .unwrap()
                .1
                .split_once(" through ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            assert!(y >= x);
            let pp = password.clone();
            let cc = pp[x..=y].iter().rev();
            for (&c, i) in cc.zip(x..=y) {
                password[i] = c;
            }
        } else if line.starts_with("move position") {
            // move position X to position Y
            let (x, y) = line
                .split_once("move position ")
                .unwrap()
                .1
                .split_once(" to position ")
                .unwrap();
            let x: usize = x.parse().unwrap();
            let y: usize = y.parse().unwrap();
            let c = password.remove(y);
            password.insert(x, c);
        }
    }
    password.iter().collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> String {
    solve_part1(input, "abcdefgh")
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> String {
    solve_part2(input, "fbgdceah")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let out = super::solve_part1(
            "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d",
            "abcde",
        );
        assert_eq!(out, "decab")
    }

    #[test]
    fn part2() {
        // let input = "swap position 4 with position 0
        // swap letter d with letter b
        // reverse positions 0 through 4
        // rotate left 1 step
        // move position 1 to position 4
        // move position 3 to position 0
        // rotate based on position of letter b
        // rotate based on position of letter d";
        // let p0 = "abcde";
        // let p1 = super::solve_part1(input, p0);
        // let p2 = super::solve_part2(input, &p1);
        // assert_eq!(p0, p2);

        // t("swap position 4 with position 0");
        // t("swap letter d with letter b");
        // t("reverse positions 0 through 4");
        // t("rotate left 1 step");
        // t("move position 1 to position 4");
        // t("move position 3 to position 0");
        t("rotate based on position of letter c");
        //t("rotate based on position of letter d");

        fn t(input: &str) {
            let p0 = "qwertyuiopabcde";
            let p1 = super::solve_part1(input, p0);
            let p2 = super::solve_part2(input, &p1);
            dbg!(&p0, &p1, &p2);
            assert_eq!(p0, p2);
        }
    }
}
