use colored::*;
use std::cmp::Eq;
use std::fmt::Debug;

pub struct Info {
    pub name: &'static str,
    pub year: i16,
    pub day: i8,
}

pub trait Puzzle {
    // The type of the parsed input file
    type InputType: Debug;

    // Puzzle solution types
    type T1: Debug + Eq;
    type T2: Debug + Eq;

    // Parse the input
    fn parse_input(&self) -> Self::InputType;

    // Solve part 1
    fn part1(&self, input: &Self::InputType) -> Self::T1;

    // Solve part 2
    fn part2(&self, input: &Self::InputType) -> Self::T2;

    // Return the expected results
    fn expected(&self) -> (Self::T1, Self::T2);

    // Puzzle info
    fn info(&self) -> Info;
}

pub fn run_puzzle<T: Puzzle>(p: &T) {
    let now = std::time::Instant::now();
    let info = p.info();

    let input = p.parse_input();
    let r1 = p.part1(&input);
    let r2 = p.part2(&input);
    assert_eq!((r1, r2), p.expected());

    let elapsed = now.elapsed().as_nanos();
    let elapsed_usecs = elapsed as f64 / 1_000.0;
    let elapsed_ms = elapsed as f64 / 1_000_000.0;

    let (e1, e2) = p.expected();

    let limit_ms = 1000;
    let limit_per_puzzle_ms = limit_ms as f64 / 25.0;
    let exceeded_limit = elapsed_ms > limit_per_puzzle_ms;

    let elapsed_fmt = format!("{} \u{03BC}s", elapsed_usecs);

    println!(
        "Day {:2} {:30} {:<15?} {:<15?} {:>20}",
        info.day,
        info.name.blue().bold(),
        e1,
        e2,
        if exceeded_limit {
            elapsed_fmt.red().bold()
        } else {
            elapsed_fmt.green().bold()
        }
    );
}
