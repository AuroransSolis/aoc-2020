use aoc_runner_derive::aoc;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse::<usize>().unwrap();
    lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| id.parse::<usize>().unwrap())
        .map(|id| (id, id - (earliest % id)))
        .min_by(|(_, res_1), (_, res_2)| res_1.cmp(res_2))
        .map(|(id, res)| id * res)
        .unwrap()
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> usize {
    let mut ids_and_times = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(time, id)| (id.parse::<usize>().unwrap(), time));
    let mut value = ids_and_times.next().unwrap().0;
    let mut increment = value;
    for (id, residue) in ids_and_times {
        while (value + residue) % id != 0 {
            value += increment;
        }
        increment *= id;
    }
    value
}
