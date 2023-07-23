use itertools::Itertools;

pub fn part1_solution(content: String) -> String {
    let (towers, instructions) = split(&content);
    let mut towers = make_towers(towers);
    let instructions = make_instructions(instructions);
    instructions.for_each(|(count, from, to)| {
        move_items(
            &mut towers,
            count as usize,
            from as usize - 1,
            to as usize - 1,
        );
    });
    let mut res = Vec::<char>::new();
    for tower in towers {
        let last = tower.last().unwrap();
        res.push(last.clone());
    }
    res.iter().collect::<String>()
}

pub fn part2_solution(content: String) -> String {
    let (towers, instructions) = split(&content);
    let mut towers = make_towers(towers);
    let instructions = make_instructions(instructions);
    instructions.for_each(|(count, from, to)| {
        move_items_keep_order(
            &mut towers,
            count as usize,
            from as usize - 1,
            to as usize - 1,
        );
    });
    let mut res = Vec::<char>::new();
    for tower in towers {
        let last = tower.last().unwrap();
        res.push(last.clone());
    }
    res.iter().collect::<String>()
}

pub fn split(content: &str) -> (&str, &str) {
    let c = content
        .split("\n\n")
        .collect_tuple::<(&str, &str)>()
        .unwrap();
    c
}

pub fn make_instructions<'a>(instructions: &'a str) -> impl Iterator<Item = (u32, u32, u32)> + 'a {
    instructions.lines().map(|line| {
        line.clone()
            .trim()
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "")
            .split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple::<(u32, u32, u32)>()
            .unwrap()
    })
}

pub fn make_towers<'a>(towers: &'a str) -> Vec<Vec<char>> {
    let mut t = towers.lines().rev().collect::<Vec<_>>();
    let indexes = t.drain(0..1).collect::<Vec<_>>().pop().unwrap();
    let num_towers = num_columns(indexes);
    let mut towers: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_towers {
        towers.push(Vec::new());
    }
    t.iter().for_each(|line| {
        let mut chars = line.chars();
        for i in 0..num_towers {
            let indx = i * 4 + 1;
            if line.len() > indx {
                let char = chars.clone().nth(indx).unwrap();
                if char != ' ' {
                    towers[i].push(char);
                }
            }
        }
    });
    towers
}

fn num_columns(line: &str) -> usize {
    (line.len() as f32 / 4.0).ceil() as usize
}

pub fn move_items(towers: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    let count = count;
    let new_len = towers[from].len() - count;
    let items = towers[from][new_len..].to_vec();
    towers[from].truncate(new_len);
    towers[to].extend(items);
}

pub fn move_items_keep_order(towers: &mut Vec<Vec<char>>, count: usize, from: usize, to: usize) {
    let mut count = count;
    while count > 0 {
        let item = towers[from].pop().unwrap();
        towers[to].push(item);
        count -= 1;
    }
}

#[cfg(test)]
mod tests {
    use aochelper::prepare_example;

    use super::*;

    pub fn example() -> String {
        String::from(
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        )
    }

    #[test]
    fn first_works() {
        assert_eq!(part1_solution(example()), "CMZ");
    }

    #[test]
    fn second_works() {
        assert_eq!(part2_solution(example()), "MCD");
    }

    #[test]
    fn instruction_making_works() {
        let example = example();
        let instructions = split(&example).1;
        let instructions = make_instructions(instructions).collect::<Vec<_>>();
        assert_eq!(instructions[0], (1, 2, 1));
        assert_eq!(instructions[1], (3, 1, 3));
        assert_eq!(instructions[2], (2, 2, 1));
        assert_eq!(instructions[3], (1, 1, 2));
    }

    #[test]
    fn tower_making_works() {
        let example = example();
        let towers = split(&example).0;
        let towers = make_towers(towers);

        assert_eq!(towers.len(), 3);
        assert_eq!(towers[0], vec!['Z', 'N']);
        assert_eq!(towers[1], vec!['M', 'C', 'D']);
        assert_eq!(towers[2], vec!['P']);
    }
}
