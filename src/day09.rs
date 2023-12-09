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
    let y0 = y[0];
    if y.iter().skip(1).all(|&x| x == y0) {
        return y0;
    }
    let n = y.len();
    let mut yp = vec![0; n - 1];
    for i in 0..n - 1 {
        yp[i] = y[i + 1] - y[i];
    }
    let ypn = newton_extrapolation(&yp);
    return ypn + y.last().unwrap();
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
