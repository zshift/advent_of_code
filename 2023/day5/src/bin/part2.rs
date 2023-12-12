use day5::{Almanac, Almanac2};

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();
    let almanac: Almanac2 = almanac.into();
    almanac.lowest_location_that_needs_a_seed()
}
