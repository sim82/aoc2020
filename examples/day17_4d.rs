use aoc2020::math::*;
use std::{collections::HashSet, hash::Hash, io::BufRead, iter::Extend, ops::RangeInclusive};

fn neighbors_3d(v: &Vec3) -> HashSet<Vec3> {
    let mut res = HashSet::new();
    res.reserve(26);
    for z in [-1, 0, 1].iter().cloned() {
        for y in [-1, 0, 1].iter().cloned() {
            for x in [-1, 0, 1].iter().cloned() {
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
    for w in [-1, 0, 1].iter().cloned() {
        for z in [-1, 0, 1].iter().cloned() {
            for y in [-1, 0, 1].iter().cloned() {
                for x in [-1, 0, 1].iter().cloned() {
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

fn simulate<T, F>(
    mut domain: HashSet<T>,
    get_neighbors: F,
    max: i32,
    spawn: RangeInclusive<usize>,
    survive: RangeInclusive<usize>,
) where
    T: Eq + Hash + Clone + Copy + PartialEq,
    F: Fn(&T) -> HashSet<T>,
{
    for i in 1..=max {
        let mut new_domain = HashSet::<T>::new();
        let mut candidates = HashSet::<T>::new();
        for v in domain.iter() {
            let neighbors = get_neighbors(v);
            candidates.extend(neighbors.difference(&domain));

            let active = neighbors.intersection(&domain).count();
            if survive.contains(&active) {
                new_domain.insert(*v);
            }
        }

        for v in candidates.iter() {
            let neighbors = get_neighbors(v);
            let active = neighbors.intersection(&domain).count();
            if spawn.contains(&active) {
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
    println!("3d");
    simulate::<Vec3, _>(domain_3d, neighbors_3d, 6, 3..=3, 2..=3);
    println!("4d");
    simulate::<Vec4, _>(domain_4d, neighbors_4d, 6, 3..=3, 2..=3);
}
