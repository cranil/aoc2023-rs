pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
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
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

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
        01 => day01::main(&args.test, &args.part, &args.num_runs),
        02 => day02::main(&args.test, &args.part, &args.num_runs),
        03 => day03::main(&args.test, &args.part, &args.num_runs),
        04 => day04::main(&args.test, &args.part, &args.num_runs),
        05 => day05::main(&args.test, &args.part, &args.num_runs),
        06 => day06::main(&args.test, &args.part, &args.num_runs),
        07 => day07::main(&args.test, &args.part, &args.num_runs),
        08 => day08::main(&args.test, &args.part, &args.num_runs),
        09 => day09::main(&args.test, &args.part, &args.num_runs),
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
            day01::main(&args.test, &args.part, &args.num_runs);
            day02::main(&args.test, &args.part, &args.num_runs);
            day03::main(&args.test, &args.part, &args.num_runs);
            day04::main(&args.test, &args.part, &args.num_runs);
            day05::main(&args.test, &args.part, &args.num_runs);
            day06::main(&args.test, &args.part, &args.num_runs);
            day07::main(&args.test, &args.part, &args.num_runs);
            day08::main(&args.test, &args.part, &args.num_runs);
            day09::main(&args.test, &args.part, &args.num_runs);
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
