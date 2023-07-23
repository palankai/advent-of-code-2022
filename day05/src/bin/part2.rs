use day05::part2_solution;

fn main() {
    let content = std::fs::read_to_string("day05/input.txt").unwrap();
    let res = part2_solution(content);
    println!("Result: {:?}", res);
}
