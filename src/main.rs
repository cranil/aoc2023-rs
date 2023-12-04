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
        1 => day1::main(&args.test, &args.part),
        2 => day2::main(&args.test, &args.part),
        3 => day3::main(&args.test, &args.part),
        4 => day4::main(&args.test, &args.part),
        5 => day5::main(&args.test, &args.part),
        6 => day6::main(&args.test, &args.part),
        7 => day7::main(&args.test, &args.part),
        8 => day8::main(&args.test, &args.part),
        9 => day9::main(&args.test, &args.part),
        10 => day10::main(&args.test, &args.part),
        11 => day11::main(&args.test, &args.part),
        12 => day12::main(&args.test, &args.part),
        13 => day13::main(&args.test, &args.part),
        14 => day14::main(&args.test, &args.part),
        15 => day15::main(&args.test, &args.part),
        16 => day16::main(&args.test, &args.part),
        17 => day17::main(&args.test, &args.part),
        18 => day18::main(&args.test, &args.part),
        19 => day19::main(&args.test, &args.part),
        20 => day20::main(&args.test, &args.part),
        21 => day21::main(&args.test, &args.part),
        22 => day22::main(&args.test, &args.part),
        23 => day23::main(&args.test, &args.part),
        24 => day24::main(&args.test, &args.part),
        25 => day25::main(&args.test, &args.part),
        _ => {
            day1::main(&args.test, &args.part);
            day2::main(&args.test, &args.part);
            day3::main(&args.test, &args.part);
            day4::main(&args.test, &args.part);
            day5::main(&args.test, &args.part);
            day6::main(&args.test, &args.part);
            day7::main(&args.test, &args.part);
            day8::main(&args.test, &args.part);
            day9::main(&args.test, &args.part);
            day10::main(&args.test, &args.part);
            day11::main(&args.test, &args.part);
            day12::main(&args.test, &args.part);
            day13::main(&args.test, &args.part);
            day14::main(&args.test, &args.part);
            day15::main(&args.test, &args.part);
            day16::main(&args.test, &args.part);
            day17::main(&args.test, &args.part);
            day18::main(&args.test, &args.part);
            day19::main(&args.test, &args.part);
            day20::main(&args.test, &args.part);
            day21::main(&args.test, &args.part);
            day22::main(&args.test, &args.part);
            day23::main(&args.test, &args.part);
            day24::main(&args.test, &args.part);
            day25::main(&args.test, &args.part);
        }
    }
}
