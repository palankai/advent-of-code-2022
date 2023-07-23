
pub fn part1_solution(content: String) -> String {
    "".to_string()
}

pub fn part2_solution(content: String) -> String {
    "".to_string()
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
        assert_eq!(
            part2_solution(example()),
            crate::part2_solution(example())
        );
        assert_eq!(
            part2_solution(input.clone()),
            crate::part2_solution(input)
        );
    }
}
