use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn set_new_surrounding_p1([z, y, x]: [isize; 3], cur: &HashSet<[isize; 3]>, new: &mut HashSet<[isize; 3]>) {
    (-1..=1)
        .flat_map(|dz| (-1..=1).map(move |dy| [dz, dy]))
        .flat_map(|[dz, dy]| (-1..=1).map(move |dx| [dz, dy, dx]))
        .filter(|&diff| diff != [0; 3])
        .map(|[dz, dy, dx]| [z + dz, y + dy, x + dx])
        .filter(|coord| !cur.contains(coord))
        .map(|coord| (coord, count_surrounding_p1(coord, cur)))
        .filter(|&(_, count)| count == 3)
        .for_each(|(coord, _)| drop(new.insert(coord)));
}

fn count_surrounding_p1([z, y, x]: [isize; 3], cur: &HashSet<[isize; 3]>) -> usize {
    (-1..=1)
        .flat_map(|dz| (-1..=1).map(move |dy| [dz, dy]))
        .flat_map(|[dz, dy]| (-1..=1).map(move |dx| [dz, dy, dx]))
        .filter(|&diff| diff != [0; 3])
        .map(|[dz, dy, dx]| [z + dz, y + dy, x + dx])
        .filter(|coord| cur.contains(coord))
        .count()
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
                .map(move |(x, active)| [0, y as isize, x as isize])
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let mut new_cells = HashSet::with_capacity(cells.len());
        cells
            .iter()
            .copied()
            .map(|coord| (coord, count_surrounding_p1(coord, &cells)))
            .for_each(|(coord, count)| {
                if count == 2 || count == 3 {
                    let _ = new_cells.insert(coord);
                }
                set_new_surrounding_p1(coord, &cells, &mut new_cells)
            });
        cells = new_cells;
    }
    cells.len()
}

fn set_new_surrounding_p2([w, z, y, x]: [isize; 4], cur: &HashSet<[isize; 4]>, new: &mut HashSet<[isize; 4]>) {
    (-1..=1)
        .flat_map(|dw| (-1..=1).map(move |dz| [dw, dz]))
        .flat_map(|[dw, dz]| (-1..=1).map(move |dy| [dw, dz, dy]))
        .flat_map(|[dw, dz, dy]| (-1..=1).map(move |dx| [dw, dz, dy, dx]))
        .filter(|&diff| diff != [0; 4])
        .map(|[dw, dz, dy, dx]| [w + dw, z + dz, y + dy, x + dx])
        .filter(|coord| !cur.contains(coord))
        .map(|coord| (coord, count_surrounding_p2(coord, cur)))
        .filter(|&(_, count)| count == 3)
        .for_each(|(coord, _)| drop(new.insert(coord)));
}

fn count_surrounding_p2([w, z, y, x]: [isize; 4], cur: &HashSet<[isize; 4]>) -> usize {
    (-1..=1)
        .flat_map(|dw| (-1..=1).map(move |dz| [dw, dz]))
        .flat_map(|[dw, dz]| (-1..=1).map(move |dy| [dw, dz, dy]))
        .flat_map(|[dw, dz, dy]| (-1..=1).map(move |dx| [dw, dz, dy, dx]))
        .filter(|&diff| diff != [0; 4])
        .map(|[dw, dz, dy, dx]| [w + dw, z + dz, y + dy, x + dx])
        .filter(|coord| cur.contains(coord))
        .count()
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
                .map(move |(x, active)| [0, 0, y as isize, x as isize])
        })
        .collect::<HashSet<_>>();
    for _ in 0..6 {
        let mut new_cells = HashSet::new();
        cells
            .iter()
            .copied()
            .map(|coord| (coord, count_surrounding_p2(coord, &cells)))
            .for_each(|(coord, count)| {
                if count == 2 || count == 3 {
                    let _ = new_cells.insert(coord);
                }
                set_new_surrounding_p2(coord, &cells, &mut new_cells)
            });
        cells = new_cells;
    }
    cells.len()
}
