use regex::Regex;
use std::path::Path;

pub const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

#[derive(Debug, Clone)]
pub struct Range {
    pub src_start: i64,
    pub dest_start: i64,
    pub length: i64,
}

impl Range {
    pub fn parse(input: &str) -> Range {
        let split_re = Regex::new("[ ]+").unwrap();
        let segs: Vec<u64> = split_re
            .split(input)
            .map(|str| str.parse::<u64>().unwrap())
            .collect();
        Self {
            src_start: segs[1] as i64,
            dest_start: segs[0] as i64,
            length: segs[2] as i64,
        }
    }

    pub fn contains(&self, val: i64) -> bool {
        self.src_start < val  && val < self.src_start + self.length
    }
}

#[derive(Debug, Clone)]
pub struct AlmanacMap {
    ranges: Vec<Range>,
    pub name: Option<String>,
}

impl AlmanacMap {
    pub fn map(&self, source: u64) -> u64 {
        let res = self
            .ranges
            .iter()
            .map(|r| {
                let diff = source as i64 - r.src_start;
                if diff >= 0 && diff < r.length as i64 {
                    let res = (r.dest_start + diff) as u64;
                    Some(res)
                } else {
                    None
                }
            })
            .filter_map(std::convert::identity)
            .collect::<Vec<u64>>()
            .into_iter()
            .next();

        res.unwrap_or(source)
    }

    pub fn rev_map(&self, dest: u64) -> u64 {
        let res = self
            .ranges
            .iter()
            .rev()
            .map(|r| {
                let diff = dest as i64 - r.dest_start;
                if diff >= 0 && diff < r.length as i64 {
                    let res = (r.src_start + diff) as u64;
                    Some(res)
                } else {
                    None
                }
            })
            .filter_map(std::convert::identity)
            .collect::<Vec<u64>>()
            .into_iter()
            .next();

        res.unwrap_or(dest)
    }
}

pub fn parse_input(path: &Path) -> Result<(Vec<u64>, Vec<AlmanacMap>), aoctk::err::Error> {
    let split_re = Regex::new("[ ]+").unwrap();
    let input_chunks = aoctk::io::read_text_chunks(path)?;
    let seeds = split_re
        .split(
            input_chunks
                .first()
                .unwrap()
                .first()
                .unwrap()
                .replace("seeds: ", "")
                .as_str(),
        )
        .map(|str| str.parse::<u64>().unwrap())
        .collect();
    let maps: Vec<AlmanacMap> = input_chunks
        .iter()
        .skip(1)
        .map(|chunk| {
            let name = chunk.first().unwrap().replace(" map:", "");
            let ranges: Vec<Range> = chunk
                .iter()
                .skip(1)
                .map(|line| Range::parse(line))
                .collect();
            AlmanacMap {
                ranges: ranges,
                name: Some(name),
            }
        })
        .collect();
    Ok((seeds, maps))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn day5_part1() {
        let (seeds, maps) =
            parse_input(Path::new("data/day5/data.txt")).expect("Unable to parse input. ");
        let map_map: HashMap<_, _> = maps
            .into_iter()
            .map(|m| (m.name.clone().unwrap(), m.clone()))
            .collect();
        let output: Vec<u64> = seeds
            .iter()
            .map(|seed| {
                MAP_ORDER
                    .iter()
                    .fold(*seed, |acc, map_key| map_map[*map_key].map(acc))
            })
            .collect();

        assert_eq!(output.into_iter().min().unwrap(), 621354867);
    }

    #[test]
    #[ignore = "long running test"]
    fn day5_part2() {

        let (seeds, maps) =
            parse_input(Path::new("data/day5/data.txt")).expect("Unable to parse input. ");
        let seed_ranges: Vec<Range> = seeds
            .chunks(2)
            .into_iter()
            .map(|range| {
                Range { src_start: range[0] as i64, dest_start: 0, length: range[1] as i64 }
            })
            .collect();
        let map_map: HashMap<_, _> = maps
            .into_iter()
            .map(|m| (m.name.clone().unwrap(), m.clone()))
            .collect();
        let mut iter = (0..)
            .map(|dest_test| {
                if dest_test % 1000000 == 0 {
                    println!("{}", dest_test);
                }
                let res = MAP_ORDER
                    .iter()
                    .rev()
                    .fold(dest_test, |acc, map_key| map_map[*map_key].rev_map(acc));
                let maps_to_seed_range = seed_ranges 
                    .iter()
                    .map(|range| range.contains(res as i64))
                    .any(|in_range| in_range);
                if maps_to_seed_range {
                    Some(dest_test)
                } else {
                    None
                }
            })
            .filter(|item| item.is_some());
        let output = iter.next().unwrap();

        assert_eq!(output.unwrap(), 15880236);
    }
}
