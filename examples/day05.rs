use std::{collections::HashSet, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let rowbin = |c| match c {
        'B' => 1,
        _ => 0,
    };
    let colbin = |c| match c {
        'R' => 1,
        _ => 0,
    };
    let allseats: HashSet<_> = input
        .lines()
        .map(|line| {
            let row = line
                .chars()
                .take(7)
                .map(rowbin)
                .enumerate()
                .map(|(i, c)| (c << (6 - i)))
                .sum::<i32>();
            let column = line
                .chars()
                .skip(7)
                .map(colbin)
                .enumerate()
                .map(|(i, c)| (c << (2 - i)))
                .sum::<i32>();

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
