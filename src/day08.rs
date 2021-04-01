use crate::puzzle::{Info, Puzzle};
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum_macros::EnumString;
pub struct Day08 {}

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString)]
enum Op {
    acc,
    jmp,
    nop,
}

#[derive(Debug)]
pub struct Instr {
    op: Op,
    val: i32,
}

enum ProgResult {
    Terminated(i32),
    Looped(i32),
}

impl Puzzle for Day08 {
    type InputType = Vec<Instr>;
    type T1 = i32;
    type T2 = i32;

    fn info(&self) -> Info {
        Info {
            name: "Handheld Halting",
            year: 2020,
            day: 8,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        let info = self.info();
        BufReader::new(
            File::open(format!("inputs/{}/input{:02}.txt", info.year, info.day)).unwrap(),
        )
        .lines()
        .map(|line| {
            let s = line.unwrap();
            let mut arr = s.split_ascii_whitespace();
            Instr {
                op: arr.next().unwrap().parse::<Op>().unwrap(),
                val: arr.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect()
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        match run_prog(input, None) {
            ProgResult::Terminated(_) => {
                panic!();
            }
            ProgResult::Looped(acc) => acc,
        }
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        for idx in 0..input.len() - 1 {
            match run_prog(input, Some(idx)) {
                ProgResult::Looped(_) => {
                    continue;
                }
                ProgResult::Terminated(acc) => {
                    return acc;
                }
            }
        }
        panic!();
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (1384, 761)
    }
}

/// Run program until it terminates or loops.
///
/// # Arguments
///
/// * `prog` - A list of instructions.
/// * `maybe_flip` - An optional index of an instruction which should be  
///   flipped from jmp <-> nop;
fn run_prog(prog: &[Instr], maybe_flip: Option<usize>) -> ProgResult {
    let mut pc: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited: Vec<bool> = vec![false; prog.len()];

    loop {
        let idx = pc as usize;
        // Program terminated by executing past the last instruction
        if idx >= prog.len() {
            return ProgResult::Terminated(acc);
        }
        if visited[idx] {
            return ProgResult::Looped(acc);
        } else {
            visited[idx] = true;
        }

        let instr = &prog[idx];
        let op = match (&instr.op, maybe_flip) {
            (Op::jmp, Some(flip)) if flip == idx => &Op::nop,
            (Op::nop, Some(flip)) if flip == idx => &Op::jmp,
            (_, _) => &instr.op,
        };
        match op {
            Op::acc => {
                acc += instr.val;
                pc += 1;
            }
            Op::jmp => {
                pc += instr.val;
            }
            Op::nop => {
                pc += 1;
            }
        }
    }
}
