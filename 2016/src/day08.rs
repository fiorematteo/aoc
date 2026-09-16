struct Screen([[bool; 50]; 6]);

impl Screen {
    fn new() -> Self {
        Self([[false; 50]; 6])
    }

    fn rect(&mut self, a: usize, b: usize) {
        for y in 0..b {
            for x in 0..a {
                self.0[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, a: usize, b: usize) {
        let row = self.0[a];
        for (i, pixel) in (0..50).zip(row.into_iter()) {
            self.0[a][(i + b) % 50] = pixel;
        }
    }

    fn rotate_col(&mut self, a: usize, b: usize) {
        let col: Vec<_> = self.0.iter().map(|row| row[a]).collect();
        for (i, pixel) in (0..6).zip(col.into_iter()) {
            self.0[(i + b) % 6][a] = pixel;
        }
    }

    fn print(&self) {
        for r in self.0 {
            for p in r {
                print!("{}", if p { "#" } else { " " });
            }
            println!();
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let screen = solve(input);
    screen
        .0
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|&x| x)
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let screen = solve(input);
    screen.print();
    0
}

fn solve(input: &str) -> Screen {
    let mut screen = Screen::new();
    for line in input.lines() {
        if line.starts_with("rect") {
            let (a, b) = line.split_once(" ").unwrap().1.split_once("x").unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            screen.rect(a, b);
        } else if line.starts_with("rotate row") {
            let (a, b) = line.split_once("y=").unwrap().1.split_once(" by ").unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            screen.rotate_row(a, b);
        } else if line.starts_with("rotate col") {
            let (a, b) = line.split_once("x=").unwrap().1.split_once(" by ").unwrap();
            let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
            screen.rotate_col(a, b);
        } else {
            unreachable!()
        }
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
    }
}
