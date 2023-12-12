use anyhow::Error;
use std::{ops::Range, str::FromStr};

mod utils;

use utils::Overlap;

trait MergeOverlap {
    fn merge_overlap(&self) -> Self;
}

impl MergeOverlap for Vec<Range<u64>> {
    fn merge_overlap(&self) -> Self {
        self.iter().fold(vec![], |mut acc, range| {
            if let Some(last) = acc.last_mut() {
                if last.overlaps(range) {
                    *last = last.merge(range);
                    return acc;
                }
            }

            acc.push(range.clone());
            acc
        })
    }
}

#[derive(Clone, Debug)]
pub struct RangeMap {
    pub dest: Range<u64>,
    pub src: Range<u64>,
}

impl FromStr for RangeMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let dest_start = split.next().unwrap().parse()?;
        let source_start = split.next().unwrap().parse()?;
        let length: u64 = split.next().unwrap().parse()?;

        Ok(RangeMap {
            dest: dest_start..(dest_start + length),
            src: source_start..(source_start + length),
        })
    }
}

impl RangeMap {
    pub fn lookup(&self, value: u64) -> Option<u64> {
        if self.src.contains(&value) {
            let offset = value - self.src.start;
            Some(self.dest.start + offset)
        } else {
            None
        }
    }

    // TODO: Go through all of the RangeMaps, and only the leftover ranges don't get mapped.
    // This should be Some((overlap, leftover)) or None if there is no overlap.
    pub fn map_onto(&self, input: Range<u64>) -> Option<Vec<Range<u64>>> {
        if !self.src.overlaps(&input) {
            return None;
        }

        // 4 cases of overlap:
        //
        //  * input within self
        //  * self within input
        //  * self starts before input
        //  * input start before self

        let mut results = if self.src.start <= input.start && self.src.end >= input.end {
            // input within self
            // Map[[start .... input_start...input_end...end]] -> map(input_start)..map(input_end)
            let start_diff = input.start.saturating_sub(self.src.start);
            let end_diff = self.src.end.saturating_sub(input.end);
            let start = self.dest.start + start_diff;
            let end = self.dest.end - end_diff;

            #[allow(clippy::single_range_in_vec_init)]
            {
                vec![start..end]
            }
        } else if self.src.start >= input.start && self.src.end <= input.end {
            // self within input.
            // input_start...Map[[start...end]]...input_end -> [input_start..src_start] [map(src_start)...map(src_end)] [src...input_end]
            let front = input.start..self.src.start;
            let middle = self.dest.start..self.dest.end;
            let end = self.src.end..input.end;

            vec![front, middle, end]
        } else if self.src.start >= input.start && self.src.end >= input.end {
            // input start before self
            let front = input.start..self.src.start;
            let end = {
                let diff = input.end - self.src.start;
                self.dest.start..(self.dest.start + diff)
            };
            vec![front, end]
        } else if self.src.start <= input.start && self.src.end <= input.end {
            // self starts before input
            let front = {
                let diff = self.src.end - input.start;
                (self.dest.end - diff)..self.dest.end
            };
            let end = self.src.end..input.end;
            vec![front, end]
        } else {
            unreachable!("Should have return if there was no overlap")
        };

        results.sort_by(|a, b| a.start.cmp(&b.start));
        Some(results.merge_overlap())
    }
}

#[cfg(test)]
mod range_map_tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lookup() -> Result<()> {
        let range_map: RangeMap = "0 10 10".parse()?;
        assert_eq!(range_map.lookup(0), None);
        assert_eq!(range_map.lookup(5), None);
        assert_eq!(range_map.lookup(10), Some(0));
        assert_eq!(range_map.lookup(11), Some(1));
        assert_eq!(range_map.lookup(19), Some(9));
        assert_eq!(range_map.lookup(20), None);
        Ok(())
    }

    #[test]
    fn test_map_onto() -> Result<()> {
        let range_map: RangeMap = "10 20 10".parse()?;
        assert_eq!(range_map.map_onto(1..5), None);
        assert_eq!(range_map.map_onto(22..28), Some(vec![12..18]));
        assert_eq!(range_map.map_onto(8..32), Some(vec![8..20, 30..32]));
        assert_eq!(range_map.map_onto(18..22), Some(vec![10..12, 18..20]));
        assert_eq!(range_map.map_onto(28..32), Some(vec![18..20, 30..32]));
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<RangeMap>,
    soil_to_fertilizer_map: Vec<RangeMap>,
    fertilizer_to_water_map: Vec<RangeMap>,
    water_to_light_map: Vec<RangeMap>,
    light_to_temperature_map: Vec<RangeMap>,
    temperature_to_humidity_map: Vec<RangeMap>,
    humidity_to_location_map: Vec<RangeMap>,
}

