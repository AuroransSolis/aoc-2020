use aoc_runner_derive::aoc;

fn map_to_binary(s: &str, one: u8) -> usize {
    s.bytes()
        .rev()
        .enumerate()
        .filter(|&(_, b)| b == one)
        .map(|(shift, _)| 1 << shift)
        .sum::<usize>()
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(7))
        .map(|(row, col)| (map_to_binary(row, b'B'), map_to_binary(col, b'R')))
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let mut flags = [true; (1 << 7) * (1 << 3)];
    let min = input
        .lines()
        .map(|line| line.split_at(7))
        .map(|(row, col)| (map_to_binary(row, b'B'), map_to_binary(col, b'R')))
        .map(|(row, col)| row * 8 + col)
        .inspect(|&id| flags[id] = false)
        .min()
        .unwrap();
    flags[min..].iter().position(|&unoccupied| unoccupied).unwrap() + min
}
