use crate::puzzle::{Info, Puzzle};

pub struct Day06 {}

fn str_to_bits(s: &str) -> u32 {
    let mut bits: u32 = 0;
    for c in s.chars() {
        let n = (c as i32) - ('a' as i32);
        assert!((0..=26).contains(&n));
        bits |= 1 << n;
    }
    bits
}

impl Puzzle for Day06 {
    type InputType = Vec<Vec<u32>>;
    type T1 = u32;
    type T2 = u32;

    fn info(&self) -> Info {
        Info {
            name: "Custom Customs",
            year: 2020,
            day: 6,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        std::fs::read_to_string("inputs/2020/input06.txt")
            .unwrap()
            .trim()
            .split("\n\n")
            .map(|s| s.split('\n').map(|s0| str_to_bits(s0)).collect())
            .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        input.iter().fold(0, |sum, list| {
            sum + list.iter().fold(0, |bits, n| bits | n).count_ones()
        })
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        input.iter().fold(0, |sum, list| {
            sum + list
                .iter()
                .skip(1)
                .fold(list[0], |bits, n| bits & n)
                .count_ones()
        })
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (6680, 3117)
    }
}
