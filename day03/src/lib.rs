pub mod improved;

pub fn part1_solution(content: String) -> String {
    "".to_string()
}

pub fn part2_solution(content: String) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    pub fn example() -> String {
        prepare_example("")
    }

    #[test]
    fn part1_works() {
        assert_eq!(part1_solution(example()), "1");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        assert_eq!(part2_solution(example()), "2");
    }

}
