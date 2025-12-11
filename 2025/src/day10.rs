use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};
use z3::{ast::Int, Optimize, SatResult};

#[aoc(day10, part1, dumb)]
pub fn part1_dumb(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let indicators: &str = parts[0];
        let indicators: usize = indicators[1..indicators.len() - 1]
            .chars()
            .map(|c| c == '#')
            .enumerate()
            .map(|(i, b)| if b { 1 << i } else { 0 })
            .sum();

        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|&s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let start_state = 0;
        let mut queue = BinaryHeap::new();
        queue.push(State {
            state: start_state.clone(),
            presses: 0,
        });
        while let Some(State { state, presses }) = queue.pop() {
            if state == indicators {
                total += presses;
                break;
            }
            for button in &buttons {
                let mut new_state = state.clone();
                for &index in button {
                    new_state ^= 1 << index;
                }
                queue.push(State {
                    state: new_state,
                    presses: presses + 1,
                });
            }
        }
    }
    total
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let indicators: &str = parts[0];
        let indicators: usize = indicators[1..indicators.len() - 1]
            .chars()
            .map(|c| c == '#')
            .enumerate()
            .map(|(i, b)| if b { 1 << i } else { 0 })
            .sum();

        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|&s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        let start_state = 0;
        let mut states_seen = HashMap::new();
        states_seen.insert(start_state.clone(), 0);
        let mut queue = BinaryHeap::new();
        queue.push(State {
            state: start_state.clone(),
            presses: 0,
        });
        while let Some(State { state, presses }) = queue.pop() {
            if state == indicators {
                total += presses;
                break;
            }
            for button in &buttons {
                let mut new_state = state.clone();
                for &index in button {
                    new_state ^= 1 << index;
                }
                if let Some(&seen_presses) = states_seen.get(&new_state) {
                    if seen_presses < presses + 1 {
                        continue;
                    }
                }
                states_seen.insert(new_state.clone(), presses + 1);
                queue.push(State {
                    state: new_state,
                    presses: presses + 1,
                });
            }
        }
    }
    total
}

#[derive(Eq, PartialEq)]
struct State<S: Clone + Eq + PartialEq + Hash> {
    state: S,
    presses: usize,
}

impl<S: Clone + Eq + PartialEq + Hash> PartialOrd for State<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.presses.partial_cmp(&self.presses)
    }
}

impl<S: Clone + Eq + PartialEq + Hash> Ord for State<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.presses.cmp(&self.presses)
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
            .iter()
            .map(|&s| {
                s[1..s.len() - 1]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let buttons_presses: Vec<_> = buttons.iter().map(|_| Int::fresh_const("bp")).collect();
        let joltages: &str = parts[parts.len() - 1];
        let joltages: Vec<usize> = joltages[1..joltages.len() - 1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let solver = Optimize::new();
        for bp in &buttons_presses {
            solver.assert(&bp.ge(&Int::from_u64(0)));
        }
        for (i, &joltage) in joltages.iter().enumerate() {
            let sum = buttons_presses
                .iter()
                .enumerate()
                .filter(|(j, _)| buttons[*j].contains(&i))
                .map(|(_, x)| x.clone())
                .fold(Int::from_u64(0), |a, b| a + b);
            solver.assert(&sum.eq(&Int::from_u64(joltage as u64)));
        }
        let min_presses = buttons_presses.iter().fold(Int::from_u64(0), |a, b| a + b);

        solver.minimize(&min_presses);
        assert_eq!(solver.check(&[]), SatResult::Sat);
        let model = solver.get_model().unwrap();
        let min = model.eval(&min_presses, true).unwrap().as_u64().unwrap();
        total += min as usize;
    }
    total
}

#[test]
fn part1_test() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(part1(input), 7);
}

#[test]
fn part2_test() {
    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(part2(input), 33);
}
