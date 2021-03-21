use crate::puzzle::{Info, Puzzle};
use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::EnumString;

pub struct Day04 {}

#[derive(EnumString)]
#[allow(non_camel_case_types)]
pub enum Color {
    amb,
    blu,
    brn,
    gry,
    grn,
    hzl,
    oth,
}

#[derive(EnumString, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Field {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    invalid,
}

#[derive(Debug)]
pub struct FieldError;

#[derive(Debug)]
pub struct Passport {
    fields: Vec<(Field, String)>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.fields
            .iter()
            .all(|(f, v)| Passport::is_valid_field(f, v))
    }

    fn is_valid_year(year: &str, lower: i16, upper: i16) -> bool {
        let y = year.parse::<i16>().unwrap_or(0);
        y >= lower && y <= upper
    }

    fn is_valid_hcl(hcl: &str) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
        }

        HCL_RE.is_match(hcl)
    }

    fn is_valid_pid(pid: &str) -> bool {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new("^\\d{9}$").unwrap();
        }

        PID_RE.is_match(pid)
    }

    fn is_valid_hgt(hgt: &str) -> bool {
        let h = hgt[..hgt.len() - 2].parse::<i16>().unwrap_or(0);
        if hgt.ends_with("in") {
            (59..=76).contains(&h)
        } else if hgt.ends_with("cm") {
            (150..=193).contains(&h)
        } else {
            false
        }
    }

    fn is_valid_ecl(ecl: &str) -> bool {
        Color::from_str(ecl).is_ok()
    }
    fn is_valid_field(f: &Field, v: &str) -> bool {
        match f {
            Field::byr => Passport::is_valid_year(v, 1920, 2002),
            Field::iyr => Passport::is_valid_year(v, 2010, 2020),
            Field::eyr => Passport::is_valid_year(v, 2020, 2030),
            Field::hgt => Passport::is_valid_hgt(v),
            Field::hcl => Passport::is_valid_hcl(v),
            Field::ecl => Passport::is_valid_ecl(v),
            Field::pid => Passport::is_valid_pid(v),
            Field::invalid => panic!(),
        }
    }
}

impl FromStr for Passport {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Passport, Self::Err> {
        Ok(Passport {
            fields: s
                .trim()
                .replace("\n", " ")
                .split(' ')
                .map(|s| {
                    (
                        Field::from_str(&s[..3]).unwrap_or(Field::invalid),
                        s[4..].to_string(),
                    )
                })
                .filter(|(k, _)| *k != Field::invalid)
                .collect(),
        })
    }
}

impl Puzzle for Day04 {
    type InputType = Vec<Passport>;
    type T1 = usize;
    type T2 = usize;

    fn info(&self) -> Info {
        Info {
            name: "Passport Processing",
            year: 2020,
            day: 4,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        std::fs::read_to_string("inputs/2020/input04.txt")
            .unwrap()
            .split("\n\n")
            .map(|s| s.parse::<Passport>().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        /*
         * For part 1, we are looking for the number of passports where all required
         * fields are present. The "cid" field is ignored, so just check that there are at
         * least 7 fields.
         */

        input.iter().filter(|p| p.fields.len() == 7).count()
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        /*
         * For part 2, we need to validate the fields too.
         */
        input
            .iter()
            .filter(|p| p.fields.len() == 7 && p.is_valid())
            .count()
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (200, 116)
    }
}
