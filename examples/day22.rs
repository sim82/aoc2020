use std::collections::VecDeque;

fn main() {
    let input1 = [
        5, 20, 28, 30, 48, 7, 41, 24, 29, 8, 37, 32, 16, 17, 34, 27, 46, 43, 14, 49, 35, 11, 6, 38,
        1,
    ];
    let input2 = [
        22, 18, 50, 31, 12, 13, 33, 39, 45, 21, 19, 26, 44, 10, 42, 3, 4, 15, 36, 2, 40, 47, 9, 23,
        25,
    ];

    // [9, 2, 6, 3, 1]
    // [5, 8, 4, 7, 10]
    let mut player1: VecDeque<usize> = input1.iter().cloned().collect();
    let mut player2: VecDeque<usize> = input2.iter().cloned().collect();

    while !player1.is_empty() && !player2.is_empty() {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        if c1 > c2 {
            player1.push_back(c1);
            player1.push_back(c2);
        } else {
            player2.push_back(c2);
            player2.push_back(c1);
        }
    }

    let score1: usize = player1
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c)
        .sum();
    let score2: usize = player2
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c)
        .sum();
    println!("p1: {:?} {}", player1, score1);
    println!("p2: {:?} {}", player2, score2);
}
