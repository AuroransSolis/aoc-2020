use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut mem = HashMap::new();
    for part in input.split("mask = ").filter(|s| !s.is_empty()) {
        let mut lines = part.trim().lines();
        let mut ones_mask = 0;
        let mut zeros_mask = 0;
        let mask = lines.next().unwrap();
        mask.chars()
            .rev()
            .enumerate()
            .filter(|&(_, c)| c != 'X')
            .for_each(|(shift, c)| match c {
                '0' => zeros_mask |= 1 << shift,
                '1' => ones_mask |= 1 << shift,
                _ => panic!(format!("Unknown mask char: {}", c)),
            });
        zeros_mask = !zeros_mask;
        for line in lines {
            let mut halves = line.split("] = ");
            let memloc = halves.next().unwrap();
            let (_, loc) = memloc.split_at(4);
            let loc = loc.parse::<usize>().unwrap();
            let mut value = halves.next().unwrap().parse::<usize>().unwrap();
            value |= ones_mask;
            value &= zeros_mask;
            mem.insert(loc, value);
        }
    }
    mem.into_iter().map(|(_, value)| value).sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let mut mem = HashMap::new();
    for part in input.split("mask = ").filter(|s| !s.is_empty()) {
        let mut lines = part.trim().lines();
        let mask = lines.next().unwrap();
        let mut dcs = Vec::new();
        let mut ones_mask = 0;
        mask.chars()
            .rev()
            .enumerate()
            .for_each(|(shift, c)| match c {
                '0' => {}
                '1' => ones_mask |= 1 << shift,
                'X' => dcs.push(shift),
                _ => panic!(format!("Unknown mask char: {}", c)),
            });
        for line in lines {
            let mut halves = line.split("] = ");
            let memloc = halves.next().unwrap();
            let (_, loc) = memloc.split_at(4);
            let mut loc = loc.parse::<usize>().unwrap();
            loc |= ones_mask;
            let value = halves.next().unwrap().parse::<usize>().unwrap();
            for fill in 0..1 << dcs.len() {
                let mut dc_ones = 0;
                let mut dc_zeros = 0;
                for shift in 0..dcs.len() {
                    if fill >> shift & 1 == 1 {
                        dc_ones |= 1 << dcs[shift];
                    } else {
                        dc_zeros |= 1 << dcs[shift];
                    }
                }
                dc_zeros = !dc_zeros;
                let mut write_addr = loc;
                write_addr |= dc_ones;
                write_addr &= dc_zeros;
                mem.insert(write_addr, value);
            }
        }
    }
    mem.into_iter().map(|(_, value)| value).sum()
}
