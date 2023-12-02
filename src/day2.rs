use crate::utils::main;

fn part1(input_file: &str) {
    let lines = crate::utils::read_lines(input_file);
    let mut sum = 0;
    for line in lines {
        let mut toks = line.split([':']);
        let game_str = toks.next().unwrap().trim();
        let id = i32::from_str_radix(&game_str[5..], 10).unwrap();
        let mut valid = true;
        let draws = toks.next().unwrap().trim();
        for draw in draws.split(';') {
            for cube in draw.split(',') {
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
    println!("Day 2, Part 1 result: {}", sum);
}

fn part2(input_file: &str) {
    let lines = crate::utils::read_lines(input_file);
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
    println!("Day 2, Part 1 result: {}", sum);
}

main!(2);
