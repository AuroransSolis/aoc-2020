use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| {
            let mut all = group
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();
            let mut first = all.pop().unwrap();
            all
                .into_iter()
                .for_each(|set| {
                    let common = first.intersection(&set).copied().collect::<HashSet<_>>();
                    first = common;
                });
            first.len()
        })
        .sum::<usize>()
}
