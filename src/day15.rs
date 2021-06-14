use crate::puzzle::{Info, Puzzle};
use std::collections::{hash_map::Entry, HashMap};
pub struct Day15 {}

impl Puzzle for Day15 {
    type InputType = Vec<usize>;
    type T1 = usize;
    type T2 = usize;

    fn info(&self) -> Info {
        Info {
            name: "Rambunctious Recitation",
            year: 2020,
            day: 15,
        }
    }
    fn parse_input(&self) -> Self::InputType {
        return vec![6, 4, 12, 1, 20, 0, 16];
    }

    fn part1(&self, input: &Self::InputType) -> Self::T1 {
        solve(&input, 2020, 0.1)
    }

    fn part2(&self, input: &Self::InputType) -> Self::T2 {
        solve(&input, 30_000_000, 0.1)
    }

    fn expected(&self) -> (Self::T1, Self::T2) {
        (475, 11261)
    }
}

/// This is the Van Eck Sequence: https://www.numberphile.com/videos/van-eck-sequence.
/// There is no known way to compute the nth number without computing the entire
/// sequence up to n. Some optimizations are possible (e.g.
/// https://github.com/timvisee/advent-of-code-2020/blob/master/day15b/src/main.rs)
/// but nothing radical.
pub fn solve(input: &[usize], limit: usize, frac: f32) -> usize {
    let mut input0 = input.to_vec();
    let boundary: usize = if limit > 1_000_000 {
        (limit as f32 * frac) as usize
    } else {
        limit
    };

    let mut last = input0.pop().unwrap();
    let mut lower = vec![0; boundary];
    let mut upper = HashMap::new();

    for turn in 1..=input0.len() {
        lower[input0[turn - 1]] = turn;
    }

    for turn in input.len()..limit {
        if last < boundary {
            let index_of_last = lower[last];
            let next = if index_of_last == 0 {
                0
            } else {
                turn - index_of_last
            };
            lower[last] = turn;
            last = next;
        } else {
            last = match upper.entry(last) {
                Entry::Vacant(entry) => {
                    entry.insert(turn);
                    0
                }
                Entry::Occupied(mut entry) => turn - entry.insert(turn),
            }
        }
    }

    last
}
