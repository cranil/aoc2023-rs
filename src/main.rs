pub mod day1;
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
    match args.day {
        1 => day1::main(&args.test, &args.part),
        _ => {
            day1::main(&args.test, &args.part);
        }
    }
}
