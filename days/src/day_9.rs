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
        .windows(26)
        .find(|&window| {
            !window[0..24]
                .iter()
                .copied()
                .enumerate()
                .flat_map(|(ind, n1)| window[ind + 1..25].iter().copied().map(move |n2| (n1, n2)))
                .any(|(n1, n2)| n1 + n2 == window[25])
        })
        .unwrap()[25]
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
