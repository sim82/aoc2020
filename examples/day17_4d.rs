use aoc2020::math::*;
use std::{collections::HashSet, hash::Hash, io::BufRead, iter::Extend, ops::RangeInclusive};

fn neighbors_3d(v: &Vec3) -> HashSet<Vec3> {
    let mut res = HashSet::new();
    res.reserve(26);
    for z in -1..=1 {
        for y in -1..=1 {
            for x in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                res.insert(*v + Vec3(x, y, z));
            }
        }
    }
    res
}

fn neighbors_4d(v: &Vec4) -> HashSet<Vec4> {
    let mut res = HashSet::new();
    res.reserve(80);
    for w in -1..=1 {
        for z in -1..=1 {
            for y in -1..=1 {
                for x in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    res.insert(*v + Vec4(x, y, z, w));
                }
            }
        }
    }
    res
}

fn simulate<T, F, R>(mut domain: HashSet<T>, get_neighbors: F, max: i32, rules: R)
where
    T: Eq + Hash + Clone + Copy + PartialEq,
    F: Fn(&T) -> HashSet<T>,
    R: Fn(bool, usize) -> bool,
{
    for i in 1..=max {
        let mut new_domain = HashSet::<T>::new();
        let mut candidates = HashSet::<T>::new();
        for v in domain.iter() {
            let neighbors = get_neighbors(v);
            let active = neighbors.intersection(&domain).count();

            if rules(true, active) {
                new_domain.insert(*v);
            }
            candidates.extend(neighbors);
        }

        for v in candidates.difference(&domain) {
            let neighbors = get_neighbors(v);
            let active = neighbors.intersection(&domain).count();
            if rules(false, active) {
                new_domain.insert(*v);
            }
        }

        domain = new_domain;
        // std::mem::swap(&mut domain, &mut new_domain);
        // new_domain.clear();
        println!("active: {} {}", i, domain.len());
    }
}

fn main() {
    let mut domain_3d = HashSet::<Vec3>::new();
    let mut domain_4d = HashSet::<Vec4>::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        domain_3d.extend(line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some(Vec3(x as i32, y as i32, 0))
            } else {
                None
            }
        }));
        domain_4d.extend(line.chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some(Vec4(x as i32, y as i32, 0, 0))
            } else {
                None
            }
        }));
    }
    let rules = |alive: bool, neighbors: usize| match alive {
        false => (3..=3).contains(&neighbors),
        true => (2..=3).contains(&neighbors),
    };
    println!("3d");
    simulate::<Vec3, _, _>(domain_3d, neighbors_3d, 6, rules);
    println!("4d");
    simulate::<Vec4, _, _>(domain_4d, neighbors_4d, 12, rules);
}
