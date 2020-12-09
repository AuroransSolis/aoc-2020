use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day9, part1)]
pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .copied()
        .enumerate()
        .skip(25)
        .find(|&(ind, num)| {
            input[ind - 25..ind - 1]
                .iter()
                .copied()
                .enumerate()
                .flat_map(|(j, n1)| input[j + 1..ind].iter().copied().map(move |n2| (n1, n2)))
                .any(|(n1, n2)| n1 + n2 == num)
        })
        .unwrap()
        .1
}

#[aoc(day9, part2)]
pub fn part2(input: &[usize]) -> usize {
    let invalid = part1(input);
    (2..input.len())
        .find_map(|window_size| {
            input
                .windows(window_size)
                .find(|&window| window.iter().sum::<usize>() == invalid)
                .map(|window| {
                    window
                        .iter()
                        .fold([!0, 0], |[min, max], &n| [min.min(n), max.max(n)])
                        .iter()
                        .sum()
                })
        })
        .unwrap()
}
