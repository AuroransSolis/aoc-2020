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
        .filter(|&(y, x)| input[y][x % input[0].len()])
        .count()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<bool>]) -> usize {
    count_trees(input, 1, 1)
}

#[aoc(day3, part2, Regular)]
pub fn part2_reg(input: &[Vec<bool>]) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(dy, dx)| count_trees(input, dy, dx))
        .product()
}

use num_bigint::BigUint;

const LARGE_DXS: [usize; 15] = [2, 3, 4, 6, 8, 9, 12, 16, 18, 24, 32, 36, 48, 54, 64];
const LARGE_DYS: [usize; 15] = [1, 5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37, 41, 47];

#[cfg(not(nightly))]
#[aoc(day3, part2, Large)]
pub fn part2_lg(input: &[Vec<bool>]) -> BigUint {
    LARGE_DYS
        .iter()
        .flat_map(|dy| LARGE_DXS.iter().map(move |dx| (dy, dx)))
        .map(|(dy, dx)| count_trees(input, *dy, *dx))
        .fold(BigUint::from(1usize), |acc, count| acc * count)
}

#[cfg(nightly)]
const fn combine_arrays<const N1: usize, const N2: usize>(
    a1: [usize; N1],
    a2: [usize; N2],
) -> [(usize, usize); N1 * N2] {
    let mut a = [(0, 0); N1 * N2];
    let mut i = 0;
    let mut j = 0;
    while i < N1 {
        while j < N2 {
            a[i * N2 + j] = (a1[i], a2[j]);
            j += 1;
        }
        i += 1;
        j = 0;
    }
    a
}

#[cfg(nightly)]
const LARGE_SLOPES: [(usize, usize); LARGE_DXS.len() * LARGE_DYS.len()] =
    combine_arrays(LARGE_DYS, LARGE_DXS);

#[cfg(nightly)]
#[aoc(day3, part2, Large)]
pub fn part2_lg(input: &[Vec<bool>]) -> BigUint {
    LARGE_SLOPES
        .iter()
        .map(|&(dy, dx)| count_trees(input, dy, dx))
        .fold(BigUint::from(1usize), |acc, count| acc * count)
}
