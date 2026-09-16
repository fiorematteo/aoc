#[aoc(day7, part1)]
pub fn part1(input: &str) -> i32 {
    let mut out = 0;
    for line in input.lines() {
        let mut hypernet = Vec::new();
        let mut supernet = Vec::new();
        let mut is_supernet = true;
        let mut last = 0;
        for i in 0..line.len() {
            if &line[i..i + 1] == "[" && is_supernet {
                is_supernet = false;
                supernet.push(&line[last..i]);
                last = i + 1;
                continue;
            }
            if &line[i..i + 1] == "]" && !is_supernet {
                is_supernet = true;
                hypernet.push(&line[last..i]);
                last = i + 1;
                continue;
            }
        }
        if last != line.len() {
            supernet.push(&line[last..]);
        }

        let mut flag = false;
        for s in &supernet {
            for i in 0..s.len() - 4 {
                let first = &s[i..i + 1];
                let second = &s[i + 1..i + 2];
                let third = &s[i + 2..i + 3];
                let fourth = &s[i + 3..i + 4];
                if first == fourth && second == third && first != second {
                    flag = true;
                    break;
                }
            }
            if flag {
                out += 1;
                break;
            }
        }
    }
    out
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i32 {
    let mut out = 0;
    for line in input.lines() {
        let mut hypernet = Vec::new();
        let mut supernet = Vec::new();
        let mut is_supernet = true;
        let mut last = 0;
        for i in 0..line.len() {
            if &line[i..i + 1] == "[" && is_supernet {
                is_supernet = false;
                supernet.push(&line[last..i]);
                last = i + 1;
                continue;
            }
            if &line[i..i + 1] == "]" && !is_supernet {
                is_supernet = true;
                hypernet.push(&line[last..i]);
                last = i + 1;
                continue;
            }
        }
        if last != line.len() {
            supernet.push(&line[last..]);
        }

        let mut possible = Vec::new();
        for s in supernet {
            for i in 0..s.len() - 2 {
                let first = &s[i..i + 1];
                let second = &s[i + 1..i + 2];
                let third = &s[i + 2..i + 3];
                if first == third && first != second {
                    possible.push(&s[i..i + 3]);
                }
            }
        }
        for p in &possible {
            let rev = format!("{}{}{}", &p[1..2], &p[0..1], &p[1..2]);
            if hypernet.iter().any(|h| h.contains(&rev)) {
                out += 1;
                break;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(
                "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb"
            ),
            3
        );
    }
}
