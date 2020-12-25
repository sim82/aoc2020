use std::{collections::HashSet, io::BufRead};

use aoc2020::math::Vec2;

fn adjacent(v: &Vec2) -> Vec<Vec2> {
    let xshift = v.y().abs() % 2;
    let mut d = vec![
        Vec2(1, 0),
        Vec2(-1, 0),
        Vec2(-1 + xshift, 1),
        Vec2(0 + xshift, 1),
        Vec2(-1 + xshift, -1),
        Vec2(0 + xshift, -1),
    ];

    d.iter_mut().for_each(|f| *f = *f + *v);
    d
}
#[test]
fn test_hex() {
    println!("{:?}", adjacent(&Vec2(0, 0)));
    println!("{:?}", adjacent(&Vec2(0, 1)));
    println!("{:?}", adjacent(&Vec2(0, -1)));
}
fn main() {
    let mut black = HashSet::new();
    for line in std::io::stdin().lock().lines().map(|l| l.unwrap()) {
        let mut c = line.chars();
        let mut x = 0i32;
        let mut y = 0i32;

        loop {
            match c.next() {
                Some('e') => x += 1,
                Some('w') => x -= 1,
                Some('s') => match c.next() {
                    Some('e') => {
                        x += (y.abs() % 2);
                        y += 1
                    }
                    Some('w') => {
                        y += 1;
                        x -= (y.abs() % 2);
                    }
                    _ => panic!("unhandled"),
                },
                Some('n') => match c.next() {
                    Some('e') => {
                        x += (y.abs() % 2);
                        y -= 1
                    }
                    Some('w') => {
                        y -= 1;
                        x -= y.abs() % 2;
                    }
                    _ => panic!("unhandled"),
                },
                None => break,

                _ => panic!("unhandled"),
            }
        }
        if black.contains(&Vec2(x, y)) {
            black.remove(&Vec2(x, y));
        } else {
            black.insert(Vec2(x, y));
        }
        println!("{} {}", x, y);
    }
    println!("len: {}", black.len());

    for i in 1..=200 {
        let mut black_new = HashSet::new();

        for v in black.iter() {
            let n = adjacent(v).iter().filter(|v| black.contains(*v)).count();
            if (1..=2).contains(&n) {
                black_new.insert(*v);
            }
        }

        let white = black
            .iter()
            .flat_map(|v| adjacent(v))
            .collect::<HashSet<_>>();

        let white = white.difference(&black).cloned().collect::<Vec<_>>();
        for v in white.iter() {
            let n = adjacent(v).iter().filter(|v| black.contains(*v)).count();
            if n == 2 {
                black_new.insert(*v);
            }
        }
        black = black_new;
        println!("day: {} {}", i, black.len());
    }
}
