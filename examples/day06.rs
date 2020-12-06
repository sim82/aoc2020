#![feature(iterator_fold_self)]
use std::{collections::HashSet, io::Read};

fn union(group: &str) -> usize {
    group
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<HashSet<_>>()
        .len()
    // let mut answers: Vec<_> = group.chars().filter(|c| *c != '\n').collect();
    // answers.sort();
    // answers.dedup();
    // answers.len()
}

fn intersection(group: &str) -> usize {
    group
        .split("\n")
        .map(|person| person.chars().collect::<HashSet<_>>())
        .fold_first(|ref a, ref b| a & b)
        .unwrap()
        .len()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    println!("start");
    println!(
        "union: {}",
        input.split("\n\n").map(|g| union(g)).sum::<usize>()
    );
    println!(
        "intersection: {}",
        input.split("\n\n").map(|g| intersection(g)).sum::<usize>()
    );
}
