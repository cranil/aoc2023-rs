use crate::utils::{main, test};

fn get_contents(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = crate::utils::read_lines(filename);
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    return (times, distances);
}

fn calculate_num_ways(total_time: i64, distance: i64) -> i64 {
    let discriminant = (total_time * total_time) - (distance * 4);
    let begin = (total_time as f64 - (discriminant as f64).sqrt()) / 2.0;
    let end = (total_time as f64 + (discriminant as f64).sqrt()) / 2.0;
    let mut num_ways = (end.floor() - begin.ceil()) as i64;
    if num_ways * num_ways == discriminant {
        num_ways -= 1;
    } else {
        num_ways += 1;
    }
    return num_ways;
}

fn part1(input: &(Vec<i32>, Vec<i32>)) -> i64 {
    let mut out = 1;
    let (times, distances) = input;
    for (total_time, distance) in times.iter().zip(distances.iter()) {
        let num_ways = calculate_num_ways(*total_time as i64, *distance as i64);
        out *= num_ways;
    }
    return out;
}

fn part2(input: &(Vec<i32>, Vec<i32>)) -> i64 {
    let (times, distances) = input;
    let mut actual_time = String::from("");
    let mut actual_distance = String::from("");
    // tooling huh... 🥲
    for (total_time, distance) in times.iter().zip(distances.iter()) {
        actual_time.push_str(&format!("{}", total_time));
        actual_distance.push_str(&format!("{}", distance));
    }

    let total_time = actual_time.parse::<i64>().unwrap();
    let distance = actual_distance.parse::<i64>().unwrap();
    return calculate_num_ways(total_time, distance);
}

test!(
    part1 {
        "test_inputs/day06/test01.txt" => 288
    },
    part2 {
        "test_inputs/day06/test01.txt" => 71503
    }
);
main!();
