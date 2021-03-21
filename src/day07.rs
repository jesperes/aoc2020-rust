use crate::puzzle::{Info, Puzzle};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Day07 {}

#[derive(Debug)]
pub struct Bag {
    color: String,
    contents: Vec<(i32, String)>,
}

// Parser
impl FromStr for Bag {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("^(?P<bag>.*) bags contain (?P<contents>.*)\\.$").unwrap();
            static ref CONTENT_RE: Regex =
                Regex::new("(?P<num>\\d+) (?P<color>[a-z]+ [a-z]+) (bag|bags)").unwrap();
        }

        let m = LINE_RE.captures(s).unwrap();
        Ok(Bag {
            color: m.name("bag").unwrap().as_str().to_string(),
            contents: CONTENT_RE
                .captures_iter(m.name("contents").unwrap().as_str())
                .map(|cap| {
                    let num = cap.name("num").unwrap().as_str().parse::<i32>().unwrap();
                    let color = cap.name("color").unwrap().as_str().to_string();
                    (num, color)
                })
                .collect(),
        })
    }
}

impl Puzzle for Day07 {
    type InputType = Vec<Bag>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Handy Haversacks",
            year: 2020,
            day: 7,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap().parse::<Bag>().unwrap())
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
