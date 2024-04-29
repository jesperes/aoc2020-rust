use crate::puzzle::{Info, Puzzle};
// use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};
pub struct Day19 {}

#[derive(Debug)]
pub struct Rules {
    rules: Vec<Vec<i32>>,
    strings: Vec<String>,
}

fn enc(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(num) => num,
        Err(_err) => s.chars().nth(1).unwrap() as i32 + 1000,
    }
}

// fn to_regex(start: i32, rules: Vec<Vec<i32>>) {

// }

impl Puzzle for Day19 {
    type InputType = Rules;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Monster Messages",
            year: 2020,
            day: 19,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        let mut buf = String::new();
        let _bytes = BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .read_to_string(&mut buf)
        .unwrap();

        let s: Vec<&str> = buf.split("\n\n").collect();
        let rules = s[0];

        let mut input = Rules {
            rules: Vec::with_capacity(2048),
            strings: Vec::new(),
        };

        for rule in rules.split('\n') {
            let terms: Vec<_> = rule
                .split(|c| ":| ".contains(c))
                .filter(|c| !c.is_empty())
                .map(|t| enc(t))
                .collect();
            println!("Rule: {:?}", terms);
            let idx: usize = terms[0] as usize;
            input.rules[idx] = terms;
        }

        input.strings = s[1].split('\n').map(|s| s.to_string()).collect();
        input
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
