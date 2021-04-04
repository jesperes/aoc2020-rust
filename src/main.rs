#[macro_use]
extern crate lazy_static;
extern crate colored;
use colored::*;

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
mod day11;
mod day12;

mod puzzle;
mod template; // just to have cargo compile the template

fn main() {
    // In the spirit of solving all the solutions in Rust in less than a second...
    // https://www.reddit.com/r/adventofcode/comments/lttus6/2020_rust_solving_advent_of_code_2020_in_under_a/
    // https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/
    let limit_ms = 1000;
    let limit_ns = std::time::Duration::from_millis(limit_ms).as_nanos();
    println!(
        "Total limit: {} ms ({} \u{03BC}s/puzzle)",
        limit_ms,
        (limit_ns as f64 / (25.0 * 1000.0)) as i64
    );

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
    puzzle::run_puzzle(&day11::Day11 {});
    puzzle::run_puzzle(&day12::Day12 {});
    let elapsed_ns = now.elapsed().as_nanos();
    let exceeded_limit = elapsed_ns >= limit_ns;
    let elapsed_fmt = format!("{:.3} ms", elapsed_ns as f64 / 1_000_000.0);
    println!(
        "Total time: {}",
        if exceeded_limit {
            elapsed_fmt.red().bold()
        } else {
            elapsed_fmt.green().bold()
        }
    );
}
