use anyhow::{Error, Result};
use std::{collections::HashSet, fmt::Display, ops::RangeInclusive, str::FromStr};

// Iterate over the input and find all the parts.
// A part is a number with a symbol on either side or diagnol of the number.
// A . is ignored.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part {
    pub number: u32,
    pub row: usize,
    pub start: usize,
    pub end: usize,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

fn truncated_range(start: usize, end: usize, length: usize) -> RangeInclusive<usize> {
    start.checked_sub(1).unwrap_or(0)..=(length - 1).min(end + 1)
}

impl Part {
    /// A part is valid if it has a symbol on either side or diagnol of the number.
    ///
    /// Example:
    ///
    /// *****
    /// *123*
    /// *****
    ///
    /// A symbol in any position where there is a * makes 123 a valid part.
    pub fn is_valid(&self, s: &str) -> bool {
        let special: HashSet<char> =
            ['\n', '\r', '#', '$', '%', '&', '*', '+', '-', '/', '=', '@'].into();
        let is_valid = |c| special.contains(&c);

        let lines = || s.lines().map(str::trim);

        // Get 3 lines: above, current, and below.
        // When row is 0, there is no above.
        // When row is the last row, there is no below.
        let valid_above = || {
            if self.row != 0 {
                let above = lines().nth(self.row - 1).unwrap();
                above[truncated_range(self.start, self.end, above.len())]
                    .trim()
                    .chars()
                    .any(is_valid)
            } else {
                false
            }
        };

        let valid_below = || {
            if self.row != lines().count() - 1 {
                let below = lines().nth(self.row + 1).unwrap();
                below[truncated_range(self.start, self.end, below.len())]
                    .trim()
                    .chars()
                    .any(is_valid)
            } else {
                false
            }
        };

        let valid_left = || {
            if self.start == 0 {
                return false;
            }

            lines()
                .nth(self.row)
                .unwrap()
                .chars()
                .skip(self.start - 1)
                .take(1)
                .any(is_valid)
        };

        let valid_right = || {
            if self.end == lines().count() {
                return false;
            }

            lines()
                .nth(self.row)
                .unwrap()
                .chars()
                .skip(self.end + 1)
                .take(1)
                .any(is_valid)
        };

        valid_left() || valid_right() || valid_above() || valid_below()
    }
}

