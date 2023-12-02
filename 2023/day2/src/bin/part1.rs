use day2::*;

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.is_valid())
        .map(|game| game.number)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve(input), 8);
    }
}
