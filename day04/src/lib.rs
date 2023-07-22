use std::{ops::RangeInclusive};

use itertools::Itertools;

pub fn part1_solution(content: String) -> String {
    content.lines()
    .map(parse_line)
    .filter(|(a, b)| {
        a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())

    }).count().to_string()
}

pub fn part2_solution(content: String) -> String {
    content.lines()
    .map(parse_line)
    .filter(|(a, b)| {
        a.contains(&b.start()) || a.contains(&b.end()) || b.contains(&a.start()) || b.contains(&a.end())

    }).count().to_string()
}

pub fn parse_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let ((a, b), (c, d)) = line.split(",").map(|r| r.split("-").collect_tuple().unwrap()).collect_tuple().unwrap();
    ((a.parse().unwrap()..=b.parse().unwrap()), (c.parse().unwrap()..=d.parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    pub fn example() -> String {
        prepare_example("2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8")
    }

    #[test]
    fn first_works() {
        assert_eq!(part1_solution(example()), "2");
    }

    #[test]
    fn second_works() {
        assert_eq!(part2_solution(example()), "4");
    }

}
