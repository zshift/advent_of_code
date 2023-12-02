use std::collections::HashMap;

fn main() {
    println!("{}", solve(include_str!("../../input.txt")));
}

fn solve(input: &str) -> u32 {
    let zero = u32::from('0');
    input
        .lines()
        .map(|line| {
            let digits: Vec<(usize, u32)> = line
                .chars()
                .enumerate()
                .filter(|(_, x)| x.is_ascii_digit())
                .map(|(i, x)| (i, u32::from(x) - zero))
                .collect();

            let parsed_digits = parse_number_as_word(line);
            let mut all_digits = [digits, parsed_digits].concat();
            all_digits.sort_by(|(i, _), (j, _)| i.cmp(j));
            let digits: Vec<u32> = all_digits.iter().map(|(_, x)| *x).collect();

            let first = digits.first().unwrap() * 10;
            let last = digits.last().unwrap();
            first + last
        })
        .sum()
}

fn parse_number_as_word(input: &str) -> Vec<(usize, u32)> {
    let mut words: HashMap<&str, u32> = HashMap::new();
    words.insert("one", 1);
    words.insert("two", 2);
    words.insert("three", 3);
    words.insert("four", 4);
    words.insert("five", 5);
    words.insert("six", 6);
    words.insert("seven", 7);
    words.insert("eight", 8);
    words.insert("nine", 9);

    words
        .iter()
        .flat_map(|(&k, &v)| {
            if input.contains(k) {
                input
                    .match_indices(k)
                    .map(|(i, _)| (i, v))
                    .collect::<Vec<(usize, u32)>>()
            } else {
                Vec::new()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_words() {
        let input = "one2three4five";
        let mut output = parse_number_as_word(input);
        output.sort_by(|(i, _), (j, _)| i.cmp(j));
        assert_eq!(output, vec![(0, 1), (4, 3), (10, 5)]);
    }

    #[test]
    fn solution() {
        let inputs = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expected_outputs = [29, 83, 13, 24, 42, 14, 76];

        inputs
            .iter()
            .zip(expected_outputs.iter())
            .for_each(|(input, expected_output)| {
                let output = solve(input);
                assert_eq!(output, *expected_output);
            });
    }
}
