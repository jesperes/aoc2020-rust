use crate::puzzle::{Info, Puzzle};
use multimap::MultiMap;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day07 {}

impl Puzzle for Day07 {
    type InputType = MultiMap<String, (i32, String)>;
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
        lazy_static! {
            static ref LINE_RE: Regex =
                Regex::new("^(?P<bag>.*) bags contain (?P<contents>.*)\\.$").unwrap();
            static ref CONTENT_RE: Regex =
                Regex::new("(?P<num>\\d+) (?P<color>[a-z]+ [a-z]+) (bag|bags)").unwrap();
        }

        let info = self.info();
        let mut map = MultiMap::new();
        for line in BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        {
            let s = line.unwrap();
            let m = LINE_RE.captures(&s).unwrap();
            let parent_color = m.name("bag").unwrap().as_str();
            for cap in CONTENT_RE.captures_iter(m.name("contents").unwrap().as_str()) {
                let num = cap.name("num").unwrap().as_str().parse::<i32>().unwrap();
                let child_color = cap.name("color").unwrap().as_str().to_string();
                map.insert(parent_color.to_string(), (num, child_color));
            }
        }
        map
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        let mut map = MultiMap::new();

        // Construct the reverse map of (color -> can be contained in)
        for (parent_color, bags) in input {
            for (_, child_color) in bags {
                map.insert(child_color, parent_color);
            }
        }

        find_bags("shiny gold", &map)
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        count_bags("shiny gold", input)
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (355, 5312)
    }
}

fn find_bags(color: &str, map: &MultiMap<&String, &String>) -> usize {
    let mut set = HashSet::new();
    find_bags_rec(color, map, &mut set);
    set.len()
}

fn find_bags_rec(color: &str, map: &MultiMap<&String, &String>, set: &mut HashSet<String>) {
    if let Some(colors) = map.get_vec(&color.to_string()) {
        for child_color in colors {
            set.insert(child_color.to_string());
            find_bags_rec(child_color, map, set);
        }
    }
}

fn count_bags(color: &str, map: &MultiMap<String, (i32, String)>) -> usize {
    let mut count = 0;
    if let Some(bags) = map.get_vec(color) {
        for (num, child_color) in bags {
            count += *num as usize * (1 + count_bags(child_color, map));
        }
    }
    count
}
