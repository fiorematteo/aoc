use std::collections::HashSet;

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<usize> {
    let mut cards: Vec<usize> = Vec::new();
    for line in input.lines() {
        let (winning, actual) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<usize> = winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let actual: HashSet<usize> = actual
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let intersection = winning.intersection(&actual).count();
        cards.push(intersection);
    }
    cards
}

#[aoc(day4, part1)]
fn part1(cards: &[usize]) -> usize {
    let mut total = 0;
    for &card in cards {
        if card > 0 {
            total += 2_usize.pow(card as u32 - 1);
        }
    }
    total
}

#[aoc(day4, part2, fold)]
fn part2_fold(cards: &[usize]) -> usize {
    let start_cards: Vec<_> = (0..cards.len()).map(|_| 1).collect();
    cards
        .iter()
        .enumerate()
        .fold(start_cards, |mut acc, (index, &value)| {
            for new_card_index in (index + 1)..=(index + value) {
                acc[new_card_index] += acc[index];
            }
            acc
        })
        .iter()
        .sum()
}

#[aoc(day4, part2, recursive)]
fn part2_recursive(cards: &[usize]) -> usize {
    let mut total = cards.len();
    let mut running_cards: Vec<_> = cards.iter().enumerate().rev().collect();
    while let Some((index, card)) = running_cards.pop() {
        for (new_card_index, new_card) in cards.iter().enumerate().skip(index + 1).take(*card) {
            running_cards.push((new_card_index, &new_card));
            total += 1;
        }
    }
    total
}

#[test]
fn test_part1() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part1(&generator(input)), 13);
}

#[test]
fn test_part2() {
    let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    assert_eq!(part2_recursive(&generator(input)), 30);
    assert_eq!(part2_fold(&generator(input)), 30);
}
