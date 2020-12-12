use aoc_runner_derive::aoc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Position {
    Occupied,
    Empty,
    Floor,
}

impl Position {
    fn is_occupied(&self) -> bool {
        match self {
            Position::Occupied => true,
            _ => false,
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Position::Empty => true,
            _ => false,
        }
    }

    fn is_floor(&self) -> bool {
        match self {
            Position::Floor => true,
            _ => false,
        }
    }
}

fn do_sim_step_p1(sim: &[Vec<Position>]) -> Vec<Vec<Position>> {
    let mut new = Vec::with_capacity(sim.len());
    for row in 0..sim.len() {
        let mut row_list = Vec::with_capacity(sim[row].len());
        for col in 0..sim[row].len() {
            if !sim[row][col].is_floor() {
                let adjacent_occupied = (-1..=1)
                    .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                    .filter(|&delta| delta != (0, 0))
                    .map(|(dy, dx)| ((row as isize + dy) as usize, (col as isize + dx) as usize))
                    .filter_map(|(y, x)| sim.get(y).map(|row| row.get(x)))
                    .filter_map(|from_row| from_row)
                    .filter(|position| position.is_occupied())
                    .count();
                if sim[row][col].is_occupied() && adjacent_occupied > 3 {
                    row_list.push(Position::Empty);
                } else if sim[row][col].is_empty() && adjacent_occupied == 0 {
                    row_list.push(Position::Occupied)
                } else {
                    row_list.push(sim[row][col]);
                }
            } else {
                row_list.push(Position::Floor);
            }
        }
        new.push(row_list);
    }
    new
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut seats = input
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
    loop {
        let new = do_sim_step_p1(&seats);
        if new == seats {
            break;
        } else {
            seats = new;
        }
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|pos| pos.is_occupied())
        .count()
}

fn do_sim_step_p2(sim: &[Vec<Position>]) -> Vec<Vec<Position>> {
    let mut new = Vec::with_capacity(sim.len());
    for row in 0..sim.len() {
        let mut row_list = Vec::with_capacity(sim[row].len());
        for col in 0..sim[row].len() {
            if !sim[row][col].is_floor() {
                let adjacent_occupied = (-1..=1)
                    .flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
                    .filter(|&delta| delta != (0, 0))
                    .map(|(dy, dx)| {
                        let res = (1..)
                            .map(move |mul| (dy * mul, dx * mul))
                            .take_while(|(dy, dx): &(isize, isize)| {
                                row as isize + dy >= 0
                                    && ((row as isize + dy) as usize) < sim.len()
                                    && col as isize + dx >= 0
                                    && ((col as isize + dx) as usize) < sim[row].len()
                            })
                            .map(|(dy, dx)| {
                                ((row as isize + dy) as usize, (col as isize + dx) as usize)
                            })
                            .map(|(y, x)| sim[y][x])
                            .find(|pos| !pos.is_floor())
                            .map(|pos| pos.is_occupied())
                            .unwrap_or(false);
                        res
                    })
                    .filter(|&has_occupied| has_occupied)
                    .count();
                if sim[row][col].is_occupied() && adjacent_occupied > 4 {
                    row_list.push(Position::Empty);
                } else if sim[row][col].is_empty() && adjacent_occupied == 0 {
                    row_list.push(Position::Occupied)
                } else {
                    row_list.push(sim[row][col]);
                }
            } else {
                row_list.push(Position::Floor);
            }
        }
        new.push(row_list);
    }
    new
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut seats = input
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
    loop {
        let new = do_sim_step_p2(&seats);
        if new == seats {
            break;
        } else {
            seats = new;
        }
    }
    seats
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|pos| pos.is_occupied())
        .count()
}
