use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let sections = line.split_whitespace().collect::<Vec<_>>();
        let counts = sections[0].split('-').collect::<Vec<_>>();
        let min = counts[0].parse::<usize>().unwrap();
        let max = counts[1].parse::<usize>().unwrap();
        let letter = sections[1].split(':').next().unwrap();
        let test_char = letter.chars().next().unwrap();
        let letter_count = sections[2].chars().filter(|&c| c == test_char).count();
        if letter_count >= min && letter_count <= max {
            count += 1;
        }
    }
    count
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let sections = line.split_whitespace().collect::<Vec<_>>();
        let counts = sections[0].split('-').collect::<Vec<_>>();
        let min = counts[0].parse::<usize>().unwrap();
        let max = counts[1].parse::<usize>().unwrap();
        let letter = sections[1].split(':').next().unwrap();
        let test_char = letter.chars().next().unwrap();
        let chars = sections[2].chars().collect::<Vec<char>>();
        let cond_1 = chars[min - 1] == test_char;
        let cond_2 = chars[max - 1] == test_char;
        if (cond_1 && !cond_2) || (!cond_1 && cond_2) {
            count += 1;
        }
    }
    count
}
