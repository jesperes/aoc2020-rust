use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Day02 {}

#[derive(Debug)]
pub struct Password {
    a: i32,
    b: i32,
    c: char,
    pwd: Vec<char>,
}

impl FromStr for Password {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<&str> = s.split(&[' ', '-', ':'][..]).collect();
        Ok(Password {
            a: elems[0].parse::<i32>()?,
            b: elems[1].parse::<i32>()?,
            c: elems[2].chars().next().unwrap(),
            pwd: elems[4].chars().collect(),
        })
    }
}

impl Puzzle for Day02 {
    type InputType = Vec<Password>;
    type T1 = usize;
    type T2 = usize;

    fn info(&self) -> Info {
        return Info {
            name: "Password Philosophy",
            year: 2020,
            day: 2,
        };
    }
    fn parse_input(&self) -> Self::InputType {
        BufReader::new(File::open("inputs/2020/input02.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<Password>().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        input.iter().filter(|p| is_valid1(p)).count()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        input.iter().filter(|p| is_valid2(p)).count()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (660, 530)
    }
}

fn is_valid1(p: &Password) -> bool {
    let count = p.pwd.iter().filter(|c| **c == p.c).count() as i32;
    return count >= p.a && count <= p.b;
}

fn is_valid2(p: &Password) -> bool {
    let c1 = p.pwd[(p.a - 1) as usize];
    let c2 = p.pwd[(p.b - 1) as usize];
    let mut either: i32 = 0;
    if c1 == p.c {
        either += 1;
    }
    if c2 == p.c {
        either += 1;
    }

    return either == 1;
}
