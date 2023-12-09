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

    #[arg(short, long, default_value_t = 0)]
    part: u32,

    #[arg(short, long, default_value_t = 1)]
    num_runs: usize,

    #[arg(short, long)]
    input_file: Option<String>,
}

macro_rules! input_file {
    ($day:expr, $file:expr) => {
        match $file {
            Some(f) => f.clone(),
            None => format!("input/day{:02}/input.txt", $day),
        }
        .as_str()
    };
    ($day:expr) => {
        format!("input/day{:02}/input.txt", $day).as_str()
    };
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
        01 => day01::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        02 => day02::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        03 => day03::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        04 => day04::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        05 => day05::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        06 => day06::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        07 => day07::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        08 => day08::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        09 => day09::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        10 => day10::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        11 => day11::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        12 => day12::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        13 => day13::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        14 => day14::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        15 => day15::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        16 => day16::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        17 => day17::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        18 => day18::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        19 => day19::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        20 => day20::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        21 => day21::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        22 => day22::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        23 => day23::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        24 => day24::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        25 => day25::main(
            &args.part,
            &args.num_runs,
            input_file!(args.day, args.input_file),
        ),
        _ => {
            day01::main(&args.part, &args.num_runs, input_file!(args.day));
            day02::main(&args.part, &args.num_runs, input_file!(args.day));
            day03::main(&args.part, &args.num_runs, input_file!(args.day));
            day04::main(&args.part, &args.num_runs, input_file!(args.day));
            day05::main(&args.part, &args.num_runs, input_file!(args.day));
            day06::main(&args.part, &args.num_runs, input_file!(args.day));
            day07::main(&args.part, &args.num_runs, input_file!(args.day));
            day08::main(&args.part, &args.num_runs, input_file!(args.day));
            day09::main(&args.part, &args.num_runs, input_file!(args.day));
            day10::main(&args.part, &args.num_runs, input_file!(args.day));
            day11::main(&args.part, &args.num_runs, input_file!(args.day));
            day12::main(&args.part, &args.num_runs, input_file!(args.day));
            day13::main(&args.part, &args.num_runs, input_file!(args.day));
            day14::main(&args.part, &args.num_runs, input_file!(args.day));
            day15::main(&args.part, &args.num_runs, input_file!(args.day));
            day16::main(&args.part, &args.num_runs, input_file!(args.day));
            day17::main(&args.part, &args.num_runs, input_file!(args.day));
            day18::main(&args.part, &args.num_runs, input_file!(args.day));
            day19::main(&args.part, &args.num_runs, input_file!(args.day));
            day20::main(&args.part, &args.num_runs, input_file!(args.day));
            day21::main(&args.part, &args.num_runs, input_file!(args.day));
            day22::main(&args.part, &args.num_runs, input_file!(args.day));
            day23::main(&args.part, &args.num_runs, input_file!(args.day));
            day24::main(&args.part, &args.num_runs, input_file!(args.day));
            day25::main(&args.part, &args.num_runs, input_file!(args.day));
        }
    }
}
