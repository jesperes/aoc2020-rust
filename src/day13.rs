use crate::puzzle::{Info, Puzzle};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day13 {}

#[derive(Debug)]
pub struct InputData {
    timestamp: i32,
    departures: Vec<String>,
}

impl Puzzle for Day13 {
    type InputType = InputData;
    type T1 = i32;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Shuttle Search",
            year: 2020,
            day: 13,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        let lines: Vec<String> = BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap())
        .collect();

        InputData {
            timestamp: lines[0].parse::<i32>().unwrap(),
            departures: lines[1].split(',').map(|s| s.to_string()).collect(),
        }
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let (t, id): (i32, i32) = input
            .departures
            .iter()
            .filter(|s| !s.starts_with('x')) // pick just the departure times
            .map(|n| n.parse::<i32>().unwrap()) // convert to i64
            .map(|id| {
                (
                    id - (input.timestamp % id), // time to this departure
                    id, // keep 'id', because we need it for computing the answer
                )
            })
            .sorted() // sort them in departure order
            .next() // take the next occurring one
            .unwrap();

        t * id
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut residues: Vec<i64> = Vec::new();
        let mut modulii: Vec<i64> = Vec::new();

        for (a, n) in input.departures.iter().enumerate() {
            if n.starts_with('x') {
                continue;
            }

            let modulo = n.parse::<i64>().unwrap();
            modulii.push(modulo);
            let residue = modulo - a as i64;
            residues.push(residue);
        }
        chinese_remainder(&residues, &modulii).unwrap()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (333, 690123192779524)
    }
}

// Chinese Remainder Theorem implementation courtesy of rosettacode.org:
// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust

#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % prod)
}
