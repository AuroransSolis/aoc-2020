use aoc_runner_derive::aoc;
use voracious_radix_sort::RadixSort;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut input = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    input.voracious_sort();
    let mut diffs =
        input
            .windows(2)
            .map(|window| window[1] - window[0])
            .fold([0; 4], |mut diffs, diff| {
                diffs[diff] += 1;
                diffs
            });
    diffs[input[0]] += 1;
    diffs[1] * (diffs[3] + 1)
}

use std::collections::{HashMap, HashSet};

pub fn poss_followers(
    cache: &mut HashMap<usize, usize>,
    adaptors: &HashSet<usize>,
    adaptor: usize,
    max: usize,
) -> usize {
    if !adaptors.contains(&adaptor) {
        0
    } else if adaptor == max {
        1
    } else {
        if let Some(&cached) = cache.get(&adaptor) {
            cached
        } else {
            let to_cache = poss_followers(cache, adaptors, adaptor + 1, max)
                + poss_followers(cache, adaptors, adaptor + 2, max)
                + poss_followers(cache, adaptors, adaptor + 3, max);
            cache.insert(adaptor, to_cache);
            to_cache
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut max = 0;
    let input = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .inspect(|&val| max = max.max(val))
        .collect::<HashSet<_>>();
    let mut cache = HashMap::with_capacity(input.len());
    poss_followers(&mut cache, &input, 1, max)
        + poss_followers(&mut cache, &input, 2, max)
        + poss_followers(&mut cache, &input, 3, max)
}
