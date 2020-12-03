use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.bytes().map(|byte| byte == b'#').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn count_trees(input: &[Vec<bool>], dy: usize, dx: usize) -> usize {
    (1..)
        .map(|mul| (mul * dy, mul * dx))
        .take_while(|&(y, _)| y < input.len())
        .filter(|&(y, x)| unsafe { *input.get_unchecked(y).get_unchecked(x % input[0].len()) })
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> usize {
    count_trees(input, 1, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<bool>]) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(dy, dx)| count_trees(input, dy, dx))
        .product()
}
