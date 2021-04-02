use crate::puzzle::{Info, Puzzle};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day10 {}

impl Puzzle for Day10 {
    type InputType = Vec<i64>;
    type T1 = i64;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Adapter Array",
            year: 2020,
            day: 10,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .sorted()
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut map = HashMap::new();
        let mut jolts: i64 = 0;
        map.insert(3, 1);
        for adapter in input {
            let diff: i64 = adapter - jolts;
            *map.entry(diff).or_insert(0) += 1;
            jolts += diff;
        }
        map[&1] * map[&3]
    }

    // Idea borrowed from
    // https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/gf990qj/
    // For each adapter, add the number of ways the previous three adapters can be reached.
    // The slightly convoluted loop is because we need to include the final device joltage which
    // is 3 higher than the highest one found in the input.
    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut cache = HashMap::new();
        cache.insert(0, 1);
        for adapter in input.iter().chain([input.iter().max().unwrap() + 3].iter()) {
            cache.insert(
                *adapter,
                (adapter - 3..*adapter)
                    .map(|a| cache.get(&a).unwrap_or(&0))
                    .sum(),
            );
        }

        *cache.values().max().unwrap()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (2738, 74049191673856)
    }
}
