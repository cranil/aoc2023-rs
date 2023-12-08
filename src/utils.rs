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
    () => {
        pub fn main(part: &u32, num_runs: &usize) {
            let day = std::module_path!()
                .split("day")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let input_file = format!("input/day{}.txt", day);
            let contents = get_contents(input_file.as_str());
            let time_p1 = || {
                let (result, time) = crate::utils::time_it(|| part1(&contents), num_runs);
                let output = format!(
                    "|{:^4}|{:^8}|{:^16}|{:^12}|
+----+--------+----------------+------------+",
                    day, 1, result, time
                );
                println!("{}", output);
            };
            let time_p2 = || {
                let (result, time) = crate::utils::time_it(|| part2(&contents), num_runs);
                let output = format!(
                    "|{:^4}|{:^8}|{:^16}|{:^12}|
+----+--------+----------------+------------+",
                    day, 2, result, time
                );
                println!("{}", output);
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

macro_rules! test {
    () => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn part1() {
                for input in consts::PART1_INPUTS.iter() {
                    let inputs = get_contents(input.0);
                    assert_eq!((input.0, super::part1(&inputs)), *input);
                }
            }

            #[test]
            fn part2() {
                for input in consts::PART2_INPUTS.iter() {
                    let inputs = get_contents(input.0);
                    assert_eq!((input.0, super::part2(&inputs)), *input);
                }
            }
        }
    };
}

pub fn time_it<T: Default, F: Fn() -> T>(f: F, num_runs: &usize) -> (T, f64) {
    let start = std::time::Instant::now();
    let mut result = T::default();
    for _ in 0..*num_runs {
        result = f();
    }
    let end = std::time::Instant::now();
    return (
        result,
        end.duration_since(start).as_micros() as f64 / *num_runs as f64,
    );
}

pub(crate) use main;
pub(crate) use test;
