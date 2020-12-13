use aoc_runner_derive::aoc;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy)]
struct Vec2 {
    y: i32,
    x: i32,
}

impl Vec2 {
    const fn new(y: i32, x: i32) -> Self {
        Vec2 { y, x }
    }

    fn rot_acw(&mut self, amt: i32) {
        for _ in 0..(amt / 90) % 4 {
            let tmp = -self.y;
            self.y = self.x;
            self.x = tmp;
        }
    }

    fn rot_cw(&mut self, amt: i32) {
        for _ in 0..(amt / 90) % 4 {
            let tmp = self.y;
            self.y = -self.x;
            self.x = tmp;
        }
    }

    fn manhattan_dist(&self) -> usize {
        self.y.abs() as usize + self.x.abs() as usize
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec2::new(self.y + rhs.y, self.x + rhs.x)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.y += rhs.y;
        self.x += rhs.x;
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Vec2::new(self.y * rhs, self.x * rhs)
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut ship = Vec2::new(0, 0);
    let mut facing = Vec2::new(0, 1);
    for line in input.lines() {
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<i32>().unwrap();
        if dir == "L" {
            facing.rot_acw(amt);
        } else if dir == "R" {
            facing.rot_cw(amt);
        } else {
            let delta = match dir {
                "N" => Vec2::new(1, 0),
                "S" => Vec2::new(-1, 0),
                "E" => Vec2::new(0, 1),
                "W" => Vec2::new(0, -1),
                "F" => facing,
                _ => continue,
            };
            ship += delta * amt;
        }
    }
    ship.manhattan_dist()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    let mut ship = Vec2::new(0, 0);
    let mut waypoint = Vec2::new(1, 10);
    for line in input.lines() {
        let (dir, amt) = line.split_at(1);
        let amt = amt.parse::<i32>().unwrap();
        match dir {
            "N" => waypoint += Vec2::new(1, 0) * amt,
            "S" => waypoint += Vec2::new(-1, 0) * amt,
            "E" => waypoint += Vec2::new(0, 1) * amt,
            "W" => waypoint += Vec2::new(0, -1) * amt,
            "L" => waypoint.rot_acw(amt),
            "R" => waypoint.rot_cw(amt),
            "F" => ship += waypoint * amt,
            _ => {}
        };
    }
    ship.manhattan_dist()
}
