use crate::puzzle::{Info, Puzzle};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Day16 {}

#[derive(Debug, Default)]
pub struct InputData {
    fields: HashMap<String, Vec<i32>>,
    my_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

fn to_int_list(list_of_ints: String) -> Vec<i32> {
    list_of_ints
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn is_valid_range(value: &i32, ranges: &[i32]) -> bool {
    let a = ranges[0];
    let b = ranges[1];
    let c = ranges[2];
    let d = ranges[3];
    (a..=b).contains(value) || (c..=d).contains(value)
}

// Is value a valid field value for some field
fn is_valid(value: &i32, input_data: &InputData) -> bool {
    input_data
        .fields
        .values()
        .any(|ranges: &Vec<i32>| is_valid_range(value, ranges))
}

impl Puzzle for Day16 {
    type InputType = InputData;
    type T1 = i32;
    type T2 = i64;

    fn info(&self) -> Info {
        Info {
            name: "Ticket Translation",
            year: 2020,
            day: 16,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let mut section = 0; // current section
        let mut input_data = Self::InputType::default();
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| {
            match (line.as_str(), section) {
                ("", _) => {} // ignore empty lines
                ("your ticket:", _) => {
                    section += 1;
                }
                ("nearby tickets:", _) => {
                    section += 1;
                }
                (_, 0) => {
                    // Avoid using regexps to parse when string splitting works
                    // just as well.
                    let s1: Vec<&str> = line.split(':').collect();
                    input_data.fields.insert(
                        s1[0].to_string(),
                        s1[1]
                            .split(|c| c == ' ' || c == '-' || c == 'o' || c == 'r')
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect(),
                    );
                }
                (ticket_nums, 1) => {
                    // my ticket
                    input_data.my_ticket = to_int_list(ticket_nums.to_string());
                }
                (ticket_nums, 2) => {
                    // nearby tickets
                    input_data
                        .nearby_tickets
                        .push(to_int_list(ticket_nums.to_string()));
                }
                _ => panic!("no match"),
            }
        });

        input_data
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        input
            .nearby_tickets
            .iter()
            .map(|ticket| {
                ticket
                    .iter()
                    .filter(|field| !is_valid(field, input))
                    .sum::<i32>()
            })
            .sum::<i32>()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        let valid_tickets: Vec<&Vec<i32>> = input
            .nearby_tickets
            .iter()
            .filter(|t| t.iter().all(|f| is_valid(f, input)))
            .collect();

        let num_pos = input.my_ticket.len();

        // Map from ticket positions -> field names
        let mut pos_map: HashMap<usize, String> = HashMap::new();

        // Loop as long as we have not found all keys
        while pos_map.len() < num_pos {
            for pos in 0..num_pos {
                if pos_map.contains_key(&pos) {
                    continue; // position already found
                }

                // Count the number of fields this position can correspond to. If
                // there is only one field, we can store that mapping and go on to
                // the next.
                let mut matching_fields = Vec::new();
                for (field, ranges) in input.fields.iter() {
                    if pos_map.values().any(|v| v == field) {
                        // TODO optimize this check
                        // ignore fields we have already assigned
                        continue;
                    }

                    if valid_tickets
                        .iter()
                        .all(|ticket| is_valid_range(&ticket[pos as usize], ranges))
                    {
                        matching_fields.push(field);
                    }
                }

                if matching_fields.len() == 1 {
                    pos_map.insert(pos, matching_fields[0].to_string());
                    break;
                }
            }
        }

        pos_map
            .iter()
            .filter(|(_, v)| v.starts_with("departure"))
            .map(|(k, _)| input.my_ticket[*k as usize] as i64)
            .product()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (27850, 491924517533)
    }
}
