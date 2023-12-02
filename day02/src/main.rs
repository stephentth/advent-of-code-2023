// Cube Conundrum
use std::collections::HashMap;
use std::str::FromStr;

// APPROACH 2

type Cubes = [u32; 3]; // red, green, blue

#[derive(Debug)]
pub struct Game {
    id: u32,
    cubes: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = std::convert::Infallible;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (id, game_data) = line.split_once(':').unwrap();
        Ok(Self {
            id: id.split_once(' ').unwrap().1.parse().unwrap(),
            cubes: game_data
                .split("; ")
                .map(|round| {
                    let mut round_cubes = [0, 0, 0];
                    for (n, color) in round.split(',').map(|s| s.trim().split_once(' ').unwrap()) {
                        round_cubes[color.as_bytes()[0] as usize % 3] = n.parse().unwrap();
                    }
                    round_cubes
                })
                .collect(),
        })
    }
}

fn cube_conundrum_part1_approach2(input: &str) -> u32 {
    input
        .lines()
        .map(|row| Game::from_str(row).unwrap())
        .into_iter()
        .filter(|g| {
            g.cubes
                .iter()
                .all(|r| r[0] <= 12 && r[1] <= 13 && r[2] <= 14)
        })
        .map(|g| g.id)
        .sum()
}

// APPROACH 1

use regex::Regex;

fn cube_conundrum_part1_approach1(input: &str) -> u32 {
    let re = Regex::new(r"Game (?P<game_no>\d*): (?P<game_data>.*)").unwrap();
    let mut total = 0;
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    let mut elf_cubes = HashMap::new();
    elf_cubes.insert("red", 12);
    elf_cubes.insert("green", 13);
    elf_cubes.insert("blue", 14);

    'outer: for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let game_no = &caps["game_no"].parse::<u32>().unwrap();
        let game_data = &caps["game_data"].replace(",", "").replace(";", "");

        let round: Vec<&str> = game_data.split([' ']).collect();

        for i in 0..(round.len() / 2) {
            if round[i * 2].parse::<u32>().unwrap() > *elf_cubes.get(round[i * 2 + 1]).unwrap() {
                continue 'outer;
            }
        }
        total += game_no;
    }
    total
}

fn cube_conundrum_part2_approach1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut min_cubes = HashMap::new();

        let game_data_line = line
            .split(":")
            .nth(1)
            .unwrap()
            .trim()
            .replace(",", "")
            .replace(";", "");

        let game_data_vec: Vec<&str> = game_data_line.split_whitespace().collect();

        for i in 0..(game_data_vec.len() / 2) {
            let color_key = game_data_vec[i * 2 + 1].to_owned();
            let cube_count = game_data_vec[i * 2].parse::<u32>().unwrap();

            if &cube_count > min_cubes.get(&color_key).unwrap_or(&0) {
                min_cubes.insert(color_key, cube_count);
            }
        }
        total += min_cubes.get("red").unwrap()
            * min_cubes.get("green").unwrap()
            * min_cubes.get("blue").unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(cube_conundrum_part1_approach1(input), 8);
        assert_eq!(cube_conundrum_part1_approach2(input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(cube_conundrum_part2_approach1(input), 2286);
    }
}

fn main() {
    let input = include_str!("input.txt");
    let p1_a1 = cube_conundrum_part1_approach1(input);
    let p1_a2 = cube_conundrum_part1_approach2(input);
    let p2_a1 = cube_conundrum_part2_approach1(input);
    println!("{}", p1_a1);
    println!("{}", p1_a2);
    println!("{}", p2_a1);

    assert_eq!(p1_a1, 2265);
    assert_eq!(p1_a2, 2265);
    assert_eq!(p2_a1, 64097);
}
