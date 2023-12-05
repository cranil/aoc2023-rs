pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub mod algos;
pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u32,

    #[arg(short, long, default_value_t = false)]
    test: bool,

    #[arg(short, long, default_value_t = 0)]
    part: u32,

    #[arg(short, long, default_value_t = 1)]
    num_runs: usize,
}

fn main() {
    let args = Args::parse();
    let header = format!(
        "
+----+--------+----------------+------------+
|{:^4}|{:^8}|{:^16}|{:^12}|
+----+--------+----------------+------------+",
        "Day", "Part", "Result", "Time (us)"
    );
    println!("{}", header);
    match args.day {
        1 => day1::main(&args.test, &args.part, &args.num_runs),
        2 => day2::main(&args.test, &args.part, &args.num_runs),
        3 => day3::main(&args.test, &args.part, &args.num_runs),
        4 => day4::main(&args.test, &args.part, &args.num_runs),
        5 => day5::main(&args.test, &args.part, &args.num_runs),
        6 => day6::main(&args.test, &args.part, &args.num_runs),
        7 => day7::main(&args.test, &args.part, &args.num_runs),
        8 => day8::main(&args.test, &args.part, &args.num_runs),
        9 => day9::main(&args.test, &args.part, &args.num_runs),
        10 => day10::main(&args.test, &args.part, &args.num_runs),
        11 => day11::main(&args.test, &args.part, &args.num_runs),
        12 => day12::main(&args.test, &args.part, &args.num_runs),
        13 => day13::main(&args.test, &args.part, &args.num_runs),
        14 => day14::main(&args.test, &args.part, &args.num_runs),
        15 => day15::main(&args.test, &args.part, &args.num_runs),
        16 => day16::main(&args.test, &args.part, &args.num_runs),
        17 => day17::main(&args.test, &args.part, &args.num_runs),
        18 => day18::main(&args.test, &args.part, &args.num_runs),
        19 => day19::main(&args.test, &args.part, &args.num_runs),
        20 => day20::main(&args.test, &args.part, &args.num_runs),
        21 => day21::main(&args.test, &args.part, &args.num_runs),
        22 => day22::main(&args.test, &args.part, &args.num_runs),
        23 => day23::main(&args.test, &args.part, &args.num_runs),
        24 => day24::main(&args.test, &args.part, &args.num_runs),
        25 => day25::main(&args.test, &args.part, &args.num_runs),
        _ => {
            day1::main(&args.test, &args.part, &args.num_runs);
            day2::main(&args.test, &args.part, &args.num_runs);
            day3::main(&args.test, &args.part, &args.num_runs);
            day4::main(&args.test, &args.part, &args.num_runs);
            day5::main(&args.test, &args.part, &args.num_runs);
            day6::main(&args.test, &args.part, &args.num_runs);
            day7::main(&args.test, &args.part, &args.num_runs);
            day8::main(&args.test, &args.part, &args.num_runs);
            day9::main(&args.test, &args.part, &args.num_runs);
            day10::main(&args.test, &args.part, &args.num_runs);
            day11::main(&args.test, &args.part, &args.num_runs);
            day12::main(&args.test, &args.part, &args.num_runs);
            day13::main(&args.test, &args.part, &args.num_runs);
            day14::main(&args.test, &args.part, &args.num_runs);
            day15::main(&args.test, &args.part, &args.num_runs);
            day16::main(&args.test, &args.part, &args.num_runs);
            day17::main(&args.test, &args.part, &args.num_runs);
            day18::main(&args.test, &args.part, &args.num_runs);
            day19::main(&args.test, &args.part, &args.num_runs);
            day20::main(&args.test, &args.part, &args.num_runs);
            day21::main(&args.test, &args.part, &args.num_runs);
            day22::main(&args.test, &args.part, &args.num_runs);
            day23::main(&args.test, &args.part, &args.num_runs);
            day24::main(&args.test, &args.part, &args.num_runs);
            day25::main(&args.test, &args.part, &args.num_runs);
        }
    }
}
