use std::collections::HashMap;

use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<(Hand, i32)> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .iter()
        .map(|line| {
            let hand = Hand {
                cards: line.split_whitespace().nth(0).unwrap().to_string(),
            };
            let bid = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            return (hand, bid);
        })
        .collect::<Vec<_>>();
}

#[derive(Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_type(hand: &str) -> HandType {
    let mut cards = HashMap::new();
    for card in hand.chars() {
        let count = cards.entry(card).or_insert(0);
        *count += 1;
    }
    let mut counts = cards.values().collect::<Vec<_>>();

    match counts.len() {
        1 => return HandType::FiveOfAKind,
        2 => {
            counts.sort();
            if *counts[0] == 1 {
                return HandType::FourOfAKind;
            } else {
                return HandType::FullHouse;
            }
        }
        3 => {
            counts.sort();
            if *counts[0] == 1 {
                if *counts[1] == 1 {
                    return HandType::ThreeOfAKind;
                } else {
                    return HandType::TwoPair;
                }
            } else {
                return HandType::TwoPair;
            }
        }
        4 => return HandType::OnePair,
        _ => return HandType::HighCard,
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: String,
}

fn handtype_to_usize(t: &HandType) -> usize {
    match t {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0,
    }
}

fn get_type_joker(hand: &str) -> HandType {
    let mut cards = hand.chars().collect::<Vec<_>>();
    let mut map = HashMap::new();
    for card in cards.iter() {
        let count = map.entry(*card).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut replace_card = ' ';
    for (card, count) in map {
        if card == 'J' {
            continue;
        }
        if count > max {
            max = count;
            replace_card = card;
        }
    }
    if replace_card != ' ' {
        for card in cards.iter_mut() {
            if *card == 'J' {
                *card = replace_card;
            }
        }
    }
    let joker_type = get_type(cards.iter().collect::<String>().as_str());
    return joker_type;
}

fn part1(hand_bid_pairs: &Vec<(Hand, i32)>) -> i32 {
    let mut hands_by_type = vec![Vec::new(); 7];
    for (hand, bid) in hand_bid_pairs.iter() {
        let hand_type = get_type(hand.cards.as_str());
        hands_by_type[handtype_to_usize(&hand_type)].push((hand.clone(), *bid));
    }
    for card_vec in hands_by_type.iter_mut() {
        card_vec.sort_by(|h1, h2| {
            for (c0, c1) in h1.0.cards.chars().zip(h2.0.cards.chars()) {
                if c0 == c1 {
                    continue;
                }
                match (c0, c1) {
                    ('A', _) => return std::cmp::Ordering::Greater,
                    ('K', 'Q' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('Q', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('J', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('T', '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('9', '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('8', '7' | '6' | '5' | '4' | '3' | '2') => return std::cmp::Ordering::Greater,
                    ('7', '6' | '5' | '4' | '3' | '2') => return std::cmp::Ordering::Greater,
                    ('6', '5' | '4' | '3' | '2') => return std::cmp::Ordering::Greater,
                    ('5', '4' | '3' | '2') => return std::cmp::Ordering::Greater,
                    ('4', '3' | '2') => return std::cmp::Ordering::Greater,
                    ('3', '2') => return std::cmp::Ordering::Greater,
                    _ => {}
                }
                match (c1, c0) {
                    ('A', _) => return std::cmp::Ordering::Less,
                    ('K', 'Q' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('Q', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('J', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Less
                    }
                    ('T', '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Less
                    }
                    ('9', '8' | '7' | '6' | '5' | '4' | '3' | '2') => {
                        return std::cmp::Ordering::Less
                    }
                    ('8', '7' | '6' | '5' | '4' | '3' | '2') => return std::cmp::Ordering::Less,
                    ('7', '6' | '5' | '4' | '3' | '2') => return std::cmp::Ordering::Less,
                    ('6', '5' | '4' | '3' | '2') => return std::cmp::Ordering::Less,
                    ('5', '4' | '3' | '2') => return std::cmp::Ordering::Less,
                    ('4', '3' | '2') => return std::cmp::Ordering::Less,
                    ('3', '2') => return std::cmp::Ordering::Less,
                    _ => {}
                }
            }
            panic!("Hands are equal");
        })
    }
    return hands_by_type
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, x)| (i + 1) as i32 * x.1)
        .sum();
}

fn part2(hand_bid_pairs: &Vec<(Hand, i32)>) -> i32 {
    let mut hands_by_type = vec![Vec::new(); 7];
    for (hand, bid) in hand_bid_pairs.iter() {
        let hand_type = get_type_joker(hand.cards.as_str());
        hands_by_type[handtype_to_usize(&hand_type)].push((hand.clone(), *bid));
    }
    for card_vec in hands_by_type.iter_mut() {
        card_vec.sort_by(|h1, h2| {
            for (c0, c1) in h1.0.cards.chars().zip(h2.0.cards.chars()) {
                if c0 == c1 {
                    continue;
                }
                match (c0, c1) {
                    ('A', _) => return std::cmp::Ordering::Greater,
                    ('K', 'Q' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('Q', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('T', '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('9', '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('8', '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Greater
                    }
                    ('7', '6' | '5' | '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Greater,
                    ('6', '5' | '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Greater,
                    ('5', '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Greater,
                    ('4', '3' | '2' | 'J') => return std::cmp::Ordering::Greater,
                    ('3', '2' | 'J') => return std::cmp::Ordering::Greater,
                    ('2', 'J') => return std::cmp::Ordering::Greater,
                    _ => {}
                }
                match (c1, c0) {
                    ('A', _) => return std::cmp::Ordering::Less,
                    ('K', 'Q' | 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('Q', 'T' | '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('T', '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('9', '8' | '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('8', '7' | '6' | '5' | '4' | '3' | '2' | 'J') => {
                        return std::cmp::Ordering::Less
                    }
                    ('7', '6' | '5' | '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Less,
                    ('6', '5' | '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Less,
                    ('5', '4' | '3' | '2' | 'J') => return std::cmp::Ordering::Less,
                    ('4', '3' | '2' | 'J') => return std::cmp::Ordering::Less,
                    ('3', '2' | 'J') => return std::cmp::Ordering::Less,
                    ('2', 'J') => return std::cmp::Ordering::Less,
                    _ => {}
                }
            }
            panic!("Hands are equal");
        })
    }
    return hands_by_type
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, x)| (i + 1) as i32 * x.1)
        .sum();
}

test!(
    part1 {
        "test_inputs/day07/test01.txt" => 6440
    },
    part2 {
        "test_inputs/day07/test01.txt" => 5905
    }
);
main!();
