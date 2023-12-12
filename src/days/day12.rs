use itertools::Itertools;
use regex::Regex;
use memoize::memoize;

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

/// This is the brute-force approach that I originally solves part 1 with.
/// This would not scale to part 2. 
#[memoize]
fn count_valid_combinations(row: String, broken_pattern: Vec<usize>) -> usize {
    let unknown_posns = row.match_indices("?").collect_vec();
    if unknown_posns.len() > 0 {
        let pos = unknown_posns.first().unwrap().0;
        let mut working_test = row.to_string();
        working_test.replace_range(pos..pos+1, ".");
        let mut broken_test = row.to_string();
        broken_test.replace_range(pos..pos+1, "#");

        count_valid_combinations(working_test, broken_pattern.clone()) + count_valid_combinations(broken_test, broken_pattern.clone())
    } else {
        if is_valid_row(&row, &broken_pattern) {
            1
        } else {
            0
        }
    }
}

/// Thanks to https://www.youtube.com/watch?v=g3Ms5e7Jdqo for explaining
/// this solution. Nice little logic puzzle. 
#[memoize]
fn count_combinations(row: String, pattern: Vec<usize>) -> usize {
    if row.chars().count() == 0 {
        // If we have reached the end of the row, 
        // and we still have numbers to match, then it is not a valid combination.
        return if pattern.len() == 0 { 1 } else { 0 }
    }
    
    if pattern.len() == 0 {
        // If after all of the numbers we are left with any non-matched, 
        // then it is not a valid combination. 
        return if row.contains("#") { 0 } else { 1 }
    }

    let mut count = 0;
    if ".?".contains(row.chars().nth(0).unwrap()) {
        count += count_combinations(row.chars().skip(1).collect(), pattern.clone());
    }

    if "#?".contains(row.chars().nth(0).unwrap()) {
        if pattern[0] <= row.chars().count() && 
            row.chars().take(pattern[0]).all(|c| c != '.') &&
            (pattern[0] == row.chars().count() || row.chars().nth(pattern[0]).unwrap() != '#') 
        {
            count += count_combinations(row.chars().skip(pattern[0] + 1).collect(), pattern[1..].to_vec());
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;


    const INPUT: &str = include_str!("../../data/day12/input.txt");

    #[test]
    fn day12_part1() {
        let (broken_map, broken_pattern) = parse_input(INPUT);
        let count: Vec<usize> = broken_map
            .par_iter()
            .zip(broken_pattern)
            .map(|(row, pattern)| count_combinations(row.to_string(), pattern))
            .collect();

        assert_eq!(count.iter().sum::<usize>(), 7939);
    }

    #[test]
    fn day12_part2() {
        let (broken_map, broken_pattern) = parse_input(INPUT);
        let count: Vec<usize> = broken_map
            .iter()
            .zip(broken_pattern)
            .map(|(row, pattern)| {
                let joined_row = row.clone() + &("?".to_string() + row).repeat(4);
                let join_broken_pattern = pattern.repeat(5);
                count_combinations(joined_row, join_broken_pattern)
            })
            .collect();

        assert_eq!(count.iter().sum::<usize>(), 850504257483930);

    }

    #[test]
    fn test_is_valid_row() {
        assert!(is_valid_row("#.#.###", &vec![1,1,3]));
        assert!(!is_valid_row("##..###", &vec![1,1,3]));

        assert!(is_valid_row("#....######..#####.", &vec![1,6,5]));
    }
}