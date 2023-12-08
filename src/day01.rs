use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<String> {
    return crate::utils::read_lines(filename);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut first = -1;
        let mut last = -1;
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    if first == -1 {
                        first = c as i32 - '0' as i32;
                    } else {
                        last = c as i32 - '0' as i32;
                    }
                }
                _ => {}
            }
        }
        if last != -1 {
            sum += first * 10 + last;
        } else {
            sum += first * 11;
        }
    }

    return sum;
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut first = -1;
        let mut last = -1;
        for (i, c) in line.chars().enumerate() {
            let ii = i as isize;
            match c {
                '0'..='9' => {
                    if first == -1 {
                        first = c as i32 - '0' as i32;
                    } else {
                        last = c as i32 - '0' as i32;
                    }
                }

                'a'..='z' => {
                    // look back
                    let mut digit = -1;
                    match c {
                        'o' => {
                            if ii - 2 >= 0 {
                                if &line[i - 2..i] == "tw" {
                                    digit = 2;
                                }
                            }
                            if ii - 3 >= 0 {
                                if &line[i - 3..i] == "zer" {
                                    digit = 0;
                                }
                            }
                        }

                        'e' => {
                            if ii - 2 >= 0 {
                                if &line[i - 2..i] == "on" {
                                    digit = 1;
                                }
                            }
                            if ii - 4 >= 0 {
                                if &line[i - 4..i] == "thre" {
                                    digit = 3;
                                }
                            }
                            if ii - 3 >= 0 {
                                if &line[i - 3..i] == "nin" {
                                    digit = 9;
                                } else if &line[i - 3..i] == "fiv" {
                                    digit = 5;
                                }
                            }
                        }

                        'r' => {
                            if ii - 3 >= 0 {
                                if &line[i - 3..i] == "fou" {
                                    digit = 4;
                                }
                            }
                        }

                        'x' => {
                            if ii - 2 >= 0 {
                                if &line[i - 2..i] == "si" {
                                    digit = 6;
                                }
                            }
                        }

                        'n' => {
                            if ii - 4 >= 0 {
                                if &line[i - 4..i] == "seve" {
                                    digit = 7;
                                }
                            }
                        }

                        't' => {
                            if ii - 4 >= 0 {
                                if &line[i - 4..i] == "eigh" {
                                    digit = 8;
                                }
                            }
                        }

                        _ => {}
                    }

                    if digit != -1 {
                        if first == -1 {
                            first = digit;
                        } else {
                            last = digit;
                        }
                    }
                }
                _ => {
                    println!("Unexpected character: {}", c);
                }
            }
        }
        if last != -1 {
            sum += first * 10 + last;
        } else {
            sum += first * 11;
        }
    }
    return sum;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i32); 1] = [("test_inputs/day01/test01.txt", 142)];
    pub const PART2_INPUTS: [(&str, i32); 1] = [("test_inputs/day01/test02.txt", 281)];
}

test!();
main!();
