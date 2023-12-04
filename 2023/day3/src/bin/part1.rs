use day3::Engine;

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u32 {
    let engine: Engine = input.parse().unwrap();
    engine.sum_of_parts()
}
 