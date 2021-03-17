use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day03 {}

pub struct Point {
    x: usize,
    y: usize,
}

pub struct Slope {
    dx: usize,
    dy: usize,
}

impl Puzzle for Day03 {
    type InputType = Vec<Vec<char>>;
    type T1 = i32;
    type T2 = i64;

    fn info(&self) -> Info {
        return Info {
            name: "Toboggan Trajectory",
            year: 2020,
            day: 3,
        };
    }
    fn parse_input(&self) -> Self::InputType {
        BufReader::new(File::open("inputs/2020/input03.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut p = Point { x: 0, y: 0 };
        let dx = 3;
        let mut count = 0;
        while p.y < input.len() {
            let line = &input[p.y];
            if line[p.x % line.len()] == '#' {
                count += 1;
            }

            p.x += dx;
            p.y += 1;
        }
        return count;
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let slopes = vec![
            Slope { dx: 1, dy: 1 },
            Slope { dx: 3, dy: 1 },
            Slope { dx: 5, dy: 1 },
            Slope { dx: 7, dy: 1 },
            Slope { dx: 1, dy: 2 },
        ];
        let mut prod: i64 = 1;
        for slope in slopes {
            let mut p = Point { x: 0, y: 0 };
            let mut count = 0;
            while p.y < input.len() {
                let line = &input[p.y];
                if line[p.x % line.len()] == '#' {
                    count += 1;
                }
                p.x += slope.dx;
                p.y += slope.dy;
            }
            prod *= count;
        }
        return prod;
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (230, 9533698720)
    }
}
