use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
use lalrpop_util::lalrpop_mod;
pub struct Day18 {}

lalrpop_mod!(pub day18a_grammar);
lalrpop_mod!(pub day18b_grammar);

impl Puzzle for Day18 {
    type InputType = Vec<String>;
    type T1 = i64;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Operation Order",
            year: 2020,
            day: 18,
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

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let parser = day18a_grammar::ExprParser::new();
        input.iter().map(|line| parser.parse(line).unwrap()).sum()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let parser = day18b_grammar::ExprParser::new();
        input.iter().map(|line| parser.parse(line).unwrap()).sum()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (8298263963837, 145575710203332)
    }
}

#[test]
fn grammartest() {
    assert_eq!(day18a_grammar::ExprParser::new().parse("2 * 3 + (4 * 5)").unwrap(), 26);
    assert_eq!(day18a_grammar::ExprParser::new().parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(), 13632);
}
