use day04::part1_solution;

fn main() {
    let content = std::fs::read_to_string("day04/input.txt").unwrap();
    let res = part1_solution(content);
    println!("Result: {:?}", res);
}
