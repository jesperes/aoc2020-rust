use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct DayXX {}

impl Puzzle for DayXX {
    type InputType = Vec<String>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "??",
            year: 0,
            day: 0,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap())
        .collect()
    }

    fn part1(&self, _input: &Self::InputType) -> Self::T1 {
        0
    }

    fn part2(&self, _input: &Self::InputType) -> Self::T2 {
        0
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (0, 0)
    }
}
