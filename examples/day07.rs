#![feature(drain_filter)]

use aoc2020::bag::{Content, Policy};
use aoc2020::bag_grammar;
use std::{collections::HashMap, io::Read};
fn has_leaf(map: &HashMap<String, &Policy>, policy: &Policy, color: String) -> bool {
    match policy.content {
        Some(ref content) => content
            .iter()
            .any(|c| c.color == color || has_leaf(map, map[&c.color], color.clone())),
        None => false,
    }
}

fn count_all(map: &HashMap<String, &Policy>, policy: &Policy) -> usize {
    let num = 1 + match policy.content {
        Some(ref content) => content.iter().map(|c| c.num * count_all(map, policy)).sum(),

        None => 0,
    };

    num
}
fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();
    let mut policies: Vec<Policy> = bag_grammar::LinesParser::new().parse(&code[..]).unwrap();
    let policy_map: HashMap<_, _> = policies.iter().map(|p| (p.color.clone(), p)).collect();
    println!("policies: {:?}", policies);
    let num = policies
        .iter()
        .filter(|p| has_leaf(&policy_map, p, "shiny gold".into()))
        .count();
    println!("num: {}", num);
    // println!(
    //     "count_all: {:?}",
    //     count_all(&policy_map, policy_map["shiny gold"])
    // );

    let mut cache: HashMap<String, usize> = policies
        .drain_filter(|p| p.content.is_none())
        .map(|p| (p.color.clone(), 1))
        .collect();

    println!("done: {:?}", cache);
    // dynamic programming: calc bag dependencies bottom-up
    loop {
        if let Some(res) = cache.get("shiny gold") {
            println!("num2: {}", res);
            break;
        }

        let to_close = policies.drain_filter(|p| match p.content {
            Some(ref content) => content.iter().all(|c| cache.get(&c.color).is_some()),
            None => false,
        });

        let ext: Vec<_> = to_close
            .map(|p| {
                let content = p.content.unwrap();
                (
                    p.color,
                    1 + content
                        .iter()
                        .map(|c| c.num * cache[&c.color])
                        .sum::<usize>(),
                )
            })
            .collect();
        //.collect();
        cache.extend(ext);
    }
}
