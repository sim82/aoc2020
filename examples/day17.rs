use aoc2020::math::*;
use std::{collections::HashSet, io::BufRead, iter::Extend};

fn neighbors(v: &Vec3) -> [Vec3; 26] {
    let mut res = [*v; 26];
    let mut i = 0;
    for z in [-1, 0, 1].iter().cloned() {
        for y in [-1, 0, 1].iter().cloned() {
            for x in [-1, 0, 1].iter().cloned() {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                res[i] = *v + Vec3(x, y, z);
                i += 1;
            }
        }
    }
    res
}

type Domain = HashSet<Vec3>;
fn main() {
    let mut domain = Domain::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        domain.extend(line.unwrap().chars().enumerate().filter_map(|(x, c)| {
            if c == '#' {
                Some(Vec3(x as i32, y as i32, 0))
            } else {
                None
            }
        }));
    }

    println!("neighbors: {:?}", neighbors(&Vec3(0, 0, 0)));

    draw_zslice(&domain, 0);

    for _ in 0..100 {
        let mut new_domain = Domain::new();
        new_domain.reserve(domain.len());
        let mut candidates = Domain::new();
        for v in domain.iter() {
            let neighbors = neighbors(v).iter().cloned().collect::<Domain>();
            candidates.extend(neighbors.difference(&domain));

            let active = neighbors.intersection(&domain).count();
            if active == 2 || active == 3 {
                new_domain.insert(*v);
            }
        }

        for v in candidates.iter() {
            let neighbors = neighbors(v).iter().cloned().collect::<Domain>();
            let active = neighbors.intersection(&domain).count();
            if active == 3 {
                new_domain.insert(*v);
            }
        }

        if false {
            println!("domain: {:?}", domain);
            draw_zslice(&domain, -1);
            draw_zslice(&domain, 0);
            draw_zslice(&domain, 1);
            println!("candidates: {:?}", candidates);
            draw_zslice(&candidates, -1);
            draw_zslice(&candidates, 0);
            draw_zslice(&candidates, 1);
        }
        domain = new_domain;
        println!("active: {}", domain.len());
    }
}

fn draw_zslice(domain: &Domain, z: i32) {
    let xmin = domain.iter().map(|v| v.x()).min().unwrap();
    let xmax = domain.iter().map(|v| v.x()).max().unwrap();
    let ymin = domain.iter().map(|v| v.y()).min().unwrap();
    let ymax = domain.iter().map(|v| v.y()).max().unwrap();

    for y in ymin..=ymax {
        let mut line = String::new();
        for x in xmin..=xmax {
            if domain.contains(&Vec3(x, y, z)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}
