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
        pub fn main(test: &bool, part: &u32, num_runs: &usize) {
            let input_file = if *test {
                format!("test_input/day{}.txt", $day)
            } else {
                format!("input/day{}.txt", $day)
            };
            let contents = get_contents(input_file.as_str());
            let time_p1 = || {
                let (result, time) = crate::utils::time_it(|| part1(&contents), num_runs);
                let output = format!(
                    "|{:^4}|{:^8}|{:^16}|{:^12}|
+----+--------+----------------+------------+",
                    $day, 1, result, time
                );
                println!("{}", output);
            };
            let time_p2 = || {
                let (result, time) = crate::utils::time_it(|| part2(&contents), num_runs);
                let output = format!(
                    "|{:^4}|{:^8}|{:^16}|{:^12}|
+----+--------+----------------+------------+",
                    $day, 2, result, time
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
