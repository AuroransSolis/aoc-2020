use aoc_runner_derive::aoc;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut nums = input
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for ind in nums.len() - 1..2020 {
        if let Some(pos) = nums[0..ind].iter().rposition(|&n| n == nums[ind]) {
            nums.push(ind - pos);
        } else {
            nums.push(0);
        }
    }
    nums[2020 - 1]
}

use std::collections::HashMap;

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut nums = HashMap::with_capacity(30000000);
    let mut last_num = 0;
    input
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .inspect(|&num| last_num = num)
        .enumerate()
        .for_each(|(pos, num)| drop(nums.insert(num, (0, pos))));
    for count in nums.len()..30_000_000 {
        let &(since_last, _) = nums.get(&last_num).unwrap();
        nums.entry(since_last)
            .and_modify(|(since_last, said_when)| {
                *since_last = count - *said_when;
                *said_when = count;
            })
            .or_insert((0, count));
        last_num = since_last;
    }
    last_num
}
