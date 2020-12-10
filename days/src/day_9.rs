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
    let mut start = 0;
    let mut end = 2;
    let mut sum = input[0] + input[1];
    while sum != invalid {
        if sum < invalid {
            if input[end + 1] > invalid {
                start = end + 2;
                end = start + 2;
                sum = input[start] + input[start + 1];
            } else {
                sum += input[end];
                end += 1;
            }
        } else if sum > invalid {
            sum -= input[start];
            start += 1;
        }
    }
    let mut min = input[start];
    let mut max = min;
    for &val in &input[start + 1..end] {
        if val > max {
            max = val;
        } else if min > val {
            min = val;
        }
    }
    min + max
}
