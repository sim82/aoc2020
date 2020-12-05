use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let rowbin = |c| match c {
        'F' => 0,
        'B' => 1,
        _ => panic!("invalid"),
    };
    let colbin = |c| match c {
        'R' => 1,
        'L' => 0,
        _ => panic!("invalid"),
    };
    let allseats: HashSet<_> = input
        .lines()
        .map(|line| {
            let row = line
                .chars()
                .take(7)
                .map(rowbin)
                .enumerate()
                .fold(0, |acc, (i, c)| acc | (c << (6 - i)));
            let column = line
                .chars()
                .skip(7)
                .take(3)
                .map(colbin)
                .enumerate()
                .fold(0, |acc, (i, c)| acc | (c << (2 - i)));

            row * 8 + column
        })
        .collect();
    let maxid = *allseats.iter().max().unwrap();
    println!("max: {}", maxid);

    for i in 1..maxid {
        if !allseats.contains(&i) && allseats.contains(&(i - 1)) && allseats.contains(&(i + 1)) {
            println!("missing: {}", i);
        }
    }
}
