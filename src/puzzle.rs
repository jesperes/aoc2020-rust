use std::cmp::Eq;
use std::fmt::Debug;
use std::time::Duration;

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

trait TimeUtils {
    fn as_usecs_f64(&self) -> f64;
}

impl TimeUtils for Duration {
    fn as_usecs_f64(&self) -> f64 {
        self.as_nanos() as f64 / 1000.0
    }
}

pub fn run_puzzle<T: Puzzle>(p: &T) {
    let now = std::time::Instant::now();
    let info = p.info();
    let input = p.parse_input();
    let r1 = p.part1(&input);
    let r2 = p.part2(&input);
    assert_eq!((r1, r2), p.expected());
    println!(
        "{} day {}: {} ({} \u{03BC}s)",
        info.year,
        info.day,
        info.name,
        now.elapsed().as_usecs_f64()
    );
}
