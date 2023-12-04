use std::collections::{HashMap, HashSet};

fn scratchcards_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split(':').nth(1).unwrap())
        .filter_map(|row| {
            let mut s = row.split(" | ");
            let winning_nums: HashSet<&str> = s.next().unwrap().split_whitespace().collect();
            let having_nums: HashSet<&str> = s.next().unwrap().split_whitespace().collect();
            let res = winning_nums.intersection(&having_nums).count() as u32;
            Some(2_u32.pow(res.checked_sub(1)?))
        })
        .sum()
}

fn scratchcards_part2(input: &str) -> u32 {
    let d: HashMap<u32, u32> = input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(':');
            let card_id = line_iter
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut line_data = line_iter.next().unwrap().split('|');
            let winning_nums: HashSet<&str> =
                line_data.next().unwrap().split_whitespace().collect();
            let having_nums: HashSet<&str> = line_data.next().unwrap().split_whitespace().collect();

            (
                card_id,
                winning_nums.intersection(&having_nums).count() as u32,
            )
        })
        .collect();
    d.keys().map(|x| scratch(&d, *x)).sum()
}

fn scratch(d: &HashMap<u32, u32>, scratch_id: u32) -> u32 {
    let n = *d.get(&scratch_id).unwrap_or(&0);
    (1..=n).map(|i| scratch(d, scratch_id + i)).sum::<u32>() + 1
}

fn main() {
    let input = include_str!("input.txt");
    let p1_a1 = scratchcards_part1(input);
    println!("p1 = {}", p1_a1);
    let p2_a1 = scratchcards_part2(input);
    println!("p2 = {}", p2_a1);

    assert_eq!(p1_a1, 32001);
    assert_eq!(p2_a1, 5037841);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(scratchcards_part1(input), 13);
        assert_eq!(scratchcards_part2(input), 30);
    }
}
