#[derive(Debug)]
struct Disk {
    modulo: u32,
    start: u32,
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u64 {
    let disks: Vec<Disk> = input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once("has ")
                .unwrap()
                .1
                .split_once(" positions; at time=0, it is at position ")
                .unwrap();
            let modulo = l.parse().unwrap();
            let start = (r[..r.len() - 1]).parse().unwrap();
            Disk { modulo, start }
        })
        .collect();
    for time in 0.. {
        if disks.iter().enumerate().into_iter().all(|(i, disk)| {
            let ii = i as u32 + 1;
            let t = (time + ii + disk.start).rem_euclid(disk.modulo);
            t == 0
        }) {
            return time as _;
        }
    }
    unreachable!()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut disks: Vec<Disk> = input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once("has ")
                .unwrap()
                .1
                .split_once(" positions; at time=0, it is at position ")
                .unwrap();
            let modulo = l.parse().unwrap();
            let start = (r[..r.len() - 1]).parse().unwrap();
            Disk { modulo, start }
        })
        .collect();
    disks.push(Disk {
        start: 0,
        modulo: 11,
    });
    for time in 0.. {
        if disks.iter().enumerate().into_iter().all(|(i, disk)| {
            let ii = i as u32 + 1;
            let t = (time + ii + disk.start).rem_euclid(disk.modulo);
            t == 0
        }) {
            return time as _;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            super::part1(
                "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."
            ),
            5
        );
    }

    #[test]
    fn part2() {}
}
