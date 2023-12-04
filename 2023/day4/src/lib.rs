use anyhow::{anyhow, Error, Result};

use std::{collections::HashMap, str::FromStr};

pub struct Scratchcard {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Scratchcard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(':');
        let mut card_split = split
            .next()
            .ok_or(anyhow!("No card id found"))?
            .split_whitespace();

        let _ = card_split.next().ok_or(anyhow!("No card found"))?; // Card
        let id: u32 = card_split.next().ok_or(anyhow!("No id found"))?.parse()?;

        let winning_numbers = split
            .next()
            .ok_or(anyhow!("No winning numbers and numbers found"))?;

        let mut split = winning_numbers.split('|').map(str::trim);
        let winning = split.next().ok_or(anyhow!("No winning numbers found"))?;
        let numbers = split.next().ok_or(anyhow!("No numbers found"))?;

        let winning: Vec<u32> = winning
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let numbers: Vec<u32> = numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Scratchcard {
            id,
            winning,
            numbers,
        })
    }
}

impl Scratchcard {
    pub fn matches(&self) -> u32 {
        let mut matches: u32 = 0;
        for number in &self.numbers {
            if self.winning.contains(number) {
                matches += 1;
            }
        }

        matches
    }

    pub fn points(&self) -> u32 {
        let matches = self.matches();

        if matches == 0 {
            0
        } else {
            2u32.pow(matches - 1)
        }
    }
}

pub struct Game {
    cards: Vec<Scratchcard>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.lines().map(|line| line.trim().parse().unwrap()).collect();

        Ok(Game { cards })
    }
}

impl Game {
    pub fn points(&self) -> u32 {
        self.cards.iter().map(Scratchcard::points).sum()
    }

    // For each card, find the number of matches.
    // For each x matches, the following x cards are copied.
    // Find the total number of cards.
    pub fn total_scratchcards(&self) -> u32 {
        let mut copies: HashMap<u32, u32> = HashMap::new();

        self.cards.iter().for_each(|card| {
            let id = card.id;
            let duplicates = copies.entry(id).or_insert(1);
            let matches = card.matches();

            let duplicates = *duplicates;

            for i in 1..=matches {
                *copies.entry(id + i).or_insert(1) += duplicates;
            }
        });

        copies.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    static INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                          Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                          Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                          Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                          Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                          Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let scratchcard: Game = INPUT.parse().unwrap();
        assert_eq!(scratchcard.points(), 13);
    }

    #[test]
    fn test_part2() {
        let scratchcard: Game = INPUT.parse().unwrap();
        assert_eq!(scratchcard.total_scratchcards(), 30);
    }
}
