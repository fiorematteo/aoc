use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Floor {
    gens: u16,
    chips: u16,
}

impl Floor {
    fn valid(self) -> bool {
        // no generators => chips cannot be fried
        self.gens == 0 || (self.chips & !self.gens) == 0
    }

    fn empty(self) -> bool {
        self.gens == 0 && self.chips == 0
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    elevator: usize,
    floors: [Floor; 4],
}

#[derive(Clone, Copy)]
enum Item {
    Gen(u16),
    Chip(u16),
}

fn items(f: Floor) -> Vec<Item> {
    let mut out = Vec::new();

    for i in 0..16 {
        let bit = 1 << i;

        if f.gens & bit != 0 {
            out.push(Item::Gen(bit));
        }

        if f.chips & bit != 0 {
            out.push(Item::Chip(bit));
        }
    }

    out
}

fn shortest_steps(start: State) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((start, 0));
    seen.insert(start);

    while let Some((state, steps)) = queue.pop_front() {
        if state.floors[..3].iter().all(|f| f.empty()) {
            return Some(steps);
        }

        let current = state.elevator;
        let available = items(state.floors[current]);

        let moves = available.iter().enumerate().flat_map(|(i, &a)| {
            available[i + 1..]
                .iter()
                .map(move |&b| Some((a, b)))
                .chain(std::iter::once(None).map(move |_: Option<(usize, usize)>| Some((a, a))))
        });

        for dir in [-1isize, 1] {
            let next_floor = current as isize + dir;

            if !(0..4).contains(&next_floor) {
                continue;
            }

            let next_floor = next_floor as usize;

            for m in moves.clone().flatten() {
                let mut next = state;

                next.elevator = next_floor;

                let mut remove_gens = 0;
                let mut remove_chips = 0;
                let mut add_gens = 0;
                let mut add_chips = 0;

                for item in [Some(m.0), Some(m.1)].into_iter().flatten() {
                    match item {
                        Item::Gen(x) => {
                            remove_gens |= x;
                            add_gens |= x;
                        }
                        Item::Chip(x) => {
                            remove_chips |= x;
                            add_chips |= x;
                        }
                    }
                }

                next.floors[current].gens &= !remove_gens;
                next.floors[current].chips &= !remove_chips;

                next.floors[next_floor].gens |= add_gens;
                next.floors[next_floor].chips |= add_chips;

                if !next.floors[current].valid() || !next.floors[next_floor].valid() {
                    continue;
                }

                if seen.insert(next) {
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }

    None
}

#[aoc(day11, part1)]
pub fn part1(_input: &str) -> usize {
    // 0 - thulium
    // 1 - plutonium
    // 2 - strontium
    // 3 - promethium
    // 4 - ruthenium

    let start = State {
        elevator: 0,
        floors: [
            Floor {
                gens: (1 << 0) | (1 << 1) | (1 << 2),
                chips: (1 << 0),
            },
            Floor {
                gens: 0,
                chips: (1 << 1) | (1 << 2),
            },
            Floor {
                gens: (1 << 3) | (1 << 4),
                chips: (1 << 3) | (1 << 4),
            },
            Floor { gens: 0, chips: 0 },
        ],
    };

    shortest_steps(start).unwrap()
}

#[aoc(day11, part2)]
pub fn part2(_input: &str) -> usize {
    // 5 - elerium
    // 6 - dilithium
    let start = State {
        elevator: 0,
        floors: [
            Floor {
                gens: (1 << 0) | (1 << 1) | (1 << 2) | (1 << 5) | (1 << 6),
                chips: (1 << 0) | (1 << 5) | (1 << 6),
            },
            Floor {
                gens: 0,
                chips: (1 << 1) | (1 << 2),
            },
            Floor {
                gens: (1 << 3) | (1 << 4),
                chips: (1 << 3) | (1 << 4),
            },
            Floor { gens: 0, chips: 0 },
        ],
    };

    shortest_steps(start).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}

    #[test]
    fn part2() {}
}