#[derive(Default)]
enum ParseState {
    #[default]
    Seeds,
    SeedToSoilMap,
    SoilToFertilizerMap,
    FertilizerToWaterMap,
    WaterToLightMap,
    LightToTemperatureMap,
    TemperatureToHumidityMap,
    HumidityToLocationMap,
    Done,
}

impl ParseState {
    pub fn next_category(&mut self) {
        *self = match self {
            ParseState::Seeds => ParseState::SeedToSoilMap,
            ParseState::SeedToSoilMap => ParseState::SoilToFertilizerMap,
            ParseState::SoilToFertilizerMap => ParseState::FertilizerToWaterMap,
            ParseState::FertilizerToWaterMap => ParseState::WaterToLightMap,
            ParseState::WaterToLightMap => ParseState::LightToTemperatureMap,
            ParseState::LightToTemperatureMap => ParseState::TemperatureToHumidityMap,
            ParseState::TemperatureToHumidityMap => ParseState::HumidityToLocationMap,
            ParseState::HumidityToLocationMap => ParseState::Done,
            ParseState::Done => ParseState::Done,
        }
    }
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut state = ParseState::default();
        let mut almanac = Almanac::default();
        let mut skip = false;

        for line in s.lines().map(str::trim) {
            if skip {
                skip = false;
                continue;
            }

            if line.is_empty() {
                state.next_category();
                // skip the header of the next category
                skip = true;
                continue;
            }

            match state {
                ParseState::Seeds => {
                    almanac.seeds = line[6..]
                        .split_whitespace()
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect();
                }
                ParseState::SeedToSoilMap => {
                    almanac.seed_to_soil_map.push(line.parse()?);
                }
                ParseState::SoilToFertilizerMap => {
                    almanac.soil_to_fertilizer_map.push(line.parse()?);
                }
                ParseState::FertilizerToWaterMap => {
                    almanac.fertilizer_to_water_map.push(line.parse()?);
                }
                ParseState::WaterToLightMap => {
                    almanac.water_to_light_map.push(line.parse()?);
                }
                ParseState::LightToTemperatureMap => {
                    almanac.light_to_temperature_map.push(line.parse()?);
                }
                ParseState::TemperatureToHumidityMap => {
                    almanac.temperature_to_humidity_map.push(line.parse()?);
                }
                ParseState::HumidityToLocationMap => {
                    almanac.humidity_to_location_map.push(line.parse()?);
                }
                ParseState::Done => return Ok(almanac),
            }
        }

        Ok(almanac)
    }
}

