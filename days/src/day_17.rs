use aoc_runner_derive::aoc;
use std::{collections::HashSet, hash::Hash};

fn count_active_surrounding<Coord, Surround, SurroundIter>(
    coord: Coord,
    coords: Surround,
    cur: &HashSet<Coord>,
) -> usize
where
    Coord: Clone + Copy + Eq + Hash,
    Surround: Fn(Coord) -> SurroundIter,
    SurroundIter: Iterator<Item = Coord>,
{
    coords(coord).filter(|coord| cur.contains(coord)).count()
}

fn find_new_active_around<Coord, Surround, SurroundIter>(
    coord: Coord,
    coords: Surround,
    cur: &HashSet<Coord>,
    new: &mut HashSet<Coord>,
) where
    Coord: Clone + Copy + Eq + Hash,
    Surround: Fn(Coord) -> SurroundIter,
    SurroundIter: Iterator<Item = Coord>,
{
    coords(coord)
        .filter(|coord| !cur.contains(coord))
        .filter(|coord| count_active_surrounding(*coord, &coords, cur) == 3)
        .for_each(|coord| drop(new.insert(coord)))
}

fn set_cells<Coord, Surround, SurroundIter>(
    coords: Surround,
    cur: &HashSet<Coord>,
    new: &mut HashSet<Coord>,
) where
    Coord: Clone + Copy + Eq + Hash,
    Surround: Fn(Coord) -> SurroundIter,
    SurroundIter: Iterator<Item = Coord>,
{
    cur.iter()
        .copied()
        .map(|coord| (coord, count_active_surrounding(coord, &coords, cur)))
        .for_each(|(coord, act_adj)| {
            find_new_active_around(coord, &coords, cur, new);
            if act_adj == 2 || act_adj == 3 {
                let _ = new.insert(coord);
            }
        })
}

fn adj_iter_3d() -> impl Iterator<Item = [isize; 3]> {
    (-1..=1)
        .flat_map(|dz| (-1..=1).map(move |dy| [dz, dy]))
        .flat_map(|[dz, dy]| (-1..=1).map(move |dx| [dz, dy, dx]))
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let mut cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| c == '#')
                .enumerate()
                .filter(|&(_, active)| active)
                .map(move |(x, _)| [0, y as isize, x as isize])
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let mut new_cells = HashSet::with_capacity(cells.len());
        set_cells(
            |[z, y, x]| {
                adj_iter_3d()
                    .filter(|&diff| diff != [0; 3])
                    .map(move |[dz, dy, dx]| [z + dz, y + dy, x + dx])
            },
            &cells,
            &mut new_cells,
        );
        cells = new_cells;
    }
    cells.len()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    let mut cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| c == '#')
                .enumerate()
                .filter(|&(_, active)| active)
                .map(move |(x, _)| [0, 0, y as isize, x as isize])
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let mut new_cells = HashSet::with_capacity(cells.len());
        set_cells(
            |[w, z, y, x]| {
                adj_iter_3d()
                    .flat_map(|[dw, dz, dy]| (-1..=1).map(move |dx| [dw, dz, dy, dx]))
                    .filter(|&diff| diff != [0; 4])
                    .map(move |[dw, dz, dy, dx]| [w + dw, z + dz, y + dy, x + dx])
            },
            &cells,
            &mut new_cells,
        );
        cells = new_cells;
    }
    cells.len()
}