trait Overlap {
    fn overlaps(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> Overlap for RangeInclusive<T> {
    fn overlaps(&self, other: &Self) -> bool {
        self.start().le(other.end()) && self.end().ge(other.start())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Gear {
    pub row: usize,
    pub col: usize,
}

impl Display for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*")
    }
}

impl Gear {
    /// A gear is valid if it's connected to exactly 2 parts.
    ///
    /// Example:
    ///
    /// 123.456
    /// ...*...
    /// .......
    ///
    /// The gear at 1, 3 is connected to 2 parts, so is valid.
    ///
    /// 123.456
    /// ...*...
    /// ...3...
    ///
    /// The gear at 1, 3 is connected to 3 parts, so is not valid.
    pub fn ratio(&self, parts: &[Part]) -> Option<u32> {
        let mut connected_parts = vec![];

        for part in parts {
            let left_to_right = self.col.checked_sub(1).unwrap_or(0)..=(self.col + 1);
            let top_to_bottom = self.row.checked_sub(1).unwrap_or(0)..=(self.row + 1);

            if left_to_right.overlaps(&(part.start..=part.end)) && top_to_bottom.contains(&part.row)
            {
                connected_parts.push(part);
            }
        }

        if connected_parts.len() == 2 {
            Some(connected_parts[0].number * connected_parts[1].number)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Engine {
    pub parts: Vec<Part>,
    pub gears: Vec<Gear>,
}

impl FromStr for Engine {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = vec![];
        let mut gears = vec![];

        let mut push_part_if_valid = |part_start, number, row, end| {
            if let Some(start) = part_start {
                let part = Part {
                    number,
                    row,
                    start,
                    end,
                };

                if part.is_valid(input) {
                    parts.push(part);
                }
            }
        };

        for (row, line) in input.lines().map(|l| l.trim()).enumerate() {
            // always reset start and num at the start of each line.
            let mut number = 0;
            let mut part_start: Option<usize> = None;

            for (col, c) in line.chars().enumerate() {
                if let Some(digit) = c.to_digit(10) {
                    part_start = part_start.or(Some(col));
                    number *= 10;
                    number += digit;
                } else {
                    push_part_if_valid(part_start, number, row, col.checked_sub(1).unwrap_or(0));

                    number = 0;
                    part_start = None;
                }
                if c == '*' {
                    gears.push(Gear { row, col });
                }
            }

            push_part_if_valid(part_start, number, row, line.len() - 1);
        }

        Ok(Engine { parts, gears })
    }
}

impl Engine {
    pub fn sum_of_parts(&self) -> u32 {
        self.parts.iter().map(|p| p.number).sum()
    }

    pub fn sum_of_gears(&self) -> u32 {
        self.gears
            .iter()
            .map(|g| g.ratio(&self.parts))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn engine_from_str() {
        let input = "467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..";
        let engine = Engine::from_str(input).unwrap();
        assert_eq!(engine.sum_of_parts(), 4361);
        assert_eq!(engine.sum_of_gears(), 467835);
    }

    #[test]
    fn part_is_valid_right() {
        let input = "7*";
        let part = Part {
            number: 7,
            row: 0,
            start: 0,
            end: 0,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_left() {
        let input = "*4";
        let part = Part {
            number: 4,
            row: 0,
            start: 1,
            end: 1,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_top_left() {
        let input = "*.
                     .4";
        let part = Part {
            number: 4,
            row: 0,
            start: 1,
            end: 1,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_top_right() {
        let input = ".*
                     4.";
        let part = Part {
            number: 4,
            row: 1,
            start: 0,
            end: 0,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_bottom_right() {
        let input = "4.
                     .*";
        let part = Part {
            number: 4,
            row: 0,
            start: 0,
            end: 0,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_bottom_left() {
        let input = ".4
                     *.";
        let part = Part {
            number: 4,
            row: 0,
            start: 0,
            end: 1,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_above() {
        let input = "*
                     4";
        let part = Part {
            number: 4,
            row: 1,
            start: 0,
            end: 0,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_valid_below() {
        let input = "4
                     *";
        let part = Part {
            number: 4,
            row: 0,
            start: 0,
            end: 0,
        };
        assert!(part.is_valid(input));
    }

    #[test]
    fn part_is_not_valid_surround() {
        let input = "...
                     .4.
                     ...";
        let part = Part {
            number: 4,
            row: 1,
            start: 1,
            end: 1,
        };
        assert!(!part.is_valid(input));
    }

    #[test]
    fn part_is_not_valid() {
        let input = "4";
        let part = Part {
            number: 4,
            row: 0,
            start: 0,
            end: 0,
        };
        assert!(!part.is_valid(input));
    }

    #[test]
    fn part_is_not_valid_carriage_return() {
        let input = "\r\r\r
                     \r4\r
                     \r\r\r";
        let part = Part {
            number: 4,
            row: 0,
            start: 0,
            end: 0,
        };
        assert!(!part.is_valid(input));
    }

    #[test]
    fn test_part_input1() {
        let input = "......@...583.....*........*................358...........750........................*......532..................*...22...../....512...#....
                     863...112................178...+...........*...........5./............98......584..222..........862...235.....448...*.....737.....*.....516.
                     ....#.......425..............923.84*......947......999*..............*....280*...........732...&.....*.............14.............683.......";
        let engine: Engine = input.parse().unwrap();
        assert_eq!(
            engine.sum_of_parts(),
            358 + 750
                + 22
                + 512
                + 112
                + 178
                + 5
                + 98
                + 584
                + 222
                + 862
                + 235
                + 448
                + 737
                + 516
                + 923
                + 84
                + 947
                + 999
                + 280
                + 14
                + 683
        );
    }

    #[test]
    #[ignore]
    fn unique_symbols() {
        let input = include_str!("../input.txt");
        let set = HashSet::<char>::from_iter(input.chars());
        let mut unique_chars = set.iter().collect::<Vec<_>>();
        unique_chars.sort();
        println!("{:?}", unique_chars);
    }

    #[test]
    fn test_truncated_range1() {
        let r = truncated_range(0, 2, 5);
        assert_eq!(r, 0..=3);
    }

    #[test]
    fn test_truncated_range2() {
        let r = truncated_range(1, 2, 5);
        assert_eq!(r, 0..=3);
    }

    #[test]
    fn test_truncated_range3() {
        let r = truncated_range(1, 2, 3);
        assert_eq!(r, 0..=2);
    }

    #[test]
    fn test_truncated_range4() {
        let r = truncated_range(1, 2, 1);
        assert_eq!(r, 0..=0);
    }

    #[test]
    fn test_truncated_range5() {
        let r = truncated_range(4, 6, 10);
        assert_eq!(r, 3..=7);
    }
}
