use aoc2020::math::*;
use std::ops;

fn right(dir: char, v: i32) -> char {
    if v % 90 != 0 {
        panic!("not 90deg");
    }
    let q = (v / 90) % 4;
    match (dir, q) {
        ('N', 0) | ('W', 1) | ('S', 2) | ('E', 3) => 'N',
        ('E', 0) | ('N', 1) | ('W', 2) | ('S', 3) => 'E',
        ('S', 0) | ('E', 1) | ('N', 2) | ('W', 3) => 'S',
        ('W', 0) | ('S', 1) | ('E', 2) | ('N', 3) => 'W',
        _ => panic!("bad rotation"),
    }
}
fn left(dir: char, v: i32) -> char {
    if v % 90 != 0 {
        panic!("not 90deg");
    }
    let q = (v / 90) % 4;
    match (dir, q) {
        ('N', 0) | ('E', 1) | ('S', 2) | ('W', 3) => 'N',
        ('E', 0) | ('S', 1) | ('W', 2) | ('N', 3) => 'E',
        ('S', 0) | ('W', 1) | ('N', 2) | ('E', 3) => 'S',
        ('W', 0) | ('N', 1) | ('E', 2) | ('S', 3) => 'W',
        _ => panic!("bad rotation"),
    }
}
fn rotate_right(mut a: Vec2, v: i32) -> Vec2 {
    if v % 90 != 0 {
        panic!("not 90deg");
    }
    let q = (v / 90) % 4;
    for _ in 0..q {
        // matrix:
        // x     0    -1
        // y     1     0
        a = Vec2(a.1, -a.0);
    }
    a
}

fn rotate_left(mut a: Vec2, v: i32) -> Vec2 {
    if v % 90 != 0 {
        panic!("not 90deg");
    }
    let q = (v / 90) % 4;

    for _ in 0..q {
        // matrix:
        // x      0    1
        // y     -1    0
        a = Vec2(-a.1, a.0);
    }
    a
}

fn main() {
    let input = aoc2020::map_input_vec(|l| {
        (
            l.chars().next().unwrap(),
            l.chars()
                .skip(1)
                .collect::<String>()
                .parse::<i32>()
                .unwrap(),
        )
    });
    {
        // part 1
        let mut dir = 'E';
        let mut ship = Vec2(0, 0);
        println!("{:?}", input);
        for (c, v) in input.iter().cloned() {
            match c {
                'F' => ship += Vec2::from(dir) * v,
                'R' => dir = right(dir, v),
                'L' => dir = left(dir, v),
                'N' | 'S' | 'E' | 'W' => ship += Vec2::from(c) * v,
                _ => panic!("bad command"),
            }
            // println!("{:?} {}", ship, dir);
        }
        println!("{:?}", ship);
        println!("dist: {}", ship.manhattan());
    }
    // part 2
    let mut ship = Vec2(0, 0);
    let mut waypoint = Vec2(10, 1);
    // println!("{:?}", input);
    for (c, v) in input {
        match c {
            'F' => ship += waypoint * v,
            'R' => waypoint = rotate_right(waypoint, v),
            'L' => waypoint = rotate_left(waypoint, v),
            'N' | 'S' | 'E' | 'W' => waypoint += Vec2::from(c) * v,
            _ => panic!("bad command"),
        }
        // println!("{:?} {:?}", ship, waypoint);
    }
    println!("{:?}", ship);
    println!("dist: {}", ship.manhattan());
}
