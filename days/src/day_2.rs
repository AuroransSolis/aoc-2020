use aoc_runner_derive::aoc;

fn get_parts(line: &str) -> ([usize; 2], char, &str) {
    let mut sections = line.split_whitespace();
    let mut range_parts = sections.next().unwrap().split('-');
    let min = range_parts.next().unwrap().parse().unwrap();
    let max = range_parts.next().unwrap().parse().unwrap();
    let test_char = sections.next().unwrap().chars().next().unwrap();
    let password = sections.next().unwrap();
    ([min, max], test_char, password)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let ([min, max], test_char, password) = get_parts(line);
            let count = password.as_bytes().iter().filter(|&&b| b == (test_char as u8)).count();
            count >= min && count <= max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let ([loc1, loc2], test_char, password) = get_parts(line);
            (password.as_bytes()[loc1 - 1] == test_char as u8)
                ^ (password.as_bytes()[loc2 - 1] == test_char as u8)
        })
        .count()
}
