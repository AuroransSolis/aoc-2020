use aoc_runner_derive::aoc;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Key {
    BirthYear = 1 << 0,
    IssueYear = 1 << 1,
    ExpirationYear = 1 << 2,
    Height = 1 << 3,
    HairColour = 1 << 4,
    EyeColour = 1 << 5,
    PassportId = 1 << 6,
    CountryId = 1 << 7,
}

const MASK: usize = 0b1111111;

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Key::BirthYear),
            "iyr" => Ok(Key::IssueYear),
            "eyr" => Ok(Key::ExpirationYear),
            "hgt" => Ok(Key::Height),
            "hcl" => Ok(Key::HairColour),
            "ecl" => Ok(Key::EyeColour),
            "pid" => Ok(Key::PassportId),
            "cid" => Ok(Key::CountryId),
            _ => Err(format!("unknown key: '{}'", s)),
        }
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let mut fields = 0;
    let mut count = 0;
    input.lines().for_each(|line| {
        if !line.is_empty() {
            line.split_whitespace().for_each(|section| {
                fields |= section.split(':').next().unwrap().parse::<Key>().unwrap() as usize
            })
        } else {
            if fields & MASK == MASK {
                count += 1;
            }
            fields = 0;
        }
    });
    if fields & MASK == MASK {
        count += 1;
    }
    count
}

fn val_in_range(lo: usize, hi: usize, s: &str) -> bool {
    s.parse::<usize>()
        .map(|val| (lo..=hi).contains(&val))
        .unwrap_or(false)
}

impl Key {
    fn valid_value(&self, value: &str) -> bool {
        match self {
            Key::BirthYear => val_in_range(1920, 2002, value),
            Key::IssueYear => val_in_range(2010, 2020, value),
            Key::ExpirationYear => val_in_range(2020, 2030, value),
            Key::Height => {
                if value.ends_with("cm") {
                    val_in_range(150, 193, value.trim_end_matches("cm"))
                } else if value.ends_with("in") {
                    val_in_range(59, 76, value.trim_end_matches("in"))
                } else {
                    false
                }
            }
            Key::HairColour => {
                if value.starts_with('#') {
                    let trimmed = value.trim_start_matches('#');
                    trimmed.len() == 6 && trimmed.chars().all(|c| c.is_digit(16))
                } else {
                    false
                }
            }
            Key::EyeColour => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            Key::PassportId => value.len() == 9 && value.chars().all(|c| c.is_digit(10)),
            Key::CountryId => true,
        }
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .lines()
                .flat_map(|line| line.split_whitespace())
                .map(|section| {
                    let mut kv = section.split(':');
                    let key = kv.next().unwrap().parse::<Key>().unwrap();
                    (key, key.valid_value(kv.next().unwrap()))
                })
                .take_while(|&(_, valid)| valid)
                .fold(0, |fields, (field, _)| fields | field as usize)
        })
        .filter(|&fields| fields & MASK == MASK)
        .count()
}
