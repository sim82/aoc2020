#![feature(iterator_fold_self)]

use regex::{Captures, Regex};
use std::{collections::HashMap, io::Read};
use std::{collections::HashSet, io::BufRead};

fn main() {
    let re_rule = Regex::new(r"([a-z\s]+):\s*(\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let re_section = Regex::new(r"([a-z\s]+):$").unwrap();
    let re_ticket = Regex::new(r"\d+(,\d+)*").unwrap();

    let mut rules: Vec<(String, Vec<core::ops::RangeInclusive<i32>>)> = Vec::new();
    let mut cur_section = "".into();

    let mut my_ticket = Vec::new();
    let mut other_tickets = Vec::new();

    for line in std::io::stdin().lock().lines() {
        // let line = &line.unwrap()[..];
        let line = line.unwrap();
        if let Some(cap) = re_rule.captures(&line) {
            let name = cap.get(1).unwrap().as_str();
            let r1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap()
                ..=cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let r2 = cap.get(4).unwrap().as_str().parse::<i32>().unwrap()
                ..=cap.get(5).unwrap().as_str().parse::<i32>().unwrap();
            rules.push((name.to_string(), vec![r1, r2]));
        } else if let Some(cap) = re_section.captures(&line) {
            cur_section = cap.get(1).unwrap().as_str().to_string();
        } else if let Some(cap) = re_ticket.captures(&line) {
            let ticket = cap
                .get(0)
                .unwrap()
                .as_str()
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if cur_section == "your ticket" {
                my_ticket = ticket;
            } else if cur_section == "nearby tickets" {
                other_tickets.push(ticket);
            } else {
                panic!("unknown ticket section");
            }
        }
    }

    println!("rules: {:?}", rules);
    println!("my ticket: {:?}", my_ticket);
    println!("other tickets: {:?}", other_tickets);

    let all_nums = other_tickets.iter().flat_map(|v| v.iter());
    let error_rate = all_nums
        .filter_map(|n| {
            if !rules
                .iter()
                .any(|(_, ranges)| ranges.iter().any(|r| r.contains(n)))
            {
                println!("bad: {}", n);
                Some(*n)
            } else {
                None
            }
        })
        .sum::<i32>();

    println!("error rate: {}", error_rate);

    let good_tickets = other_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|n| {
                rules
                    .iter()
                    .any(|(_, ranges)| ranges.iter().any(|r| r.contains(n)))
            })
        })
        .collect::<Vec<_>>();

    println!("goood tickets: {:?}", good_tickets);

    let mut open_columns = (0..rules.len() as i32).collect::<HashSet<i32>>();
    let mut assignments: HashMap<String, i32> = HashMap::new();
    while !open_columns.is_empty() {
        let mut new_assignments = HashMap::new();
        for col_n in open_columns.iter() {
            let col = good_tickets
                .iter()
                .map(|ticket| ticket[*col_n as usize])
                .collect::<Vec<_>>();

            let fit = rules
                .iter()
                .filter_map(|(name, ranges)| {
                    if !assignments.contains_key(name)
                        && col
                            .iter()
                            .all(|n| ranges.iter().any(|range| range.contains(n)))
                    {
                        Some(name)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            println!("fit: {:?}", fit);
            if fit.len() == 1 {
                new_assignments.insert(fit[0], *col_n as i32);
            }
        }

        for remove in new_assignments.values() {
            open_columns.remove(remove);
        }
        for (k, v) in new_assignments {
            println!("new assignment: {} {}", k, v);
            assignments.insert(k.clone(), v);
        }
        //assignments.extend(new_assignments.iter().cloned());
        // println!("col {} fit: {:?}", col_n, fit);
    }

    println!("assignments: {:?}", assignments);
    let prod = assignments
        .iter()
        .filter_map(|(name, col)| {
            if name.starts_with("departure") {
                Some(my_ticket[*col as usize] as i64)
            } else {
                None
            }
        })
        .fold_first(|a, i| a * i)
        .unwrap();

    println!("prod {}", prod);

    for (name, col) in assignments {
        if name.starts_with("departure") {
            println!("{} {}", name, my_ticket[col as usize]);
        }
    }
}
