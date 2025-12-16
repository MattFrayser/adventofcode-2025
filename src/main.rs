use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("Usage: cargo run -- <day>");

    match day.as_str() {
        "1" => day01::solve(),
        "2" => day02::solve(),
        "3" => day03::solve(),
        _ => eprintln!("Day {} not implemented", day),
    }
}
