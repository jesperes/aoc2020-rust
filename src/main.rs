#[macro_use]
extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

mod puzzle;
mod template; // just to have cargo compile the template

fn main() {
    let now = std::time::Instant::now();
    puzzle::run_puzzle(&day01::Day01 {});
    puzzle::run_puzzle(&day02::Day02 {});
    puzzle::run_puzzle(&day03::Day03 {});
    puzzle::run_puzzle(&day04::Day04 {});
    puzzle::run_puzzle(&day05::Day05 {});
    puzzle::run_puzzle(&day06::Day06 {});
    puzzle::run_puzzle(&day07::Day07 {});
    puzzle::run_puzzle(&day08::Day08 {});
    puzzle::run_puzzle(&day09::Day09 {});
    puzzle::run_puzzle(&day10::Day10 {});
    println!(
        "Total time: {} s",
        now.elapsed().as_nanos() as f64 / 1_000_000_000.0
    );
}
