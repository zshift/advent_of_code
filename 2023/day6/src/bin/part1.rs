#[derive(Clone, Copy, Debug)]
struct Race {
    time_sec: u32,
    dist_mm: u32,
}

impl From<(u32, u32)> for Race {
    fn from(value: (u32, u32)) -> Self {
        Self {
            time_sec: value.0,
            dist_mm: value.1,
        }
    }
}

impl Race {
    fn ways_to_beat_record(&self) -> u32 {
        (1..self.time_sec)
            .map(|time_held| time_held * (self.time_sec - time_held))
            .filter(|&d| d > self.dist_mm)
            .count() as u32
    }
}

fn parse(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .filter_map(|t| t.parse::<u32>().ok());
    let dists = lines[1]
        .trim_end_matches("Distance:")
        .split_whitespace()
        .filter_map(|d| d.parse::<u32>().ok());

    times.zip(dists).map(Into::into).collect()
}

fn ways_to_beat_records(input: &str) -> u32 {
    let races = parse(input);
    races.iter().map(Race::ways_to_beat_record).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "\
    Time:      7  15   30
    Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(ways_to_beat_records(INPUT), 288);
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", ways_to_beat_records(input));
}
