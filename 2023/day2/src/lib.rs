use std::str::FromStr;

use regex::Regex;

type Error = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Pull {
    pub number: u32,
    pub color: Color,
}

impl Pull {
    pub fn is_valid(&self) -> bool {
        match self.color {
            Color::Red => self.number <= 12,
            Color::Green => self.number <= 13,
            Color::Blue => self.number <= 14,
        }
    }
}

impl FromStr for Pull {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+) (red|green|blue)")?;
        let captures = re.captures(s.trim()).unwrap();
        let number = captures.get(1).unwrap().as_str().parse::<u32>()?;
        let color = captures.get(2).unwrap().as_str().parse::<Color>()?;
        Ok(Pull { number, color })
    }
}

#[derive(Debug)]
pub struct Set {
    pub pulls: Vec<Pull>,
}

impl Set {
    pub fn is_valid(&self) -> bool {
        self.pulls.iter().all(Pull::is_valid)
    }

    pub fn min_each_color(&self) -> (u32, u32, u32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pull in &self.pulls {
            match pull.color {
                Color::Red => red = red.max(pull.number),
                Color::Green => green = green.max(pull.number),
                Color::Blue => blue = blue.max(pull.number),
            }
        }

        (red, green, blue)
    }
}

impl FromStr for Set {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pulls = s
            .split(", ")
            .map(Pull::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>();
        Ok(Set { pulls })
    }
}

#[derive(Debug)]
pub struct Game {
    pub number: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.sets.iter().all(Set::is_valid)
    }

    pub fn min_each_color(&self) -> (u32, u32, u32) {
        self.sets.iter().map(|set| set.min_each_color()).fold(
            (0, 0, 0),
            |(red, green, blue), (red2, green2, blue2)| {
                (red.max(red2), green.max(green2), blue.max(blue2))
            },
        )
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(": ").collect::<Vec<_>>();
        let number = split[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<u32>()
            .unwrap();

        let sets = split[1]
            .split("; ")
            .map(Set::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Ok(Game { number, sets })
    }
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Invalid color".into()),
        }
    }
}
