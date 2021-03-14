use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn expected() -> (i32, i32) {
    (987339, 259521570)
}

pub fn parse_input() -> Result<Vec<i32>, std::io::Error> {
    Ok(BufReader::new(File::open("inputs/2020/input01.txt")?)
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect())
}

pub fn part1(input: &Vec<i32>) -> Result<i32, &'static str> {
    for &x in input.iter() {
        for &y in input.iter() {
            if x + y == 2020 {
                return Ok(x * y);
            }
        }
    }
    return Err("no result found");
}

pub fn part2(input: &Vec<i32>) -> Result<i32, &'static str> {
    let smallest = input.iter().min().unwrap();
    for &x in input.iter() {
        for &y in input.iter() {
            if x == y || x + y + smallest >= 2020 {
                continue;
            }
            for &z in input.iter() {
                if x + y + z == 2020 {
                    return Ok(x * y * z);
                }
            }
        }
    }
    return Err("no result found");
}
