use std::collections::HashMap;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (key, values) = line.split_once(": ").unwrap();
        map.insert(key, values.split_whitespace().collect::<Vec<_>>());
    }
    get_paths("you", "out", &map)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let var_name = HashMap::new();
    let var_name = var_name;
    let mut map = var_name;
    for line in input.lines() {
        let (key, values) = line.split_once(": ").unwrap();
        map.insert(key, values.split_whitespace().collect::<Vec<_>>());
    }

    let a = get_paths("svr", "fft", &map)
        * get_paths("fft", "dac", &map)
        * get_paths("dac", "out", &map);
    let b = get_paths("svr", "dac", &map)
        * get_paths("dac", "fft", &map)
        * get_paths("fft", "out", &map);
    a + b
}
fn get_paths(from: &str, to: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    fn explore<'a>(
        from: &'a str,
        to: &str,
        cache: &mut HashMap<&'a str, usize>,
        map: &'a HashMap<&str, Vec<&str>>,
    ) -> usize {
        if let Some(&count) = cache.get(from) {
            return count;
        }
        let mut total = 0;
        let Some(next_nodes) = map.get(from) else {
            return 0;
        };
        for &next in next_nodes {
            if next == to {
                total += 1;
            } else {
                total += explore(next, to, cache, map);
            }
        }
        cache.insert(from, total);
        total
    }
    explore(from, to, &mut HashMap::new(), map)
}

#[test]
fn part1_test() {
    let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    assert_eq!(part1(input), 5);
}

#[test]
fn part2_test() {
    let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    assert_eq!(part2(input), 2);
}
