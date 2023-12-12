use day5::Almanac;

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();
    almanac.lowest_location_that_needs_a_seed()
}
