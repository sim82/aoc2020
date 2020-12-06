#![feature(iterator_fold_self)]
use std::{collections::HashSet, io::Read};

fn union(group: &str) -> usize {
    group
        .replace("\n", "")
        .chars()
        .collect::<HashSet<_>>()
        .len()
    // let mut answers: Vec<_> = group.replace("\n", "").chars().collect();
    // answers.sort();
    // answers.dedup();
    // answers.len()
}

fn intersection(group: &str) -> usize {
    group
        .split("\n")
        .map(|person| person.chars().collect::<HashSet<char>>())
        .fold_first(|a, b| a.intersection(&b).cloned().collect::<HashSet<char>>())
        .unwrap()
        .len()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    println!(
        "union: {}",
        input.split("\n\n").map(|g| union(g)).sum::<usize>()
    );
    println!(
        "intersection: {}",
        input.split("\n\n").map(|g| intersection(g)).sum::<usize>()
    );
}
