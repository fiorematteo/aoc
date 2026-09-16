pub fn solve(input: &str, rows: usize) -> usize {
    let mut safe_count = 0;
    let mut row_count = 1;
    let mut row: Vec<_> = input.chars().map(|c| c == '.').collect();
    while row_count <= rows {
        safe_count += row.iter().filter(|&&x| x).count();
        row = generate_row(&row);
        row_count += 1;
    }
    safe_count
}

fn generate_row(row: &[bool]) -> Vec<bool> {
    let mut next_row = vec![false; row.len()];
    for (i, b) in next_row.iter_mut().enumerate() {
        let l = if i == 0 { true } else { row[i - 1] };
        let c = row[i];
        let r = if i == row.len() - 1 { true } else { row[i + 1] };
        // *b = !((!l && !c && r) || (l && !c && !r) || (!l && c && r) || (l && c && !r));
        // *b = !(!l && !c && r) && !(l && !c && !r) && !(!l && c && r) && !(l && c && !r);
        // *b = (l || c || !r) && (!l || c || r) && (l || !c || !r) && (!l || !c || r);
        *b = c || l && r || !l && !r;
    }
    next_row
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 400000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            generate_row(&[true, true, false, false, true]),
            &[true, false, false, false, false]
        );
        assert_eq!(
            generate_row(&[true, false, false, false, false]),
            &[false, false, true, true, false]
        );
        assert_eq!(solve(".^^.^.^^^^", 10), 38);
    }

    #[test]
    fn part2() {}
}
