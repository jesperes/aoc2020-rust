use crate::puzzle::{Info, Puzzle};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day05 {}

impl Day05 {
    fn seat_id(s: &str) -> i32 {
        let mut id = 0;
        for c in s.chars() {
            match c {
                'B' => id = (id << 1) | 1,
                'R' => id = (id << 1) | 1,
                _ => id = id << 1,
            }
        }
        id
    }
}

impl Puzzle for Day05 {
    type InputType = Vec<i32>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        return Info {
            name: "Binary Boarding",
            year: 2020,
            day: 5,
        };
    }
    fn parse_input(&self) -> Self::InputType {
        BufReader::new(File::open("inputs/2020/input05.txt").unwrap())
            .lines()
            .map(|s| Day05::seat_id(&s.unwrap()))
            .sorted()
            .rev()
            .collect::<Vec<i32>>()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        input[0]
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        for i in 0..input.len() - 2 {
            if input[i] == input[i + 1] + 2 {
                return input[i + 1] + 1;
            }
        }
        panic!();
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (928, 610)
    }
}
