// Get the first and last digit in a line to create a 2-digit number. The first and last can be the same character.
// Sum up all the numbers.
fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u32 {
    let zero = u32::from('0');
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .chars()
                .filter(|x| x.is_ascii_digit())
                .map(|x| u32::from(x) - zero)
                .collect();
            let first = digits.first().unwrap() * 10;
            let last = digits.last().unwrap();
            first + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "asd1asdf23asdf4\n5asdfasdf678asdfasdf\nasdfasdfasdfasdf8asdfasdfasdfasdf";
        assert_eq!(solve(input), 14 + 58 + 88);
    }
}
