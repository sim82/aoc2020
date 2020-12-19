use aoc2020::message_grammar;
use aoc2020::{
    message::{Element, Node},
    semicolonized_input,
};
use regex::Regex;
use std::{
    collections::HashMap,
    io::{BufRead, Read},
};

fn build_regex(rulenum: Option<i64>, node: &Node, nodes: &HashMap<i64, Node>) -> String {
    match node {
        Node::Seq(v) => {
            match rulenum {
                Some(11) => {
                    assert!(v.len() == 2);
                    let c1 = build_regex(None, &nodes.get(&v[0]).unwrap(), nodes);
                    let c2 = build_regex(None, &nodes.get(&v[1]).unwrap(), nodes);

                    // rewrite rule 11: ab -> a{n}b{m} with n == m
                    // the general case is impossible in a regex without perl-style backrefs.
                    // brute force it by explicit 'unrolling' to (a{1}b{1})|(a{2}b{2})|...|(a{n}b{n})
                    format!(
                        "({})",
                        (1..10)
                            .map(|i| format!("({}){{{}}}({}){{{}}}", c1, i, c2, i))
                            .collect::<Vec<_>>()
                            .join("|")
                    )
                }
                Some(8) => {
                    assert!(v.len() == 1);
                    let c = build_regex(None, &nodes.get(&v[0]).unwrap(), nodes);
                    // rewrite rule 8: a -> a+
                    format!("({})+", c)
                }
                _ => {
                    // regular sequence -> concatenate rules
                    v.iter()
                        .map(|rn| build_regex(Some(*rn), &nodes.get(rn).unwrap(), nodes))
                        .collect::<Vec<_>>()
                        .concat()
                }
            }
        }
        Node::Or(n1, n2) => {
            let c1 = build_regex(None, &*n1, nodes);
            let c2 = build_regex(None, &*n2, nodes);
            format!("({}|{})", c1, c2)
        }
        Node::String(s) => s.into(),
    }
}

fn main() {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let elements = message_grammar::ElementsParser::new()
        .parse(&semicolonized_input())
        .unwrap();
    for element in elements.iter() {
        println!("element: {:?}", element);

        match element {
            Element::Rule(i, node) => {
                rules.insert(*i, node.clone());
            }
            Element::Message(msg) => messages.push(msg),
        }
    }

    // offload actual matching to Regex
    let regex = build_regex(Some(0), &rules.get(&0).unwrap(), &rules);
    let regex = format!("^{}$", regex);
    let rule0 = Regex::new(&regex[..]).unwrap();
    println!("{:?} -> {}", rules.get(&0).unwrap(), regex);
    let num = messages.iter().filter(|msg| rule0.is_match(msg)).count();
    println!("num: {}", num);
}
