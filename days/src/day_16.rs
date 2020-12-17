use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let mut parts = input.split('-');
    let start = parts.next().unwrap().parse::<usize>().unwrap();
    let end = parts.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let rules = rules
        .lines()
        .flat_map(|line| line.split(": ").skip(1))
        .map(|pair| {
            let mut parts = pair.split(" or ");
            (
                parse_range(parts.next().unwrap()),
                parse_range(parts.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    let _ = parts.next().unwrap();
    let tickets = parts.next().unwrap();
    tickets
        .lines()
        .skip(1)
        .flat_map(|line| line.split(',').map(|num| num.parse::<usize>().unwrap()))
        .filter(|val| {
            !rules
                .iter()
                .any(|(lower, upper)| lower.contains(&val) || upper.contains(&val))
        })
        .sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let mut parts = input.split("\n\n");
    let mut departure_rules = Vec::new();
    let rules = parts.next().unwrap();
    let rules = rules
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>())
        .enumerate()
        .map(|(rule_num, parts)| {
            if parts[0].starts_with("departure") {
                departure_rules.push(rule_num);
            }
            parts[1]
        })
        .map(|pair| {
            let mut parts = pair.split(" or ");
            (
                parse_range(parts.next().unwrap()),
                parse_range(parts.next().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    let my_ticket = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let tickets = parts.next().unwrap();
    let valid_tickets = tickets
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|vals| {
            vals.iter().all(|val| {
                rules
                    .iter()
                    .any(|(lower, upper)| lower.contains(&val) || upper.contains(&val))
            })
        })
        .collect::<Vec<_>>();
    let mut possible_tickets = (0..valid_tickets[0].len())
        .map(|ticket_ind| {
            rules
                .iter()
                .enumerate()
                .filter(|(_, (lower, higher))| {
                    valid_tickets
                        .iter()
                        .map(|ticket| ticket[ticket_ind])
                        .all(|ticket_val| {
                            lower.contains(&ticket_val) || higher.contains(&ticket_val)
                        })
                })
                .map(|(rule_num, _)| rule_num)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut ticket_map = Vec::with_capacity(rules.len());
    (0..rules.len()).for_each(|_| ticket_map.push(0));
    while !possible_tickets.iter().all(|list| list.is_empty()) {
        let one_poss_pos = possible_tickets
            .iter()
            .position(|poss_list| poss_list.len() == 1)
            .unwrap();
        let only_poss = possible_tickets[one_poss_pos].remove(0);
        ticket_map[only_poss] = one_poss_pos;
        possible_tickets
            .iter_mut()
            .filter(|list| !list.is_empty())
            .for_each(|poss_list| {
                poss_list
                    .iter()
                    .position(|&poss| poss == only_poss)
                    .map(|pos| poss_list.remove(pos));
            });
    }
    departure_rules
        .into_iter()
        .map(|rule_num| ticket_map[rule_num])
        .map(|ticket_ind| my_ticket[ticket_ind])
        .product()
}
