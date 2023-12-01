use std::collections::HashMap;

fn part1_approach1(input: &str) -> u32 {
    let mut total = 0;

    for row in input.split("\n").into_iter() {
        let mut t: Vec<u32> = vec![];
        for ch in row.chars() {
            if !ch.is_digit(10) {
                continue;
            }
            t.push(ch.to_digit(10).unwrap_or(0));
        }
        total += t.first().unwrap_or(&0) * 10 + t.last().unwrap_or(&0);
    }
    total
}

fn part1_approach2(input: &str) -> u32 {
    return input
        .split("\n")
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|x| x.to_digit(10).unwrap_or(0) as u32)
                .filter(|x| x != &0)
                .collect::<Vec<_>>()
        })
        .map(|row| row.first().unwrap_or(&0) * 10 + row.last().unwrap_or(&0))
        .sum();
}

fn part2_approach1(input: &str) -> u32 {
    let mut tokens: HashMap<&str, u32> = HashMap::new();
    tokens.insert("one", 1);
    tokens.insert("two", 2);
    tokens.insert("three", 3);
    tokens.insert("four", 4);
    tokens.insert("five", 5);
    tokens.insert("six", 6);
    tokens.insert("seven", 7);
    tokens.insert("eight", 8);
    tokens.insert("nine", 9);
    tokens.insert("1", 1);
    tokens.insert("2", 2);
    tokens.insert("3", 3);
    tokens.insert("4", 4);
    tokens.insert("5", 5);
    tokens.insert("6", 6);
    tokens.insert("7", 7);
    tokens.insert("8", 8);
    tokens.insert("9", 9);

    let mut total = 0;

    for row in input.split("\n").into_iter() {
        let mut res: Vec<u32> = vec![];
        for i in 0..row.len() {
            for (tok, val) in &tokens {
                if row[i..].starts_with(tok) {
                    res.push(*val);
                }
            }
        }
        total += res.first().unwrap_or(&0) * 10 + res.last().unwrap_or(&0);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1_approach1(input), 142);
        assert_eq!(part1_approach2(input), 142);
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2_approach1(input), 281);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let r1_a1 = part1_approach1(input);
    let r1_a2 = part1_approach2(input);
    let r2 = part2_approach1(input);

    println!("part 1 = {}", r1_a1);
    println!("part 1 = {}", r1_a2);
    println!("part 2 = {}", r2);
}
