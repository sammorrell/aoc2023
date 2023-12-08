use std::{cmp::Ordering, collections::HashMap};

const INPUT: &str = include_str!("../../data/day7/data.txt");
const TEST_INTPUT: &str = include_str!("../../data/day7/reddit.txt");

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
        
        let mut char_occurances_vec = char_occurances.values().collect::<Vec<&u64>>();
        char_occurances_vec.sort();
        match char_occurances_vec.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard(input.clone()),
            [1, 1, 1, 2] => HandType::OnePair(input.clone()),
            [1, 2, 2] => HandType::TwoPair(input.clone()),
            [1, 1, 3] => HandType::ThreeOfAKind(input.clone()),
            [2, 3] => HandType::FullHouse(input.clone()),
            [1, 4] => HandType::FourOfAKind(input.clone()),
            [5] => HandType::FiveOfAKind(input.clone()),
            _ => { 
                println!("{:?}", char_occurances_vec);
                panic!("Invalid hand");
            },
        }
    }

    pub fn ranking(&self) -> usize {
        match self {
            HandType::FiveOfAKind(_) => 7,
            HandType::FourOfAKind(_) => 6,
            HandType::FullHouse(_) => 5,
            HandType::ThreeOfAKind(_) => 4,
            HandType::TwoPair(_) => 3,
            HandType::OnePair(_) => 2,
            HandType::HighCard(_) => 1,
        }
    }

    pub fn cards(&self) -> &String {
        match self {
            HandType::FiveOfAKind(cards) => cards,
            HandType::FourOfAKind(cards) => cards,
            HandType::FullHouse(cards) => cards,
            HandType::ThreeOfAKind(cards) => cards,
            HandType::TwoPair(cards) => cards,
            HandType::OnePair(cards) => cards,
            HandType::HighCard(cards) => cards, 
        }
    }

    pub fn set_cards(&mut self, input_cards: &String) {
        match self {
            HandType::FiveOfAKind(cards) => *cards = input_cards.clone(),
            HandType::FourOfAKind(cards) => *cards = input_cards.clone(),
            HandType::FullHouse(cards) => *cards = input_cards.clone(),
            HandType::ThreeOfAKind(cards) => *cards = input_cards.clone(),
            HandType::TwoPair(cards) => *cards = input_cards.clone(),
            HandType::OnePair(cards) => *cards = input_cards.clone(),
            HandType::HighCard(cards) => *cards = input_cards.clone(), 
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
        match self.ranking().cmp(&other.ranking()) {
            Ordering::Equal => compare_cards(&self.cards(), &other.cards()),
            other => other,
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
        Ordering::Equal
    } else {
        out[0]
    }
}

pub fn optimise_next_joker(hand: &String) -> HandType {
    let mut char_occurances: HashMap<char, u64> = HashMap::new();
    for character in hand.chars() {
        char_occurances.insert(character, char_occurances.get(&character).unwrap_or(&0) + 1);
    }
    
    // This isn't nice, but it works for parts 2. 
    // This took me so long to fix - I should have read the instructions more carefully. 
    // Also, should not have over engineered the solution.
    char_occurances.remove(&'J');
    match char_occurances.iter().max_by_key(|entry | entry.1) {
        Some((c, _)) => {
            let mut optim_hand = HandType::parse(&hand.replace("J", &c.to_string()));
            optim_hand.set_cards(hand);
            optim_hand
        },
        None => HandType::FiveOfAKind("JJJJJ".to_string()),
    }
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
    #[ignore = "You need to change the CHAR_ORDER to CHAR_ORDER_PT2 to run this test"]
    fn day7_part2() {
        let (hands, bids): (Vec<String>, Vec<u64>) = INPUT
            .split("\n")
            .map(|line| {
                let segs: Vec<&str> = line.split_whitespace().collect();
                let bid: u64 = segs[1].parse().unwrap();
                (String::from(segs[0]), bid)
            })
            .unzip();
        let opt_hands = hands.iter().map(|hand| optimise_next_joker(hand)).collect::<Vec<HandType>>();
        let ranks = rank(&opt_hands);
        let winnings: u64 = bids
            .into_iter()
            .zip(&ranks)
            .map(|(bid, rank)| bid * *rank as u64)
            .sum();
        
        assert_eq!(winnings, 246894760);
    }
}
