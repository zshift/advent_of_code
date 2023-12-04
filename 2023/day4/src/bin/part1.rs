use day4::Game;

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u32 {
    let game: Game = input.parse().unwrap();
    game.points()
}
