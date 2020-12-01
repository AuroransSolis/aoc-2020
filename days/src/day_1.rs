use aoc_runner_derive::{aoc, aoc_generator};

// const TARGET_SUM: usize = 2020;
const TARGET_SUM: usize = 99920044;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1, For)]
pub fn part1_for(input: &[usize]) -> usize {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if input[i] + input[j] == TARGET_SUM {
                return input[i] * input[j];
            }
        }
    }
    0
}

#[aoc(day1, part2, For)]
pub fn part2_for(input: &[usize]) -> usize {
    for i in 0..input.len() - 2 {
        for j in (i + 1..input.len() - 1).filter(|&j| input[i] + input[j] < TARGET_SUM) {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == TARGET_SUM {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

use std::collections::HashSet;

#[aoc(day1, part1, HashSet)]
pub fn part1_hashset(input: &[usize]) -> usize {
    let mut map = HashSet::with_capacity(input.len());
    let mut max = input[0];
    let mut min = max;
    for val in input.iter().copied() {
        map.insert(val);
        if val > max {
            max = val;
        } else if val < min {
            min = val;
        }
    }
    for (n1, diff) in map
        .iter()
        .copied()
        .map(|n1| (n1, TARGET_SUM - n1))
        .filter(|&(_, diff)| diff >= min && diff <= max)
    {
        if map.contains(&diff) {
            return n1 * diff;
        }
    }
    0
}

#[aoc(day1, part2, HashSet)]
pub fn part2_hashset(input: &[usize]) -> u128 {
    let mut map = HashSet::with_capacity(input.len());
    let mut max = input[0];
    let mut min = max;
    for val in input.iter().copied() {
        map.insert(val);
        if val > max {
            max = val;
        } else if val < min {
            min = val;
        }
    }
    for (n1, diff1) in map
        .iter()
        .copied()
        .map(|n1| (n1, TARGET_SUM - n1))
        .filter(|&(_, diff1)| diff1 >= min && diff1 <= max)
    {
        for (_, diff2, prod) in map
            .iter()
            .copied()
            .filter(|&n2| n1 != n2)
            .map(|n2| (n2, diff1 - n2, n1 * n2))
            .filter(|&(n2, diff2, _)| diff2 >= min && diff2 <= max && diff2 != n1 && diff2 != n2)
        {
            if map.contains(&diff2) {
                return diff2 as u128 * prod as u128;
            }
        }
    }
    0
}
