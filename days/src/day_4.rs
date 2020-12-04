use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

const PASSPORT_FIELDS_P1: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut fields = HashSet::new();
    let mut count = 0;
    for line in input.lines() {
        if line == "" {
            if PASSPORT_FIELDS_P1
                .iter()
                .copied()
                .all(|field| fields.contains(field))
            {
                count += 1;
            }
            fields.clear();
        } else {
            for section in line.split_whitespace() {
                let field = section.split(':').next().unwrap();
                fields.insert(field);
            }
        }
    }
    if PASSPORT_FIELDS_P1
        .iter()
        .copied()
        .all(|field| fields.contains(field))
    {
        count += 1;
    }
    count
}

fn check_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => (1920..=2002).contains(&value.parse::<usize>().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&value.parse::<usize>().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&value.parse::<usize>().unwrap_or(0)),
        "hgt" => {
            if value.ends_with("cm") {
                (150..=193).contains(&value.trim_end_matches("cm").parse::<usize>().unwrap_or(0))
            } else if value.ends_with("in") {
                (59..=76).contains(&value.trim_end_matches("in").parse::<usize>().unwrap_or(0))
            } else {
                false
            }
        }
        "hcl" => {
            value.starts_with('#')
                && value.trim_start_matches('#').len() == 6
                && value.chars().skip(1).all(|c| c.is_digit(16))
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
        "cid" => true,
        _ => false,
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut fields = HashSet::new();
    let mut count = 0;
    let mut early_exit = false;
    for line in input.lines() {
        if line == "" {
            if !early_exit
                && PASSPORT_FIELDS_P1
                    .iter()
                    .copied()
                    .all(|field| fields.contains(field))
            {
                count += 1;
            }
            fields.clear();
            early_exit = false;
        } else if !early_exit {
            for section in line.split_whitespace() {
                let mut kv = section.split(':');
                let key = kv.next().unwrap();
                let val = kv.next().unwrap();
                if check_field(key, val) {
                    fields.insert(key);
                } else {
                    early_exit = true;
                }
            }
        }
    }
    if PASSPORT_FIELDS_P1
        .iter()
        .copied()
        .all(|field| fields.contains(field))
    {
        count += 1;
    }
    count
}
