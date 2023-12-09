use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<Vec<i64>> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
}

fn newton_extrapolation(y: &Vec<i64>) -> i64 {
    let mut dp = Vec::with_capacity(y.len());
    for yi in y.iter() {
        dp.push(*yi);
    }

    for i in (0..y.len() - 1).rev() {
        let mut tmp1 = dp[i + 1];
        for j in (0..i + 1).rev() {
            let tmp0 = dp[j];
            dp[j] = tmp1 - tmp0;
            tmp1 = tmp0;
        }
    }
    return dp.iter().sum();
}

fn part1(input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for row in input.iter() {
        sum += newton_extrapolation(row);
    }
    return sum;
}

fn part2(_input: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for row in _input.iter() {
        let rev = row.iter().rev().map(|a| a.clone()).collect::<Vec<i64>>();
        sum += newton_extrapolation(&rev);
    }
    return sum;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 1] = [("test_inputs/day09/test01.txt", 114)];
    pub const PART2_INPUTS: [(&str, i64); 1] = [("test_inputs/day09/test01.txt", 2)];
}

test!();
main!();
