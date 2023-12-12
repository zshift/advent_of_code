use anyhow::{anyhow, Error, Result};
use std::{collections::HashMap, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", score(&parse(input)).iter().sum::<u64>());
}

#[derive(Clone, Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl FromStr for Play {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_whitespace();
        let hand = parts.next().ok_or(anyhow!("Missing hand"))?.parse()?;
        let bid = parts
            .next()
            .ok_or(anyhow!("Missing bid"))?
            .parse()
            .map_err(|_| anyhow!("Invalid bid"))?;

        Ok(Self { hand, bid })
    }
}

#[derive(Clone, Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            cards: s.chars().filter_map(|c| c.try_into().ok()).collect(),
        })
    }
}

impl PartialEq for Hand {
    // TODO: Very expensive eq impl.
    fn eq(&self, other: &Self) -> bool {
        let mut s = self.cards.clone();
        s.sort();

        let mut o = other.cards.clone();
        o.sort();

        s == o
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type: HandType = self.into();
        let other_type: HandType = other.into();

        if self_type == other_type {
            self.cards.cmp(&other.cards)
        } else {
            self_type.cmp(&other_type)
        }
    }
}

#[cfg(test)]
mod hand_tests {
    use super::*;

    #[test]
    fn test_cmp() {
        let hand1 = "AAAAT".parse::<Hand>().unwrap();
        let hand2 = "AAAA9".parse::<Hand>().unwrap();

        assert!(hand1 > hand2);
    }

    #[test]
    fn test_eq() {
        let hand1 = "AAAAT".parse::<Hand>().unwrap();
        let hand2 = "AAAA9".parse::<Hand>().unwrap();
        let hand3 = "AATAA".parse::<Hand>().unwrap();

        assert_eq!(hand1, hand1);
        assert_eq!(hand1, hand3);
        assert_ne!(hand1, hand2);
    }

    #[test]
    fn test_ord() {
        let hand1 = "AAAAT".parse::<Hand>().unwrap();
        let hand2 = "AAAA9".parse::<Hand>().unwrap();

        let mut hands = vec![hand1.clone(), hand2.clone()];
        hands.sort();

        assert_eq!(hands, vec![hand2, hand1]);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err(anyhow!("Invalid card")),
        }
    }
}

#[cfg(test)]
mod card_tests {
    use super::*;

    #[test]
    fn test_ord() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Eight > Card::Seven);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Six > Card::Five);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Three > Card::Two);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut counts = HashMap::new();
        for card in value.cards.clone() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by_key(|(_, count)| *count);
        counts.reverse();

        let sl = counts.as_slice();

        match sl {
            [(_card, 5)] => Self::FiveOfAKind,
            [(_card, 4), _] => Self::FourOfAKind,
            [(_card1, 3), (_card2, 2)] => Self::FullHouse,
            [(_card, 3), ..] => Self::ThreeOfAKind,
            [(_card1, 2), (_card2, 2), _] => Self::TwoPair,
            [(_card, 2), ..] => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl From<Hand> for HandType {
    fn from(value: Hand) -> Self {
        Self::from(&value)
    }
}

#[cfg(test)]
mod hand_type_tests {
    use super::{Hand, HandType};
    use anyhow::Result;

    #[test]
    fn test_parse_into() -> Result<()> {
        let ht: HandType = "AAAAA".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::FiveOfAKind);

        let ht: HandType = "AAAAQ".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::FourOfAKind);

        let ht: HandType = "AAAQQ".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::FullHouse);

        let ht: HandType = "AAAKQ".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::ThreeOfAKind);

        let ht: HandType = "AAKKQ".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::TwoPair);

        let ht: HandType = "AAKQJ".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::OnePair);

        let ht: HandType = "AKQJT".parse::<Hand>()?.into();
        assert_eq!(ht, HandType::HighCard);

        Ok(())
    }

    #[test]
    fn test_ord() -> Result<()> {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::OnePair);
        assert!(HandType::OnePair > HandType::HighCard);

        Ok(())
    }
}

fn parse(input: &str) -> Vec<Play> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

// sorts plays by rank
fn score(plays: &[Play]) -> Vec<u64> {
    let mut plays = plays.to_vec();
    plays.sort_by(|a, b| a.hand.cmp(&b.hand));
    plays
        .iter()
        .enumerate()
        .map(|(i, p)| (i as u64 + 1) * p.bid as u64)
        .collect()
}

#[cfg(test)]
mod rank_tests {
    use super::{parse, score};

    static INPUT: &str = "\
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_score() {
        let plays = parse(INPUT);
        assert_eq!(
            score(&plays),
            vec![765 * 1, 220 * 2, 28 * 3, 684 * 4, 483 * 5]
        );
    }
}
