use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<(Vec<i32>, Vec<i32>)> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .iter()
        .map(|line| {
            let mut toks = line.split(':');
            let _ = toks.next().unwrap().trim();
            let card_str = toks.next().unwrap().trim();
            let mut toks = card_str.split('|');
            let winning_cards_str = toks.next().unwrap().trim().split(' ');
            let mut winning_cards = Vec::new();

            for card_num in winning_cards_str {
                match i32::from_str_radix(card_num, 10) {
                    Ok(num) => winning_cards.push(num),
                    Err(_) => {}
                }
            }

            let dealt_cards_str = toks.next().unwrap().trim().split(' ');
            let mut dealt_cards = Vec::new();

            for card_num in dealt_cards_str {
                match i32::from_str_radix(card_num, 10) {
                    Ok(num) => dealt_cards.push(num),
                    Err(_) => {}
                }
            }

            return (winning_cards, dealt_cards);
        })
        .collect();
}

fn part1(input: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut sum = 0;
    for (winning_cards, dealt_cards) in input {
        let mut count = 0;
        for card in dealt_cards {
            if winning_cards.contains(card) {
                count += 1;
            }
        }
        if count > 0 {
            sum += 1 << (count - 1);
        }
    }
    return sum;
}

fn part2(input: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut copies = vec![1; input.len()];
    let mut sum = 0;
    for (i, (winning_cards, dealt_cards)) in input.iter().enumerate() {
        let c = copies[i];
        sum += c;
        let mut count = 0;
        for card in dealt_cards {
            if winning_cards.contains(card) {
                count += 1;
            }
        }
        if count > 0 {
            for j in i + 1..i + 1 + count {
                copies[j] += c;
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i32); 1] = [("test_inputs/day04/test01.txt", 13)];
    pub const PART2_INPUTS: [(&str, i32); 1] = [("test_inputs/day04/test01.txt", 30)];
}

test!();
main!();
