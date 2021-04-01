use crate::puzzle::{Info, Puzzle};
use multimap::MultiMap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
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
    type T1 = usize;
    type T2 = usize;

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

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut map = MultiMap::<&String, &String>::new();
        for bag in input {
            for (_, contained_bag) in &bag.contents {
                map.insert(contained_bag, &bag.color);
            }
        }

        find_bags("shiny gold", &map)
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let mut map = HashMap::new();
        for bag in input {
            map.insert(&bag.color, &bag.contents);
        }

        count_bags("shiny gold", &map)
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (355, 5312)
    }
}

fn find_bags(color: &str, map: &MultiMap<&String, &String>) -> usize {
    let mut set = HashSet::new();
    find_bags_rec(color, &map, &mut set);
    set.len()
}

fn find_bags_rec(color: &str, map: &MultiMap<&String, &String>, set: &mut HashSet<String>) {
    if let Some(colors) = map.get_vec(&color.to_string()) {
        for sub in colors {
            set.insert(sub.to_string());
            find_bags_rec(sub, map, set);
        }
    }
}

fn count_bags(color: &str, map: &HashMap<&String, &Vec<(i32, String)>>) -> usize {
    let mut count = 0;
    if let Some(bags) = map.get(&color.to_string()) {
        for (num, childcol) in bags.iter() {
            count += *num as usize * (1 + count_bags(childcol, map));
        }
    }
    count
}
