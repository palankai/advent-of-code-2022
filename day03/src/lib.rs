use itertools::Itertools;
use std::collections::HashSet;

pub mod improved;

pub fn part1_solution(content: String) -> String {
    content
        .lines()
        .map(|line| {
            let first = &line[..line.len() / 2]
                .chars()
                .map(char_to_u8)
                .collect::<HashSet<u8>>();
            let second = &line[line.len() / 2..]
                .chars()
                .map(char_to_u8)
                .collect::<HashSet<u8>>();
            first.iter().fold(0, |acc, x| {
                if second.contains(x) {
                    return acc + *x as u64;
                }
                acc
            })
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2_solution(content: String) -> String {
    let res = content
        .lines()
        .chunks(3)
        .into_iter()
        .map(|lines| lines.map(|line| line.chars().map(char_to_u8).collect::<HashSet<u8>>()))
        .map(|lines| {
            let lines = lines.collect::<Vec<HashSet<u8>>>();
            let intersection = lines[0]
                .clone()
                .intersection(&lines[1].clone())
                .copied()
                .collect::<HashSet<u8>>();
            let intersection = intersection
                .intersection(&lines[2].clone())
                .copied()
                .collect::<HashSet<u8>>();
            intersection.into_iter().fold(0, |acc, x| acc + x as u64)
        })
        .sum::<u64>();
    res.to_string()
}

pub fn char_to_u8(c: char) -> u8 {
    let value = c as u8;
    if value >= 97 {
        value - 96
    } else {
        value - 64 + 26
    }
}

#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    pub fn example() -> String {
        prepare_example(
            "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw",
        )
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1_solution(example()), "157");
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2_solution(example()), "70");
    }

    #[test]
    fn char_to_u8_works() {
        assert_eq!(char_to_u8('a'), 1);
        assert_eq!(char_to_u8('z'), 26);
        assert_eq!(char_to_u8('A'), 27);
        assert_eq!(char_to_u8('Z'), 52);
    }
}
