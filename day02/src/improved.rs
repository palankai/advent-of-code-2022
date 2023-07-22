#[derive(Debug, PartialEq, Copy, Clone)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn from_abc(s: char) -> Self {
        match s {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            _ => panic!("Invalid Hand"),
        }
    }
    fn from_xyz(s: char) -> Self {
        match s {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("Invalid Hand"),
        }
    }
    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn beat_by(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Hand::Scissors && other == &Hand::Rock {
            return Some(std::cmp::Ordering::Less);
        }
        if self == &Hand::Rock && other == &Hand::Scissors {
            return Some(std::cmp::Ordering::Greater);
        }
        Some((*self as u8).cmp(&(*other as u8)))
    }
}

#[derive(Copy, Clone)]
enum Cases {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

impl From<(Hand, Hand)> for Cases {
    fn from(value: (Hand, Hand)) -> Self {
        if value.0 == value.1 {
            Cases::Draw
        } else if value.0 < value.1 {
            Cases::Win
        } else {
            Cases::Lost
        }
    }
}
impl From<char> for Cases {
    fn from(value: char) -> Self {
        match value {
            'X' => Cases::Lost,
            'Y' => Cases::Draw,
            'Z' => Cases::Win,
            _ => panic!("Invalid char"),
        }
    }
}

pub fn first_solution(content: String) -> i32 {
    prepare(&content)
        .map(|line| {
            let l = Hand::from_abc(line.0);
            let r = Hand::from_xyz(line.1);
            (l, r)
        })
        .fold(0, |acc, (a, b)| {
            acc + Cases::from((a, b)) as i32 + (b as i32)
        })
}

pub fn second_solution(content: String) -> i32 {
    prepare(&content)
        .map(|line| {
            let l = Hand::from_abc(line.0);
            let c = Cases::from(line.1);
            c as i32
                + match c {
                    Cases::Win => l.beat_by() as i32,
                    Cases::Draw => l as i32,
                    Cases::Lost => l.beats() as i32,
                }
        })
        .sum()
}

pub fn prepare(content: &str) -> impl Iterator<Item = (char, char)> + '_ {
    content.lines().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        (chars[0], chars[chars.len() - 1])
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::example;

    use super::*;

    #[test]
    fn first_works() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        assert_eq!(first_solution(example()), crate::first_solution(example()));
        assert_eq!(first_solution(input.clone()), crate::first_solution(input));
    }

    #[test]
    fn second_works() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        assert_eq!(
            second_solution(example()),
            crate::second_solution(example())
        );
        assert_eq!(
            second_solution(input.clone()),
            crate::second_solution(input)
        );
    }
}
