use itertools::Itertools;
use regex::Regex;

const BROKEN_SEPARATOR: &str = "[.]+";

pub fn parse_input(input: &str) -> (Vec<String>, Vec<Vec<usize>>) {
    let lines = input.lines().collect_vec();
    lines
    .iter()
    .map(|line| {
        let segs: (&str, &str) = line.split_once(" ").unwrap();
        let broken_map = segs.0.to_string();
        let broken_pattern: Vec<usize> = segs.1.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

        (broken_map, broken_pattern)
    })
    .unzip()
}

pub fn is_valid_row(row: &str, broken_pattern: &Vec<usize>) -> bool {
    let re = Regex::new(BROKEN_SEPARATOR).unwrap();
    let groups: Vec<&str> = re.split(row).filter(|group| group.len() > 0).collect();
    if groups.len() != broken_pattern.len() {
        return false;
    }

    groups
        .iter()
        .zip(broken_pattern.iter())
        .all(|(group, broken)| group.len() == *broken)
}

fn count_valid_combinations(row: &str, broken_pattern: &Vec<usize>) -> usize {
    let unknown_posns = row.match_indices("?").collect_vec();
    if unknown_posns.len() > 0 {
        let pos = unknown_posns.first().unwrap().0;
        let mut working_test = row.to_string();
        working_test.replace_range(pos..pos+1, ".");
        let mut broken_test = row.to_string();
        broken_test.replace_range(pos..pos+1, "#");

        count_valid_combinations(&working_test, broken_pattern) + count_valid_combinations(&broken_test, broken_pattern)
    } else {
        if is_valid_row(row, broken_pattern) {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indicatif::ParallelProgressIterator;
    use rayon::prelude::*;


    const INPUT: &str = include_str!("../../data/day12/input.txt");

    #[test]
    fn day12_part1() {
        let (broken_map, broken_pattern) = parse_input(INPUT);
        let count: Vec<usize> = broken_map
            .par_iter()
            .progress_count(broken_map.len() as u64)
            .zip(broken_pattern)
            .map(|(row, pattern)| count_valid_combinations(row, &pattern))
            .collect();

        assert_eq!(count.iter().sum::<usize>(), 7939);
    }

    #[test]
    fn test_is_valid_row() {
        assert!(is_valid_row("#.#.###", &vec![1,1,3]));
        assert!(!is_valid_row("##..###", &vec![1,1,3]));

        assert!(is_valid_row("#....######..#####.", &vec![1,6,5]));
    }
}