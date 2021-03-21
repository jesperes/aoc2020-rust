use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day01 {}

impl Puzzle for Day01 {
    type InputType = Vec<i32>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Report Repair",
            year: 2020,
            day: 1,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        BufReader::new(File::open("inputs/2020/input01.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<i32>().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        for &x in input.iter() {
            for &y in input.iter() {
                if x + y == 2020 {
                    return x * y;
                }
            }
        }
        panic!()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let smallest = input.iter().min().unwrap();
        for &x in input.iter() {
            for &y in input.iter() {
                if x == y || x + y + smallest >= 2020 {
                    continue;
                }
                for &z in input.iter() {
                    if x + y + z == 2020 {
                        return x * y * z;
                    }
                }
            }
        }
        panic!();
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (987339, 259521570)
    }
}
