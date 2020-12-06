use aoc_runner_derive::aoc;

fn alphabet_offset(c: char) -> usize {
    c as usize - b'a' as usize
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .map(alphabet_offset)
                .map(|shift| 1usize << shift)
                .fold(0, |flags, flag| flags | flag)
                .count_ones() as usize
        })
        .sum::<usize>()
}

#[cfg(not(nightly))]
#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let mut flags_iter = group.lines().map(|line| {
                line.chars()
                    .map(alphabet_offset)
                    .map(|shift| 1usize << shift)
                    .fold(0, |flags, flag| flags | flag)
            });
            let all = flags_iter.next().unwrap();
            flags_iter
                .fold(all, |all, single| all & single)
                .count_ones() as usize
        })
        .sum::<usize>()
}

#[cfg(nightly)]
#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| {
                    line.chars()
                        .map(alphabet_offset)
                        .map(|shift| 1usize << shift)
                        .fold(0, |flags, flag| flags | flag)
                })
                .fold_first(|all, single| all & single)
                .unwrap()
                .count_ones() as usize
        })
        .sum::<usize>()
}
