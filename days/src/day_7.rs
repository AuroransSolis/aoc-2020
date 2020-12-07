use aoc_runner_derive::aoc;
use std::collections::HashMap;

const LOOKFOR_P1: &str = "shiny gold";

fn rec_lookfor_p1(map: &HashMap<String, Vec<(usize, String)>>, parent: &String) -> bool {
    let mut any_children = false;
    for (_, child) in map.get(parent).unwrap().iter() {
        if child.as_str() == LOOKFOR_P1 {
            return true;
        } else if child.as_str() == "" {
            return false;
        } else {
            any_children |= rec_lookfor_p1(map, child);
        }
    }
    any_children
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let collections = input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace().collect::<Vec<_>>();
            let parent = format!("{} {}", words.remove(0), words.remove(0));
            let children = words[2..]
                .chunks(4)
                .map(|chunk| {
                    if chunk == &["no", "other", "bags."] {
                        (0usize, String::new())
                    } else {
                        let count = chunk[0].parse::<usize>().unwrap();
                        let child = format!("{} {}", chunk[1], chunk[2]);
                        (count, child)
                    }
                })
                .collect::<Vec<_>>();
            (parent, children)
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();
    collections
        .keys()
        .filter(|parent| parent.as_str() != LOOKFOR_P1)
        .map(|parent| rec_lookfor_p1(&collections, parent))
        .filter(|&contains| contains)
        .count()
}

fn walk_down(map: &HashMap<String, Vec<(usize, String)>>, parent: &String) -> usize {
    let children = map.get(parent).unwrap();
    if children.is_empty() {
        1
    } else {
        let mut total = 0;
        for (amt_in_parent, child) in children {
            let add = amt_in_parent * walk_down(map, child);
            total += add;
        }
        total + 1
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let collections = input
        .lines()
        .map(|line| {
            let mut words = line.split_whitespace().collect::<Vec<_>>();
            let parent = format!("{} {}", words.remove(0), words.remove(0));
            let mut children = Vec::new();
            words[2..].chunks(4).for_each(|chunk| {
                if chunk != &["no", "other", "bags."] {
                    let count = chunk[0].parse::<usize>().unwrap();
                    let child = format!("{} {}", chunk[1], chunk[2]);
                    children.push((count, child));
                }
            });
            (parent, children)
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();
    walk_down(&collections, &("shiny gold".to_string())) - 1
}

/*
shiny gold {
 7,
 2 * {
  11
 } => 22,
} => 29
*/
