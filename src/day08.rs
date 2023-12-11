use crate::utils::{main, test};
use std::collections::HashMap;

fn get_contents(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let lines = crate::utils::read_lines(filename);
    let directions = lines[0].clone();
    let dir_map = lines[2..]
        .iter()
        .map(|s| {
            let mut elements = s
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>();
            elements[2] = elements[2].replace(['(', ',', ')'], "");
            elements[3] = elements[3].replace(['(', ',', ')'], "");
            return (
                elements[0].clone(),
                (elements[2].clone(), elements[3].clone()),
            );
        })
        .collect::<HashMap<String, (String, String)>>();
    return (directions, dir_map);
}

fn part1(input: &(String, HashMap<String, (String, String)>)) -> i64 {
    let (directions, map) = input;
    let mut count = 0;

    let mut current = "AAA".to_string();
    let mut dir_indx = 0;
    let mut current_dir = directions.chars().nth(dir_indx).unwrap();

    while current != "ZZZ" {
        current = match current_dir {
            'R' => map.get(&current).unwrap().1.clone(),
            'L' => map.get(&current).unwrap().0.clone(),
            _ => panic!("Invalid direction"),
        };
        dir_indx = (dir_indx + 1) % directions.len();
        current_dir = directions.chars().nth(dir_indx).unwrap();
        count += 1;
    }
    return count;
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut h = std::cmp::max(a, b);
    let mut l = std::cmp::min(a, b);

    while l != 0 {
        let temp = l;
        l = h % l;
        h = temp;
    }

    return h;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn part2(input: &(String, HashMap<String, (String, String)>)) -> i64 {
    let (directions, map) = input;
    let start_nodes = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.clone())
        .collect::<Vec<String>>();
    let mut counts = Vec::new();

    for node in start_nodes.iter() {
        let mut current = node.clone();
        let mut dir_indx = 0;
        let mut current_dir = directions.chars().nth(dir_indx).unwrap();
        let mut count = 0;
        while !current.ends_with("Z") {
            current = match current_dir {
                'R' => map.get(&current).unwrap().1.clone(),
                'L' => map.get(&current).unwrap().0.clone(),
                _ => panic!("Invalid direction"),
            };
            dir_indx = (dir_indx + 1) % directions.len();
            current_dir = directions.chars().nth(dir_indx).unwrap();
            count += 1;
        }
        counts.push(count);
    }
    let count = counts.iter().fold(1, |acc, x| lcm(acc, *x as i64));
    return count;
}

test!(
    part1 {
        "test_inputs/day08/test01.txt" => 2,
        "test_inputs/day08/test02.txt" => 6
    },
    part2 {
        "test_inputs/day08/test03.txt" => 6
    }
);
main!();
