use std::collections::HashMap;

use regex::regex;

#[derive(Clone, Debug, Default)]
struct Bot {
    low: Output,
    high: Output,
    chips: Vec<usize>,
}

#[derive(Copy, Clone, Debug)]
enum Output {
    Bot(usize),
    Bin(usize),
}

impl Default for Output {
    fn default() -> Self {
        Output::Bot(0)
    }
}

pub fn simulate_until(input: &str, a: usize, b: usize) -> usize {
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("bot") {
            let re = regex!(r"(output|bot) (\d+)");
            let nums: Vec<Output> = re
                .captures_iter(line)
                .map(|c| match &c[1] {
                    "output" => Output::Bin(c[2].parse().unwrap()),
                    "bot" => Output::Bot(c[2].parse().unwrap()),
                    _ => {
                        unreachable!();
                    }
                })
                .collect();

            let re = regex!(r"\d+");
            let i: usize = re.find(line).unwrap().as_str().parse().unwrap();
            bots.entry(i).or_default().low = nums[1];
            bots.entry(i).or_default().high = nums[2];
        } else if line.starts_with("value") {
            let re = regex!(r"\d+");
            let nums: Vec<usize> = re
                .find_iter(line)
                .map(|c| c.as_str().parse().unwrap())
                .collect();
            let c: usize = nums[0];
            let i: usize = nums[1];
            bots.entry(i).or_default().chips.push(c);
        } else {
            unreachable!();
        }
    }

    // simulate
    let mut done;
    let mut bins: HashMap<usize, usize> = HashMap::new();
    assert_eq!(bots.iter().filter(|(_, k)| k.chips.len() == 2).count(), 1);
    loop {
        done = true;
        for i in 0..bots.len() {
            if bots[&i].chips.len() == 2 {
                done = false;

                if bots[&i].chips.contains(&a) && bots[&i].chips.contains(&b) {
                    return i;
                }

                let low_value = *bots[&i].chips.iter().min().unwrap();
                match bots[&i].low {
                    Output::Bot(i) => bots.get_mut(&i).unwrap().chips.push(low_value),
                    Output::Bin(i) => *bins.entry(i).or_default() = low_value,
                }
                let high_value = *bots[&i].chips.iter().max().unwrap();
                match bots[&i].high {
                    Output::Bot(i) => bots.get_mut(&i).unwrap().chips.push(high_value),
                    Output::Bin(i) => *bins.entry(i).or_default() = high_value,
                }
                bots.get_mut(&i).unwrap().chips.truncate(0);
            }
        }

        if done {
            assert_eq!(bots.iter().filter(|(_, k)| k.chips.len() == 2).count(), 0);
            break;
        }
    }
    unreachable!();
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    simulate_until(input, 61, 17)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    for line in input.lines() {
        if line.starts_with("bot") {
            let re = regex!(r"(output|bot) (\d+)");
            let nums: Vec<Output> = re
                .captures_iter(line)
                .map(|c| match &c[1] {
                    "output" => Output::Bin(c[2].parse().unwrap()),
                    "bot" => Output::Bot(c[2].parse().unwrap()),
                    _ => {
                        unreachable!();
                    }
                })
                .collect();

            let re = regex!(r"\d+");
            let i: usize = re.find(line).unwrap().as_str().parse().unwrap();
            bots.entry(i).or_default().low = nums[1];
            bots.entry(i).or_default().high = nums[2];
        } else if line.starts_with("value") {
            let re = regex!(r"\d+");
            let nums: Vec<usize> = re
                .find_iter(line)
                .map(|c| c.as_str().parse().unwrap())
                .collect();
            let c: usize = nums[0];
            let i: usize = nums[1];
            bots.entry(i).or_default().chips.push(c);
        } else {
            unreachable!();
        }
    }

    // simulate
    let mut done;
    let mut bins: HashMap<usize, usize> = HashMap::new();
    assert_eq!(bots.iter().filter(|(_, k)| k.chips.len() == 2).count(), 1);
    loop {
        done = true;
        for i in 0..bots.len() {
            if bots[&i].chips.len() == 2 {
                done = false;

                let low_value = *bots[&i].chips.iter().min().unwrap();
                match bots[&i].low {
                    Output::Bot(i) => bots.get_mut(&i).unwrap().chips.push(low_value),
                    Output::Bin(i) => *bins.entry(i).or_default() = low_value,
                }
                let high_value = *bots[&i].chips.iter().max().unwrap();
                match bots[&i].high {
                    Output::Bot(i) => bots.get_mut(&i).unwrap().chips.push(high_value),
                    Output::Bin(i) => *bins.entry(i).or_default() = high_value,
                }
                bots.get_mut(&i).unwrap().chips.truncate(0);
            }
        }

        if done {
            assert_eq!(bots.iter().filter(|(_, k)| k.chips.len() == 2).count(), 0);
            break;
        }
    }

    bins[&0] * bins[&1] * bins[&2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            simulate_until(
                "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2",
                2,
                5,
            ),
            2
        );
    }

    #[test]
    fn part2() {}
}
