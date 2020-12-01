use anyhow::Result as AnyResult;
use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            for k in j + 1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

