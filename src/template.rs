use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DayXX {}

impl Puzzle for DayXX {
    type InputType = i32; // TODO
    type T1 = i32; // TODO
    type T2 = i32; // TODO

    fn info(&self) -> Info {
        return Info {
            name: "??",
            year: 0,
            day: 0,
        };
    }
    fn parse_input(&self) -> Self::InputType {
        panic!()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        panic!()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        panic!()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        panic!()
    }
}
