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
pub struct AlmanacMapRange {
    pub src_start: i64,
    pub dest_start: i64,
    pub length: i64,
}

impl AlmanacMapRange {
    pub fn parse(input: &str) -> AlmanacMapRange {
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
}

#[derive(Debug, Clone)]
pub struct AlmanacMap {
    ranges: Vec<AlmanacMapRange>,
    pub name: Option<String>,
}

impl AlmanacMap {
    pub fn map(&self, source: u64) -> Option<u64> {
        let res = self
            .ranges
            .iter()
            .map(|r| {
                let diff = source as i64 - r.src_start;
                if diff >= 0 && diff <= r.length as i64 {
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

        if res.is_some() {
            res
        } else {
            Some(source)
        }
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
            let ranges: Vec<AlmanacMapRange> = chunk
                .iter()
                .skip(1)
                .map(|line| AlmanacMapRange::parse(line))
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
                    .fold(*seed, |acc, map_key| map_map[*map_key].map(acc).unwrap())
            })
            .collect();

        assert_eq!(output.into_iter().min().unwrap(), 621354867);
    }
}
