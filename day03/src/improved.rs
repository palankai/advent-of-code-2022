use itertools::Itertools;

use crate::char_to_u8;

pub fn part1_solution(content: String) -> String {
    content
        .lines()
        .map(|line| {
            let first = &line[..line.len() / 2];
            let second = &line[line.len() / 2..];
            first
                .chars()
                .find(|c| second.contains(*c))
                .map(char_to_u8)
                .unwrap() as u64
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2_solution(content: String) -> String {
    content
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunks| {
            let (a, b, c) = chunks.collect_tuple::<(&str, &str, &str)>().unwrap();
            a.chars()
                .find(|x| b.contains(*x) && c.contains(*x))
                .map(char_to_u8)
                .unwrap() as u64
        })
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::tests::example;

    use super::*;

    #[test]
    #[ignore]
    fn part1_works() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        assert_eq!(part1_solution(example()), crate::part1_solution(example()));
        assert_eq!(part1_solution(input.clone()), crate::part1_solution(input));
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        assert_eq!(part2_solution(example()), crate::part2_solution(example()));
        assert_eq!(part2_solution(input.clone()), crate::part2_solution(input));
    }
}
