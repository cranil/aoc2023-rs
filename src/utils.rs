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
            let contents = get_contents(input_file.as_str());
            let time_p1 = || {
                let (result, time) = crate::utils::time_it(|| part1(&contents));
                println!("Day {}, Part 1 result: {}, time: {}us", $day, result, time);
            };
            let time_p2 = || {
                let (result, time) = crate::utils::time_it(|| part2(&contents));
                println!("Day {}, Part 2 result: {}, time: {}us", $day, result, time);
            };
            match part {
                1 => time_p1(),
                2 => time_p2(),
                _ => {
                    time_p1();
                    time_p2();
                }
            }
        }
    };
}

pub fn time_it<T, F: Fn() -> T>(f: F) -> (T, u128) {
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = f();
    }
    let end = std::time::Instant::now();
    let result = f();
    return (result, end.duration_since(start).as_micros() / 1000);
}

pub(crate) use main;
