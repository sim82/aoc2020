#![feature(iterator_fold_self)]
#![feature(vec_remove_item)]
use std::{collections::HashSet, io::Read};

use aoc2020::ingredients_grammar;
use multimap::MultiMap;

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();
    let input: Vec<(Vec<String>, Vec<String>)> = ingredients_grammar::ListParser::new()
        .parse(&code[..])
        .unwrap();

    let all_ingredients = input
        .iter()
        .flat_map(|(ing, all)| ing.iter())
        .cloned()
        .collect::<HashSet<_>>();

    let all_allergens = input
        .iter()
        .flat_map(|(ing, all)| all.iter())
        .cloned()
        .collect::<HashSet<_>>();

    let mut have_allergens = HashSet::new();
    let mut potential_allergens = MultiMap::new();
    for a in all_allergens.iter() {
        let ingredients = input
            .iter()
            .filter_map(|(ing, all)| {
                if all.contains(a) {
                    Some(ing.iter().cloned().collect::<HashSet<_>>())
                } else {
                    None
                }
            })
            .fold_first(|a, set| a.intersection(&set).cloned().collect())
            .unwrap();
        println!("{} -> {:?}", a, ingredients);
        for i in ingredients.iter() {
            potential_allergens.insert(i.clone(), a.clone());
        }
        have_allergens.extend(ingredients);
    }
    println!(
        "no allergens: {:?}",
        all_ingredients.difference(&have_allergens)
    );

    let res = input
        .iter()
        .flat_map(|(ing, _)| ing.iter())
        .filter(|ing| !have_allergens.contains(*ing))
        .count();
    println!("count: {}", res);

    println!("pot: {:?}", potential_allergens);

    let mut allergen_map = Vec::new();
    while !potential_allergens.is_empty() {
        let nexti = potential_allergens
            .iter_all()
            .find(|(_, a)| a.len() == 1)
            .unwrap()
            .0
            .clone();
        let nexta = potential_allergens.remove(&nexti).unwrap()[0].clone();
        for (_, a) in potential_allergens.iter_all_mut() {
            //a.remove_item(&nexta); // not so sure that it is really that 'easily possible with iterators and the current Vec methods'... bit of a PITA:
            if let Some(pos) = a.iter().position(|x| *x == *nexta) {
                a.remove(pos);
            }
        }
        allergen_map.push((nexti, nexta));
    }
    allergen_map.sort_by_key(|(_, a)| a.clone());
    let dil = allergen_map
        .iter()
        .map(|(i, _)| i.clone())
        .collect::<Vec<_>>();
    println!("dangerous ingredients: {:?}", dil.join(","));
}
