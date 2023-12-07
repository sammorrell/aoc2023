use aoctk::io::read_string_col;
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
pub struct Scratchcard {
    pub game_id: u32,
    pub winning_numbers: Vec<u32>,
    pub numbers: Vec<u32>,
}

pub fn parse_cards_from_file(path: &Path) -> Vec<Scratchcard> {
    let split_re = Regex::new("[ ]+").unwrap();
    let card_re = Regex::new("Card[ ]+").unwrap();
    let card_lines = read_string_col(path).expect("Unable to read card file. ");
    card_lines
        .iter()
        .map(|line| {
            let line_segs: Vec<&str> = line.split(": ").collect();
            let game_id = card_re
                .replace(line_segs.first().unwrap(), "")
                .parse::<u32>()
                .unwrap();
            let number_strings: Vec<&str> = line_segs.last().unwrap().split(" | ").collect();
            let winning_numbers = split_re
                .split(
                    number_strings
                        .first()
                        .expect("No winning numbers found. ")
                        .trim(),
                )
                .map(|str| str.trim().parse().unwrap())
                .collect();
            let numbers = split_re
                .split(number_strings.last().expect("No numbers found. ").trim())
                .map(|str| str.trim().parse().unwrap())
                .collect();
            Scratchcard {
                game_id,
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_cards_from_file;
    use std::{collections::HashSet, path::Path};

    #[test]
    fn day4_part1() {
        let cards = parse_cards_from_file(Path::new("data/day4/data.txt"));

        let scores: Vec<u32> = cards
            .iter()
            .map(|card| {
                let winning_nums: HashSet<u32> =
                    HashSet::from_iter(card.winning_numbers.iter().cloned());
                let played_nums: HashSet<u32> = HashSet::from_iter(card.numbers.iter().cloned());
                let n_winning = winning_nums.intersection(&played_nums).count();
                if n_winning > 0 {
                    (0..n_winning - 1).fold(1, |accum, _| accum * 2)
                } else {
                    0
                }
            })
            .collect();

        assert_eq!(scores.iter().sum::<u32>(), 21158);
    }

    #[test]
    fn day4_part2() {
        let cards = parse_cards_from_file(Path::new("data/day4/data.txt"));
        let mut card_copies = vec![1; cards.len()];

        let total_cards: Vec<usize> = cards
            .iter()
            .enumerate()
            .map(|(start, card)| {
                let winning_nums: HashSet<u32> =
                    HashSet::from_iter(card.winning_numbers.iter().cloned());
                let played_nums: HashSet<u32> = HashSet::from_iter(card.numbers.iter().cloned());
                let n_matching = winning_nums.intersection(&played_nums).count();

                for offset in 0..n_matching {
                    card_copies[start + offset + 1] += card_copies[start]
                }
                card_copies[start]
            })
            .collect();

        assert_eq!(total_cards.iter().sum::<usize>(), 6050769);
    }
}
