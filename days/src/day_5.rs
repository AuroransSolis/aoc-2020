use aoc_runner_derive::aoc;

// Regular input
const ROW_BITS: usize = 7;
const COL_BITS: usize = 3;

// Large input
// const ROW_BITS: usize = 13;
// const COL_BITS: usize = 5;

const NUM_SEATS: usize = (1 << ROW_BITS) * (1 << COL_BITS);

fn map_to_binary(s: &str, one: u8) -> usize {
    s.bytes()
        .rev()
        .enumerate()
        .filter(|&(_, byte)| byte == one)
        .map(|(shift, _)| 1 << shift)
        .sum::<usize>()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(ROW_BITS))
        .map(|(row, col)| (map_to_binary(row, b'B') << COL_BITS) + map_to_binary(col, b'R'))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut flags = [true; NUM_SEATS];
    let min = input
        .lines()
        .map(|line| line.split_at(ROW_BITS))
        .map(|(row, col)| (map_to_binary(row, b'B') << COL_BITS) + map_to_binary(col, b'R'))
        .inspect(|&id| flags[id] = false)
        .min()
        .unwrap();
    flags[min..].iter().position(|&unoccupied| unoccupied).unwrap() + min
}
