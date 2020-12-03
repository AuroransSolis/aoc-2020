use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let width = grid[0].len();
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < grid.len() {
        if grid[y][x % width] == '#' {
            count += 1;
        }
        y += 1;
        x += 3;
    }
    count
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let width = grid[0].len();
    let mut total = 1;
    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    for (dx, dy) in slopes.iter().copied() {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;
        while y < grid.len() {
            if grid[y][x % width] == '#' {
                count += 1;
            }
            x += dx;
            y += dy;
        }
        total *= count;
    }
    total
}
