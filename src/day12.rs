use std::collections::HashMap;

use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<(String, Vec<i8>)> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .iter()
        .map(|s| {
            let mut tokens = s.split_whitespace();
            let spring_config = tokens.next().unwrap();
            let grouping = tokens
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i8>().unwrap())
                .collect::<Vec<i8>>();
            (spring_config.to_string(), grouping)
        })
        .collect::<Vec<_>>();
}

fn solve(
    config: &str,
    ii: usize,
    grouping: &[i8],
    flag: bool,
    dp: &mut HashMap<(usize, Vec<i8>, bool), i64>,
) -> i64 {
    if grouping.len() == 0 {
        if config[ii..].find('#').is_some() {
            return 0;
        } else {
            return 1;
        }
    }

    let mut j = 0;
    let mut grouping = grouping.to_vec();
    let mut p = flag;
    let mut i = ii;
    for c in config.chars().skip(ii) {
        if c == '#' {
            if j >= grouping.len() {
                return 0;
            }
            grouping[j] -= 1;
            if grouping[j] < 0 {
                return 0;
            }
            if !p {
                p = true;
            }
        } else if c == '.' {
            if p {
                if grouping[j] > 0 {
                    return 0;
                }
                j += 1;
                p = false;
            }
        } else if c == '?' {
            let total0 = if p {
                if grouping[j] > 0 {
                    0
                } else {
                    if let Some(n_solns) = dp.get(&(i + 1, grouping[j + 1..].to_vec(), false)) {
                        *n_solns
                    } else {
                        let n_solns = solve(&config, i + 1, &grouping[j + 1..], false, dp);
                        dp.insert((i + 1, grouping[j + 1..].to_vec(), false), n_solns);
                        n_solns
                    }
                }
            } else {
                if let Some(n_solns) = dp.get(&(i + 1, grouping[j..].to_vec(), false)) {
                    *n_solns
                } else {
                    let n_solns = solve(&config, i + 1, &grouping[j..], false, dp);
                    dp.insert((i + 1, grouping[j..].to_vec(), false), n_solns);
                    n_solns
                }
            };
            if j >= grouping.len() {
                return total0;
            }
            grouping[j] -= 1;
            let total1 = if grouping[j] < 0 {
                0
            } else {
                if let Some(n_solns) = dp.get(&(i + 1, grouping[j..].to_vec(), true)) {
                    *n_solns
                } else {
                    let n_solns = solve(&config, i + 1, &grouping[j..], true, dp);
                    dp.insert((i + 1, grouping[j..].to_vec(), true), n_solns);
                    n_solns
                }
            };
            return total0 + total1;
        }
        i += 1;
    }
    if grouping.iter().all(|&x| x == 0) {
        return 1;
    }
    return 0;
}

fn part1(problem: &Vec<(String, Vec<i8>)>) -> i64 {
    let mut total = 0;
    for (spring_config, grouping) in problem {
        let solution = solve(spring_config, 0, grouping, false, &mut HashMap::new());
        total += solution;
    }
    return total;
}

fn part2(problem: &Vec<(String, Vec<i8>)>) -> i64 {
    let mut total = 0;
    for (spring_config, grouping) in problem {
        let spring_config = vec![spring_config.clone(); 5].join("?");
        let grouping = grouping.repeat(5);
        let mut dp = HashMap::new();
        let solution = solve(&spring_config, 0, &grouping, false, &mut dp);
        total += solution;
    }

    return total;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
