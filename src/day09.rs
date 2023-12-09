use crate::{
    algos::Grid,
    utils::{main, test},
};

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
    let mut g = Grid::new(y.len(), y.len());
    for i in 0..y.len() {
        g.set(0, i, y[i]);
    }
    for i in 1..y.len() {
        let mut pval = g.at(i - 1, 1).unwrap() - g.at(i - 1, 0).unwrap();
        let mut eq = true;
        g.set(i, 0, pval);

        for j in 1..y.len() - i {
            let val = g.at(i - 1, j + 1).unwrap() - g.at(i - 1, j).unwrap();
            g.set(i, j, val);
            if val != pval {
                eq = false;
            }
            pval = val;
        }
        if eq {
            break;
        }
    }
    let mut sum = 0;
    for i in 0..y.len() {
        sum += g.at(y.len() - i - 1, i).unwrap();
    }
    return sum;
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
