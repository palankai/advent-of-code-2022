use itertools::Itertools;

pub fn first_solution(content: String) -> i32 {
    content
        .lines()
        .fold(vec![0], calc)
        .into_iter()
        .sorted()
        .last()
        .unwrap()
}

pub fn second_solution(content: String) -> i32 {
    content
        .lines()
        .fold(vec![0], calc)
        .into_iter()
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>()
}

fn calc(items: Vec<i32>, line: &str) -> Vec<i32> {
    let mut items = items;
    if line.is_empty() {
        items.push(0);
    } else {
        let num = line.parse::<i32>().unwrap();
        let last = items.pop().unwrap();
        items.push(last + num);
    }
    items
}

#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    fn example() -> String {
        prepare_example(
            "1000
            2000
            3000

            4000

            5000
            6000

            10000

            7000
            8000
            9000",
        )
    }

    #[test]
    fn first_works() {
        assert_eq!(first_solution(example()), 24000);
    }

    #[test]
    fn second_works() {
        assert_eq!(second_solution(example()), 45000);
    }
}
