use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../../data/day7/data.txt");

const CHAR_ORDER: &str = "23456789TJQKA";
const CHAR_ORDER_PT2: &str = "J23456789TQKA";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandType {
    /// All five cards are the same label
    FiveOfAKind(String),
    /// Where four cards have the same label and one card has a different label.
    FourOfAKind(String),
    /// Three cards have the same label, and the remaining two cards share a different label
    FullHouse(String),
    /// three cards have the same label, and the remaining two cards are each different from any other card in the hand
    ThreeOfAKind(String),
    // Two cards share one label, two other cards share a second label, and the remaining card has a third label
    TwoPair(String),
    // Two cards share one label, and the other three cards have a different label from the pair and each other
    OnePair(String),
    /// All cards labels are distinct.
    HighCard(String),
}

impl HandType {
    pub fn parse(input: &String) -> HandType {
        let mut char_occurances: HashMap<char, u64> = HashMap::new();
        for character in input.chars() {
            char_occurances.insert(character, char_occurances.get(&character).unwrap_or(&0) + 1);
        }

        let n_chars = char_occurances.keys().len();
        match n_chars {
            1 => HandType::FiveOfAKind(input.clone()),
            2 => {
                if *char_occurances.values().max().unwrap() == 4 {
                    HandType::FourOfAKind(input.clone())
                } else if *char_occurances.values().min().unwrap() == 2 {
                    HandType::FullHouse(input.clone())
                } else {
                    HandType::ThreeOfAKind(input.clone())
                }
            }
            3 => {
                if *char_occurances.values().max().unwrap() == 3 {
                    HandType::ThreeOfAKind(input.clone())
                } else {
                    HandType::TwoPair(input.clone())
                }
            }
            4 => HandType::OnePair(input.clone()),
            5 => HandType::HighCard(input.clone()),
            _ => panic!("Invalid hand"),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HandType::FiveOfAKind(this_str), HandType::FiveOfAKind(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::FiveOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::FiveOfAKind(_)) => std::cmp::Ordering::Less,
            (HandType::FourOfAKind(this_str), HandType::FourOfAKind(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::FourOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::FourOfAKind(_)) => std::cmp::Ordering::Less,
            (HandType::FullHouse(this_str), HandType::FullHouse(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::FullHouse(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::FullHouse(_)) => std::cmp::Ordering::Less,
            (HandType::ThreeOfAKind(this_str), HandType::ThreeOfAKind(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::ThreeOfAKind(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::ThreeOfAKind(_)) => std::cmp::Ordering::Less,
            (HandType::TwoPair(this_str), HandType::TwoPair(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::TwoPair(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::TwoPair(_)) => std::cmp::Ordering::Less,
            (HandType::OnePair(this_str), HandType::OnePair(other_str)) => {
                compare_cards(&this_str, other_str)
            }
            (HandType::OnePair(_), _) => std::cmp::Ordering::Greater,
            (_, HandType::OnePair(_)) => std::cmp::Ordering::Less,
            (HandType::HighCard(this_str), HandType::HighCard(other_str)) => {
                compare_cards(&this_str, other_str)
            }
        }
    }
}

pub fn rank(hands: &Vec<HandType>) -> Vec<usize> {
    let mut ranks: Vec<usize> = vec![1; hands.len()];
    for (i, hand) in hands.iter().enumerate() {
        for (j, other_hand) in hands.iter().enumerate() {
            if i == j {
                continue;
            }
            if hand > other_hand {
                ranks[i] += 1;
            }
        }
    }
    ranks
}

pub fn compare_cards(hand1: &String, hand2: &String) -> std::cmp::Ordering {
    let out: Vec<Ordering> = hand1
        .chars()
        .into_iter()
        .zip(hand2.chars().into_iter())
        .map(|(c1, c2)| {
            CHAR_ORDER
                .chars()
                .position(|c| c == c1)
                .unwrap()
                .cmp(&CHAR_ORDER.chars().position(|c| c == c2).unwrap())
        })
        .filter(|order| *order != Ordering::Equal)
        .collect();

    if out.len() == 0 {
        Ordering::Greater
    } else {
        out[0]
    }
}

pub fn optimise_next_joker(hand: &String, start: usize) -> String {
    let joker_index = hand.chars().skip(start).position(|c| c == 'J');
    let mut best = hand.clone();
    match joker_index {
        Some(index) => {
            for c in CHAR_ORDER.chars() {
                let mut new_hand = hand.clone();
                new_hand
                .replace_range(start+index..start+index + 1, &c.to_string());
                new_hand = optimise_next_joker(&new_hand, start+index + 1);              

                if HandType::parse(&new_hand) > HandType::parse(&best) {
                    best = new_hand;
                }
            }
        }
        None => {},
    };
    best
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_part1() {
        let (hands, bids): (Vec<String>, Vec<u64>) = INPUT
            .split("\n")
            .map(|line| {
                let segs: Vec<&str> = line.split_whitespace().collect();
                let bid: u64 = segs[1].parse().unwrap();
                (String::from(segs[0]), bid)
            })
            .unzip();
        
        let hand_types: Vec<HandType> = hands.iter().map(HandType::parse).collect();

        let ranks = rank(&hand_types);
        let winnings: u64 = bids
            .into_iter()
            .zip(&ranks)
            .map(|(bid, rank)| bid * *rank as u64)
            .sum();

        assert_eq!(winnings, 246912307);
    }

    #[test]
    #[ignore = "Not working yet"]
    fn day7_part2() {
        let (hands, bids): (Vec<String>, Vec<u64>) = INPUT
            .split("\n")
            .map(|line| {
                let segs: Vec<&str> = line.split_whitespace().collect();
                let bid: u64 = segs[1].parse().unwrap();
                (String::from(segs[0]), bid)
            })
            .unzip();
        let opt_hands = hands.iter().map(|hand| optimise_next_joker(hand, 0)).collect::<Vec<String>>();
        let hand_types: Vec<HandType> = opt_hands.iter().map(HandType::parse).collect();
        let ranks = rank(&hand_types);
        let winnings: u64 = bids
            .into_iter()
            .zip(&ranks)
            .map(|(bid, rank)| bid * *rank as u64)
            .sum();
        
        assert_eq!(winnings, 5);
    }
}