impl Almanac {
    pub fn lowest_location_that_needs_a_seed(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| {
                self.seed_to_soil_map
                    .iter()
                    .map(|map| map.lookup(seed))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(seed)
            })
            .map(|soil| {
                self.soil_to_fertilizer_map
                    .iter()
                    .map(|map| map.lookup(soil))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(soil)
            })
            .map(|fertilizer| {
                self.fertilizer_to_water_map
                    .iter()
                    .map(|map| map.lookup(fertilizer))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(fertilizer)
            })
            .map(|water| {
                self.water_to_light_map
                    .iter()
                    .map(|map| map.lookup(water))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(water)
            })
            .map(|light| {
                self.light_to_temperature_map
                    .iter()
                    .map(|map| map.lookup(light))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(light)
            })
            .map(|temperature| {
                self.temperature_to_humidity_map
                    .iter()
                    .map(|map| map.lookup(temperature))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(temperature)
            })
            .map(|humidity| {
                self.humidity_to_location_map
                    .iter()
                    .map(|map| map.lookup(humidity))
                    .fold(None, |a, b| a.or(b))
                    .unwrap_or(humidity)
            })
            .min()
            .unwrap()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Almanac2 {
    seeds: Vec<Range<u64>>,
    seed_to_soil_map: Vec<RangeMap>,
    soil_to_fertilizer_map: Vec<RangeMap>,
    fertilizer_to_water_map: Vec<RangeMap>,
    water_to_light_map: Vec<RangeMap>,
    light_to_temperature_map: Vec<RangeMap>,
    temperature_to_humidity_map: Vec<RangeMap>,
    humidity_to_location_map: Vec<RangeMap>,
}

impl From<Almanac> for Almanac2 {
    fn from(value: Almanac) -> Self {
        let mut seeds: Vec<Range<u64>> = value
            .seeds
            .windows(2)
            .step_by(2)
            .map(|window| {
                let start = window[0];
                let end = start + window[1];

                start..end
            })
            .collect();
        seeds.sort_by(|a, b| a.start.cmp(&b.start));

        Almanac2 {
            seeds,
            seed_to_soil_map: value.seed_to_soil_map,
            soil_to_fertilizer_map: value.soil_to_fertilizer_map,
            fertilizer_to_water_map: value.fertilizer_to_water_map,
            water_to_light_map: value.water_to_light_map,
            light_to_temperature_map: value.light_to_temperature_map,
            temperature_to_humidity_map: value.temperature_to_humidity_map,
            humidity_to_location_map: value.humidity_to_location_map,
        }
    }
}

impl FromStr for Almanac2 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let almanac: Almanac = s.parse()?;
        Ok(almanac.into())
    }
}

impl Almanac2 {
    pub fn maps(&self) -> Vec<Vec<RangeMap>> {
        vec![
            self.seed_to_soil_map.clone(),
            self.soil_to_fertilizer_map.clone(),
            self.fertilizer_to_water_map.clone(),
            self.water_to_light_map.clone(),
            self.light_to_temperature_map.clone(),
            self.temperature_to_humidity_map.clone(),
            self.humidity_to_location_map.clone(),
        ]
    }

    pub fn lowest_location_that_needs_a_seed(&self) -> u64 {
        let mut locations = self.maps().iter().fold(
            self.seeds.clone(),
            move |ranges: Vec<Range<u64>>, maps: &Vec<RangeMap>| {
                ranges
                    .iter()
                    .flat_map(move |range| {
                        let mut results = maps
                            .iter()
                            .filter_map(move |map| map.map_onto(range.clone()))
                            .flatten()
                            .collect::<Vec<_>>();

                        if results.is_empty() {
                            vec![range.clone()]
                        } else {
                            results.sort_by(|a, b| a.start.cmp(&b.start));
                            results.merge_overlap()
                        }
                    })
                    .collect()
            },
        );
        locations.sort_by(|a, b| a.start.cmp(&b.start));

        locations
            .iter()
            .filter(|x| x.start != 0)
            .map(|range| range.start)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    static INPUT: &str = "seeds: 79 14 55 13

                          seed-to-soil map:
                          50 98 2
                          52 50 48

                          soil-to-fertilizer map:
                          0 15 37
                          37 52 2
                          39 0 15

                          fertilizer-to-water map:
                          49 53 8
                          0 11 42
                          42 0 7
                          57 7 4

                          water-to-light map:
                          88 18 7
                          18 25 70

                          light-to-temperature map:
                          45 77 23
                          81 45 19
                          68 64 13

                          temperature-to-humidity map:
                          0 69 1
                          1 0 69

                          humidity-to-location map:
                          60 56 37
                          56 93 4";

    #[test]
    fn test_part1() -> Result<()> {
        let almanac: Almanac = INPUT.parse()?;
        assert_eq!(almanac.lowest_location_that_needs_a_seed(), 35);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let almanac: Almanac2 = INPUT.parse()?;
        assert_eq!(almanac.lowest_location_that_needs_a_seed(), 46);
        Ok(())
    }
}

// 50 98 2 -> if src between 98 and 100, map it to 50 to 52. otherwise
// 52 50 48 -> if src between 50 and 98, map it to 52 to 100. otherwise
// return the src back.
