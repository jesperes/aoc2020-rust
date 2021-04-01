use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day09 {}

impl Puzzle for Day09 {
    type InputType = Vec<i64>;
    type T1 = i64;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Encoding Error",
            year: 2020,
            day: 9,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        for i in 25..input.len() {
            if !is_valid(input, i) {
                return input[i];
            }
        }
        panic!();
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let (p1, _) = self.expected();

        for len in 2..input.len() {
            for i in 0..input.len() - len {
                let slice = &input[i..i + len];
                let sum: i64 = slice.iter().sum();
                if sum == p1 {
                    let min = slice.iter().min().unwrap();
                    let max = slice.iter().max().unwrap();
                    return min + max;
                }
            }
        }
        0
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (138879426, 23761694)
    }
}

fn is_valid(input: &[i64], i: usize) -> bool {
    let prev25 = &input[i - 25..i];
    for j in prev25 {
        for k in prev25 {
            if j != k && j + k == input[i] {
                 return true;
            }
        }
    }
    false
}
