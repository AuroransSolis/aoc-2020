use aoc_runner_derive::{aoc, aoc_generator};
use voracious_radix_sort::RadixSort;

const TARGET_SUM: usize = 2020;
// const TARGET_SUM: usize = 99920044;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut input = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    input.voracious_sort();
    input
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

#[aoc(day1, part1, ThreeSum)]
pub fn part1_3sum(input: &[usize]) -> usize {
    let mut start = 0;
    let mut end = input.len() - 1;
    while start < end {
        let n1 = input[start];
        let n2 = input[end];
        let sum = n1 + n2;
        if sum == TARGET_SUM {
            return n1 * n2;
        } else if sum > TARGET_SUM {
            end -= 1;
        } else {
            start += 1;
        }
    }
    0
}

/*#[aoc(day1, part2, For)]
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
}*/

#[aoc(day1, part2, ThreeSum)]
pub fn part2_3sum(input: &[usize]) -> u128 {
    for i in 0..input.len() - 2 {
        let n1 = input[i];
        let mut start = i + 1;
        let mut end = input.len() - 1;
        while start < end {
            let n2 = input[start];
            let n3 = input[end];
            let sum = n1 + n2 + n3;
            if sum == TARGET_SUM {
                return n1 as u128 * n2 as u128 * n3 as u128;
            } else if sum > TARGET_SUM {
                end -= 1;
            } else {
                start += 1;
            }
        }
    }
    0
}
