use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input.txt");
    let lines = input.lines().collect::<Vec<_>>();
    let time: u64 = lines[0]
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse()?;

    let distance: u64 = lines[1]
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse()?;

    print!("{}", ways_to_beat_record(time, distance));

    Ok(())
}

fn ways_to_beat_record(time: u64, distance: u64) -> usize {
    use rayon::prelude::*;
    (1..time)
        .into_par_iter()
        .map(|time_held| time_held * (time - time_held))
        .filter(|&d| d > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(ways_to_beat_record(71530, 940200), 71503);
    }
}
