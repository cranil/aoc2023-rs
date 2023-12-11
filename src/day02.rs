use crate::utils::{main, test};

fn get_contents(filename: &str) -> Vec<String> {
    return crate::utils::read_lines(filename);
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut toks = line.split([':']);
        let game_str = toks.next().unwrap().trim();
        let id = i32::from_str_radix(&game_str[5..], 10).unwrap();
        let mut valid = true;
        let draws = toks.next().unwrap().trim();
        for draw in draws.split(';') {
            for cube in draw.split(", ") {
                let mut toks = cube.trim().split(' ');
                let num = i32::from_str_radix(toks.next().unwrap().trim(), 10).unwrap();
                let color = toks.next().unwrap().trim();
                match color {
                    "red" => {
                        if num > 12 {
                            valid = false;
                        }
                    }
                    "green" => {
                        if num > 13 {
                            valid = false;
                        }
                    }
                    "blue" => {
                        if num > 14 {
                            valid = false;
                        }
                    }
                    _ => {}
                }
            }
        }
        if valid {
            sum += id;
        }
    }
    return sum;
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut toks = line.split([':']);
        let _game_str = toks.next().unwrap().trim();
        let draws = toks.next().unwrap().trim();
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for draw in draws.split(';') {
            for cube in draw.split(',') {
                let mut toks = cube.trim().split(' ');
                let num = i32::from_str_radix(toks.next().unwrap().trim(), 10).unwrap();
                let color = toks.next().unwrap().trim();
                match color {
                    "red" => {
                        if num > red {
                            red = num;
                        }
                    }
                    "green" => {
                        if num > green {
                            green = num;
                        }
                    }
                    "blue" => {
                        if num > blue {
                            blue = num;
                        }
                    }
                    _ => {}
                }
            }
        }
        sum += red * green * blue;
    }
    return sum;
}

test!(
    part1 {
        "test_inputs/day02/test01.txt" => 8
    },
    part2 {
        "test_inputs/day02/test01.txt" => 2286
    }
);
main!();
