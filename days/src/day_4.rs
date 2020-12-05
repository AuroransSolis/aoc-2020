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
                fields |= section.split_at(3).0.parse::<Key>().unwrap() as usize
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

impl Key {
    fn valid_value(&self, value: &str) -> bool {
        match self {
            Key::BirthYear => value.len() == 4 && value >= "1920" && "2002" >= value,
            Key::IssueYear => value.len() == 4 && value >= "2010" && "2020" >= value,
            Key::ExpirationYear => value.len() == 4 && value >= "2020" && "2030" >= value,
            Key::Height => {
                if value.len() == 4 {
                    let (value, units) = value.split_at(2);
                    units == "in" && value >= "59" && "76" >= value
                } else if value.len() == 5 {
                    let (value, units) = value.split_at(3);
                    units == "cm" && value >= "150" && "193" >= value
                } else {
                    false
                }
            }
            Key::HairColour => {
                let (start, value) = value.split_at(1);
                start == "#" && value.len() == 6 && value.bytes().all(|c| c.is_ascii_hexdigit())
            }
            Key::EyeColour => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            Key::PassportId => value.len() == 9 && value.bytes().all(|c| c.is_ascii_digit()),
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
                .flat_map(|line| line.split_ascii_whitespace())
                .map(|section| {
                    let (key, val) = section.split_at(3);
                    let key = key.parse::<Key>().unwrap();
                    (key, key.valid_value(val.split_at(1).1))
                })
                .take_while(|&(_, valid)| valid)
                .fold(0, |fields, (field, _)| fields | field as usize)
        })
        .filter(|&fields| fields & MASK == MASK)
        .count()
}
