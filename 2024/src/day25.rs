#[aoc(day25, part1)]
pub fn part1(input: &str) -> i32 {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schematic in input.split("\n\n") {
        let mut lines = schematic.lines();
        if lines.next().unwrap() == "....." {
            let mut key_heights = [-1, -1, -1, -1, -1];
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' && key_heights[x] == -1 {
                        key_heights[x] = 5 - y as i32;
                    }
                }
            }
            keys.push(key_heights);
        } else {
            let mut lock_heights = [0, 0, 0, 0, 0];
            for (y, line) in lines.enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock_heights[x] = 1 + y as i32;
                    }
                }
            }
            locks.push(lock_heights);
        }
    }
    let mut total = 0;
    for key in keys {
        for lock in &locks {
            if (0..5).all(|i| key[i] + lock[i] < 6){
                total += 1;
            }
        }
    }
    total
}

#[test]
fn test_part1() {
    let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    assert_eq!(part1(input), 3);
}
