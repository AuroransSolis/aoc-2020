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

fn y_x_flip_pos<'a>(
    sim: &'a [Vec<Position>],
    flips: &'a mut [Vec<bool>],
) -> impl Iterator<Item = (usize, usize, &'a mut bool, Position)> + 'a {
    flips
        .iter_mut()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, flip)| (y, x, flip))
        })
        .map(move |(y, x, flip)| (y, x, flip, sim[y][x]))
}

fn coords_within_lims(y_lim: usize, x_lim: usize, y: isize, x: isize) -> bool {
    y >= 0 && y_lim as isize > y && x >= 0 && x_lim as isize > x
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> (Vec<Vec<Position>>, Vec<Vec<bool>>) {
    let seats = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Position::Floor,
                    'L' => Position::Empty,
                    '#' => Position::Occupied,
                    _ => panic!(format!("Unknown char: {}", c)),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut flips = Vec::with_capacity(seats.len());
    (0..flips.capacity()).for_each(|i| flips.push(vec![false; seats[i].len()]));
    (seats, flips)
}

fn sim_step_p1(sim: &[Vec<Position>], flips: &mut [Vec<bool>]) -> bool {
    y_x_flip_pos(sim, flips)
        .filter(|&(.., spot)| spot != Position::Floor)
        .map(|(y, x, flip, spot)| {
            (
                flip,
                spot,
                EIGHT_DIRS
                    .iter()
                    .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                    .filter(|&(y, x)| coords_within_lims(sim.len(), sim[0].len(), y, x))
                    .map(|(y, x)| (y as usize, x as usize))
                    .map(|(y, x)| sim[y][x])
                    .filter(|&position| position == Position::Occupied)
                    .count(),
            )
        })
        .map(|(flip, spot, occ_adj)| {
            (
                flip,
                (spot == Position::Occupied && occ_adj > 3)
                    || (spot == Position::Empty && occ_adj == 0),
            )
        })
        .map(|(flip, needs_flip)| (*flip = needs_flip, needs_flip))
        .fold(false, |needs_flips, (_, needs_flip)| {
            needs_flips || needs_flip
        })
}

#[aoc(day11, part1)]
pub fn part1((seats, flips): &(Vec<Vec<Position>>, Vec<Vec<bool>>)) -> usize {
    let mut seats = seats.clone();
    let mut flips = flips.clone();
    while sim_step_p1(&seats, &mut flips) {
        seats
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .zip(flips.iter().flat_map(|row| row.iter()))
            .filter(|&(_, &flip)| flip)
            .for_each(|(spot, _)| spot.flip());
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&pos| pos == Position::Occupied)
        .count()
}

fn sim_step_p2(sim: &[Vec<Position>], flips: &mut [Vec<bool>]) -> bool {
    y_x_flip_pos(sim, flips)
        .map(|(y, x, flip, spot)| {
            (
                flip,
                spot,
                EIGHT_DIRS
                    .iter()
                    .map(|(dy, dx)| {
                        (1..)
                            .map(move |mul| (dy * mul, dx * mul))
                            .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                            .take_while(|&(y, x)| coords_within_lims(sim.len(), sim[0].len(), y, x))
                            .map(|(y, x)| (y as usize, x as usize))
                            .map(|(y, x)| sim[y][x])
                            .find(|&position| position != Position::Floor)
                            .map(|position| position == Position::Occupied)
                            .unwrap_or(false)
                    })
                    .filter(|&occ_in_dir| occ_in_dir)
                    .count(),
            )
        })
        .map(|(flip, spot, occ_adj)| {
            (
                flip,
                (spot == Position::Occupied && occ_adj > 4)
                    || (spot == Position::Empty && occ_adj == 0),
            )
        })
        .map(|(flip, needs_flip)| (*flip = needs_flip, needs_flip))
        .fold(false, |needs_flips, (_, needs_flip)| {
            needs_flips || needs_flip
        })
}

#[aoc(day11, part2)]
pub fn part2((seats, flips): &(Vec<Vec<Position>>, Vec<Vec<bool>>)) -> usize {
    let mut seats = seats.clone();
    let mut flips = flips.clone();
    while sim_step_p2(&seats, &mut flips) {
        seats
            .iter_mut()
            .flat_map(|row| row.iter_mut())
            .zip(flips.iter().flat_map(|row| row.iter()))
            .filter(|&(_, &flip)| flip)
            .for_each(|(spot, _)| spot.flip());
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&pos| pos == Position::Occupied)
        .count()
}
