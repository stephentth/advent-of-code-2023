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
        .lines()
        .map(|row| row.chars().flat_map(|x| x.to_digit(10)).collect::<Vec<_>>())
        .filter_map(|row| {
            let first = row.first()?;
            let last = row.last()?;
            Some(first * 10 + last)
        })
        .sum();
}

fn part2_approach1(input: &str) -> u32 {
    #[rustfmt::skip]
    let tokens: HashMap<&str, u32> = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
        ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ].iter().cloned().collect();

    let mut total = 0;
    for row in input.split("\n").into_iter() {
        let mut first = 0;
        'outer: for i in 0..row.len() {
            for (tok, val) in &tokens {
                if row[i..].starts_with(tok) {
                    first = *val;
                    break 'outer;
                }
            }
        }
        let mut last = 0;
        'outer: for i in (0..row.len()).rev() {
            for (tok, val) in &tokens {
                if row[i..].starts_with(tok) {
                    last = *val;
                    break 'outer;
                }
            }
        }
        total += first * 10 + last;
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

    assert!(r1_a1 == 55712);
    assert!(r1_a2 == 55712);
    assert!(r2 == 55413);
}
