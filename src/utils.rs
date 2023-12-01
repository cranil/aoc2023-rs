use std::fs::read_to_string;

pub fn read_all(filename: &str) -> String {
    return read_to_string(filename).expect(format!("Failed to read: {}", filename).as_str());
}

pub fn read_lines(filename: &str) -> Vec<String> {
    return read_all(filename)
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
}

macro_rules! main {
    ($day:expr) => {
        pub fn main(test: &bool, part: &u32) {
            let input_file = if *test {
                format!("test_input/day{}.txt", $day)
            } else {
                format!("input/day{}.txt", $day)
            };

            match part {
                1 => part1(input_file.as_str()),
                2 => part2(input_file.as_str()),
                _ => {
                    part1(input_file.as_str());
                    part2(input_file.as_str());
                }
            }
        }
    };
}

pub(crate) use main;
