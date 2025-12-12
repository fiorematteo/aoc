#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let sections: Vec<_> = input.split("\n\n").collect();
    // let pieces: Vec<_> = sections[..sections.len() - 1]
    //     .iter()
    //     .map(|&s| Piece::from_str(s))
    //     .collect();

    let mut total = 0;
    for line in sections[sections.len() - 1].lines() {
        let (l, r) = line.split_once(": ").unwrap();
        let (x_size, y_size) = l.split_once('x').unwrap();
        let x_size: usize = x_size.parse().unwrap();
        let y_size: usize = y_size.parse().unwrap();
        /*
        let grid = vec![vec![false; x_size]; y_size];
        let pieces_to_fit = r
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .enumerate()
            .flat_map(|(i, count)| vec![pieces[i].clone(); count])
            .collect::<Vec<_>>();

         if can_fit(grid, pieces_to_fit) {
             total += 1;
         }
        */
        let area = x_size * y_size;
        let needed_area: usize = r
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap() * 3 * 3)
            .sum();
        if needed_area <= area {
            total += 1;
        }
    }
    total
}

/*
fn can_fit(grid: Vec<Vec<bool>>, mut pieces_to_fit: Vec<Piece>) -> bool {
    let needed_spaces = pieces_to_fit
        .iter()
        .map(|p| p.grid.iter().flatten().filter(|&&b| b).count())
        .sum::<usize>();
    let empty_spaces = grid.iter().flatten().filter(|&&b| !b).count();
    if needed_spaces > empty_spaces {
        return false;
    }
    let Some(next_piece) = pieces_to_fit.pop() else {
        return true;
    };

    let mut permutations = vec![
        next_piece.clone(),
        next_piece.rotate(),
        next_piece.rotate().rotate(),
        next_piece.rotate().rotate().rotate(),
        next_piece.flip(),
        next_piece.flip().rotate(),
        next_piece.flip().rotate().rotate(),
        next_piece.flip().rotate().rotate().rotate(),
    ];
    permutations.sort_by(|a, b| a.grid.cmp(&b.grid));
    permutations.dedup_by(|a, b| a.grid == b.grid);

    let mut solutions = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for piece in &permutations {
                if piece.collision(&grid, x, y) {
                    continue;
                }
                let mut new_grid = grid.clone();
                for r in 0..piece.grid.len() {
                    for c in 0..piece.grid[0].len() {
                        if piece.grid[r][c] {
                            new_grid[y + r][x + c] = true;
                        }
                    }
                }
                solutions.push(new_grid);
            }
        }
    }

    solutions
        .into_iter()
        .any(|g| can_fit(g, pieces_to_fit.clone()))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Piece {
    grid: Vec<Vec<bool>>,
}

impl Piece {
    fn from_str(s: &str) -> Self {
        let grid = s
            .lines()
            .skip(1)
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Piece { grid }
    }

    fn flip(&self) -> Self {
        let grid = self
            .grid
            .iter()
            .map(|row| row.iter().cloned().rev().collect())
            .collect();
        Piece { grid }
    }

    fn rotate(&self) -> Self {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut new_grid = vec![vec![false; rows]; cols];
        for r in 0..rows {
            for c in 0..cols {
                new_grid[c][rows - r - 1] = self.grid[r][c];
            }
        }
        Piece { grid: new_grid }
    }

    fn collision(&self, grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
        if y + self.grid.len() > grid.len() || x + self.grid[0].len() > grid[0].len() {
            return true;
        }
        for r in 0..self.grid.len() {
            for c in 0..self.grid[0].len() {
                if self.grid[r][c] && grid[y + r][x + c] {
                    return true;
                }
            }
        }
        false
    }
}
*/
