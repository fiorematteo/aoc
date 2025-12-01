#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_at(1);
            let dir = if dir == "L" { -1 } else { 1 };
            let n = n.parse::<i32>().unwrap();
            (dir, n)
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(i32, i32)]) -> u32 {
    let mut total = 0;
    let mut current = 50;
    for n in input.iter().map(|(a, b)| a * b) {
        current = (current + n) % 100;
        if current == 0 {
            total += 1;
        }
    }
    total
}

#[aoc(day1, part2)]
pub fn part2(input: &[(i32, i32)]) -> u32 {
    let mut total = 0;
    let mut current = 50;
    for &(dir, n) in input {
        for _ in 0..n.abs() {
            current = (current + dir) % 100;
            if current == 0 {
                total += 1;
            }
        }
    }
    total
}

#[test]
fn part1_test() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part1(&gen(input)), 3);
}

#[test]
fn part2_test() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    assert_eq!(part2(&gen(input)), 6);
}
