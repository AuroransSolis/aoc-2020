use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Position {
    Occupied,
    Empty,
    Floor,
}

impl Position {
    fn flip(&mut self) {
        *self = match self {
            Position::Occupied => Position::Empty,
            Position::Empty => Position::Occupied,
            _ => *self,
        };
    }
}

const EIGHT_DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn coords_within_lims(y_lim: usize, x_lim: usize, y: isize, x: isize) -> bool {
    y >= 0 && y_lim as isize > y && x >= 0 && x_lim as isize > x
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> (usize, Vec<Position>, Vec<bool>) {
    let mut lines = input.lines();
    let mut seats = Vec::new();
    let width = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => Position::Occupied,
            'L' => Position::Empty,
            '.' => Position::Floor,
            other => panic!(format!("Unknown character: {}", other)),
        })
        .inspect(|&c| seats.push(c))
        .count();
    lines
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '#' => Position::Occupied,
            'L' => Position::Empty,
            '.' => Position::Floor,
            other => panic!(format!("Unknown character: {}", other)),
        })
        .for_each(|c| seats.push(c));
    let flips = vec![false; seats.len()];
    (width, seats, flips)
}

fn sim_step_p1(width: usize, sim: &[Position], flips: &mut [bool]) -> bool {
    (0..sim.len() / width)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .map(|(y, x)| (y, x, sim[y * width + x]))
        .filter(|&(.., spot)| spot != Position::Floor)
        .map(|(y, x, spot)| {
            (
                y * width + x,
                spot,
                EIGHT_DIRS
                    .iter()
                    .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                    .filter(|&(y, x)| coords_within_lims(sim.len() / width, width, y, x))
                    .map(|(y, x)| (y as usize, x as usize))
                    .map(|(y, x)| sim[y * width + x])
                    .filter(|&position| position == Position::Occupied)
                    .count(),
            )
        })
        .map(|(ind, spot, occ_adj)| {
            (
                ind,
                (spot == Position::Occupied && occ_adj > 3)
                    || (spot == Position::Empty && occ_adj == 0),
            )
        })
        .map(|(ind, needs_flip)| (flips[ind] = needs_flip, needs_flip))
        .fold(false, |needs_flips, (_, needs_flip)| {
            needs_flips || needs_flip
        })
}

#[aoc(day11, part1)]
pub fn part1((width, seats, flips): &(usize, Vec<Position>, Vec<bool>)) -> usize {
    let mut seats = seats.clone();
    let mut flips = flips.clone();
    while sim_step_p1(*width, &seats, &mut flips) {
        seats
            .iter_mut()
            .zip(flips.iter())
            .filter(|&(_, &flip)| flip)
            .for_each(|(spot, _)| spot.flip());
    }
    seats
        .into_iter()
        .filter(|&pos| pos == Position::Occupied)
        .count()
}

fn sim_step_p2(width: usize, sim: &[Position], flips: &mut [bool]) -> bool {
    (0..sim.len() / width)
        .flat_map(|y| (0..width).map(move |x| (y, x)))
        .map(|(y, x)| (y, x, sim[y * width + x]))
        .map(|(y, x, spot)| {
            (
                y * width + x,
                spot,
                EIGHT_DIRS
                    .iter()
                    .map(|(dy, dx)| {
                        (1..)
                            .map(move |mul| (dy * mul, dx * mul))
                            .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                            .take_while(|&(y, x)| coords_within_lims(sim.len() / width, width, y, x))
                            .map(|(y, x)| (y as usize, x as usize))
                            .map(|(y, x)| sim[y * width + x])
                            .find(|&position| position != Position::Floor)
                            .map(|position| position == Position::Occupied)
                            .unwrap_or(false)
                    })
                    .filter(|&occ_in_dir| occ_in_dir)
                    .count(),
            )
        })
        .map(|(ind, spot, occ_adj)| {
            (
                ind,
                (spot == Position::Occupied && occ_adj > 4)
                    || (spot == Position::Empty && occ_adj == 0),
            )
        })
        .map(|(ind, needs_flip)| (flips[ind] = needs_flip, needs_flip))
        .fold(false, |needs_flips, (_, needs_flip)| {
            needs_flips || needs_flip
        })
}

#[aoc(day11, part2)]
pub fn part2((width, seats, flips): &(usize, Vec<Position>, Vec<bool>)) -> usize {
    let mut seats = seats.clone();
    let mut flips = flips.clone();
    while sim_step_p2(*width, &seats, &mut flips) {
        seats
            .iter_mut()
            .zip(flips.iter())
            .filter(|&(_, &flip)| flip)
            .for_each(|(spot, _)| spot.flip());
    }
    seats
        .into_iter()
        .filter(|&pos| pos == Position::Occupied)
        .count()
}
