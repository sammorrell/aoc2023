use regex::Regex;
use std::{num::ParseIntError, path::Path};

#[derive(Debug, Clone)]
pub struct RaceRecord {
    time_ms: u64,
    dist: u64,
}

pub fn parse_races(path: &Path) -> Result<Vec<RaceRecord>, aoctk::err::Error> {
    let split_re = Regex::new("[ ]+").unwrap();
    let lines = aoctk::io::read_string_col(path)?;
    let times: Vec<u64> = split_re
        .split(lines[0].replace("Time: ", "").trim())
        .map(|str| str.parse::<u64>())
        .map(std::convert::identity)
        .collect::<Result<Vec<u64>, _>>()?;
    let distances: Vec<u64> = split_re
        .split(lines[1].replace("Distance: ", "").trim())
        .map(|str| str.parse::<u64>())
        .map(std::convert::identity)
        .collect::<Result<Vec<u64>, _>>()?;

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(time_ms, dist)| RaceRecord { time_ms, dist })
        .collect())
}

pub fn parse_race_pt2(path: &Path) -> RaceRecord {
    let lines = aoctk::io::read_string_col(path).unwrap();
    let time: u64 = lines[0]
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let disance: u64 = lines[1]
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    RaceRecord {
        time_ms: time,
        dist: disance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_part1() {
        let race_records = parse_races(Path::new("data/day6/data.txt")).unwrap();
        let res: Vec<u64> = race_records
            .iter()
            .map(|race| {
                // Spotted this was a quadratic. Yay!
                let discrim = (race.time_ms.pow(2) - 4 * (race.dist)) as f64;
                let x1 = (race.time_ms as f64 + discrim.sqrt()) / 2.0;
                let x2 = (race.time_ms as f64 - discrim.sqrt()) / 2.0;
                x1 as u64 - x2 as u64
            })
            .collect();

        assert_eq!(res.iter().product::<u64>(), 160816);
    }

    #[test]
    fn day6_part2() {
        let race_record = parse_race_pt2(Path::new("data/day6/data.txt"));
        let discrim = (race_record.time_ms.pow(2) - 4 * (race_record.dist)) as f64;
        let x1 = (race_record.time_ms as f64 + discrim.sqrt()) / 2.0;
        let x2 = (race_record.time_ms as f64 - discrim.sqrt()) / 2.0;
        let res = x1 as u64 - x2 as u64;

        assert_eq!(res, 46561107);
    }
}
