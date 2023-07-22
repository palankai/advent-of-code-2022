use day01::first_solution;

fn main() {
    let content = std::fs::read_to_string("day01/input.txt").unwrap();
    let res = first_solution(content);
    println!("Result: {:?}", res);
}
