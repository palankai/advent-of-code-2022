pub mod improved;

use itertools::Itertools;

pub fn prepare(content: String) -> Vec<(char, char)> {
    content.lines().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        (chars[0], chars[chars.len() - 1])
    })
    .collect_vec()
}

pub fn first_solution(content: String) -> i32 {
    prepare(content).into_iter()
    .fold(0, |acc, (a, b)| {
        let my_score = match b {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("Invalid char")
        };
        let lost = 0;
        let draw = 3;
        let win= 6;

        let score = match a {
            'A' => match b {
                'X' => draw + my_score,
                'Y' => win + my_score,
                'Z' => lost + my_score,
                _ => panic!("Invalid char")
            },
            'B' => match b {
                'X' => lost + my_score,
                'Y' => draw + my_score,
                'Z' => win + my_score,
                _ => panic!("Invalid char")
            },
            'C' => match b {
                'X' => win + my_score,
                'Y' => lost + my_score,
                'Z' => draw + my_score,
                _ => panic!("Invalid char")
            },
            _ => panic!("Invalid char")
        };
        score + acc
    })

}

pub fn second_solution(content: String) -> i32 {
    prepare(content).into_iter()
    .fold(0, |acc, (a, b)| {
        let outcome = match b {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Invalid char")
        };
        let r = 1;
        let p = 2;
        let s = 3;

        let score = match a {
            'A' => match b {
                'X' => s + outcome,
                'Y' => r + outcome,
                'Z' => p + outcome,
                _ => panic!("Invalid char")
            },
            'B' => match b {
                'X' => r + outcome,
                'Y' => p + outcome,
                'Z' => s + outcome,
                _ => panic!("Invalid char")
            },
            'C' => match b {
                'X' => p + outcome,
                'Y' => s + outcome,
                'Z' => r + outcome,
                _ => panic!("Invalid char")
            },
            _ => panic!("Invalid char")
        };
        score + acc
    })
}


#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    pub fn example() -> String {
        prepare_example("A Y
        B X
        C Z")
    }

    #[test]
    fn first_works() {
        assert_eq!(first_solution(example()), 15);
    }

    #[test]
    fn second_works() {
        assert_eq!(second_solution(example()), 12);
    }

}
