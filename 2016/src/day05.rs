use md5::compute;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let mut password = String::new();
    for i in 0.. {
        let x = input.to_owned() + &i.to_string();
        let hash = compute(&x);
        let hash = format!("{:?}", hash);
        if hash.starts_with("00000") {
            let c = hash.chars().nth(5).unwrap();
            password.push(c);
            if password.len() == 8 {
                break;
            }
        }
    }
    password
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let mut password = ['x'; 8];
    for i in 0.. {
        let x = input.to_owned() + &i.to_string();
        let hash = compute(&x);
        let hash = format!("{:?}", hash);
        if hash.starts_with("00000") {
            let mut hash = hash.chars().skip(5);
            let Some(index) = hash.next() else {
                continue;
            };
            let Some(index) = index.to_digit(10) else {
                continue;
            };
            if index >= 8 {
                continue;
            }
            let c = hash.next().unwrap();
            if password[index as usize] == 'x' {
                password[index as usize] = c;
                if !password.contains(&'x') {
                    break;
                }
            }
        }
    }
    password.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1("abc"), "18f47a30");
    }
}
