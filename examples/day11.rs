use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let lines: Vec<_> = std::io::stdin().lock().lines().collect();
    let mut seats: HashMap<(i32, i32), bool> = lines
        .iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            let line = line.as_ref().unwrap();
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == 'L' {
                    Some(((x as _, y as _), false))
                } else {
                    None
                }
            })
        })
        .collect();
    part1(seats.clone());
    part2(seats.clone());
}
fn part1(mut seats: HashMap<(i32, i32), bool>) {
    // print_seats(&seats);
    // println!("seats: {:?}", seats);
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    loop {
        let mut new_seats = seats.clone();
        for ((x, y), state) in seats.iter() {
            // for (nx, ny) in neighbors.iter() {

            // }
            *new_seats.get_mut(&(*x, *y)).unwrap() = if !state {
                !neighbors
                    .iter()
                    .any(|(nx, ny)| seats.get(&(x + nx, y + ny)).cloned().unwrap_or(false))
            } else {
                neighbors
                    .iter()
                    .filter(|(nx, ny)| seats.get(&(x + nx, y + ny)).cloned().unwrap_or(false))
                    .count()
                    < 4
            };
        }

        if new_seats == seats {
            println!("stable");
            break;
        }
        seats = new_seats;
        // print_seats(&seats);
        // println!("\n\n");
    }

    // print_seats(&seats);

    println!("occupied: {}", seats.values().filter(|v| **v).count());
}

fn part2(mut seats: HashMap<(i32, i32), bool>) {
    let maxx = seats.keys().map(|(x, _)| *x).max().unwrap();
    let maxy = seats.keys().map(|(_, y)| *y).max().unwrap();
    // print_seats(&seats);
    // println!("seats: {:?}", seats);
    let neighbors = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    loop {
        let mut new_seats = seats.clone();
        for ((x, y), state) in seats.iter() {
            let mut visible = 0;
            for (nx, ny) in neighbors.iter() {
                for i in 1.. {
                    let nx = i * *nx;
                    let ny = i * *ny;

                    let x = x + (nx * 1);
                    let y = y + (ny * 1);
                    if x < 0 || y < 0 || x > maxx || y > maxy {
                        break;
                    }
                    match seats.get(&(x, y)) {
                        Some(true) => {
                            visible += 1;
                            break;
                        }
                        Some(false) => break,
                        _ => (),
                    }
                }
            }
            *new_seats.get_mut(&(*x, *y)).unwrap() =
                if !state { visible == 0 } else { visible < 5 };
        }

        // print_seats(&seats);
        // println!("\n\n");
        if new_seats == seats {
            println!("stable");
            break;
        }
        seats = new_seats;
    }

    // print_seats(&seats);

    println!("occupied: {}", seats.values().filter(|v| **v).count());
}

fn print_seats(seats: &HashMap<(i32, i32), bool>) -> () {
    let maxx = seats.keys().map(|(x, _)| *x).max().unwrap();
    let maxy = seats.keys().map(|(_, y)| *y).max().unwrap();

    for y in 0..=maxy {
        let line: String = (0..=maxx)
            .map(|x| match seats.get(&(x, y)) {
                Some(true) => '#',
                Some(false) => 'L',
                None => '.',
            })
            .collect();
        println!("{}", line);
    }
}
