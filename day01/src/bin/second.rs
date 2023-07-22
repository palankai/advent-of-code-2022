use day01::second_solution;


fn main() {
    let content = std::fs::read_to_string("day01/input.txt").unwrap();
    let res = second_solution(content);
    println!("Result: {:?}", res);
}
