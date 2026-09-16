#[aoc(day2, part1)]
pub fn part1(input: &str) -> String {
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut pos = (1_i32, 1_i32);
    let mut output = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            match c {
                'U' => pos.0 = (pos.0 - 1).max(0),
                'D' => pos.0 = (pos.0 + 1).min(2),
                'L' => pos.1 = (pos.1 - 1).max(0),
                'R' => pos.1 = (pos.1 + 1).min(2),
                _ => unreachable!(),
            }
        }
        output.push(keypad[pos.0 as usize][pos.1 as usize])
    }
    output.iter().collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let keypad = [
        [' ', ' ', '1', ' ', ' '],
        [' ', '2', '3', '4', ' '],
        ['5', '6', '7', '8', '9'],
        [' ', 'A', 'B', 'C', ' '],
        [' ', ' ', 'D', ' ', ' '],
    ];
    let mut pos = (2_i32, 0_i32);
    let mut output = Vec::new();
    for line in input.lines() {
        for c in line.chars() {
            let next = match c {
                'U' => (pos.0 - 1, pos.1),
                'D' => (pos.0 + 1, pos.1),
                'L' => (pos.0, pos.1 - 1),
                'R' => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            };
            if (0 <= next.0 && next.0 < 5)
                && (0 <= next.1 && next.1 < 5)
                && keypad[next.0 as usize][next.1 as usize] != ' '
            {
                pos = next;
            }
        }
        output.push(keypad[pos.0 as usize][pos.1 as usize]);
    }
    output.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            super::part1(
                "ULL
RRDDD
LURDL
UUUUD"
            ),
            "1985"
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            super::part2(
                "ULL
RRDDD
LURDL
UUUUD"
            ),
            "5DB3"
        );
    }
}
