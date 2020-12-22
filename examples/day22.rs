use std::collections::{HashSet, VecDeque};
fn play(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
    s: Vec<usize>,
) -> (usize, usize, usize) {
    // println!("game {:?}: {:?} {:?}", s, player1, player2);

    let mut prev = HashSet::new();

    let mut round = *s.last().unwrap();
    while !player1.is_empty() && !player2.is_empty() {
        round += 1;

        let t = (player1.clone(), player2.clone());
        if prev.contains(&t) {
            // println!("emergency brake");
            return (1, 0, round);
        }
        prev.insert(t);

        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        let player1_wins = if player1.len() >= c1 && player2.len() >= c2 {
            // sub game
            let mut s = s.clone();
            s.push(round);
            let (score1, score2, end) = play(
                player1.iter().take(c1).cloned().collect(),
                player2.iter().take(c2).cloned().collect(),
                s,
            );
            round = end;

            score1 > score2
        } else {
            c1 > c2
        };

        if player1_wins {
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

    (score1, score2, round)
}

fn main() {
    // let input1 = [9, 2, 6, 3, 1];
    // let input2 = [5, 8, 4, 7, 10];

    let input1 = vec![
        5, 20, 28, 30, 48, 7, 41, 24, 29, 8, 37, 32, 16, 17, 34, 27, 46, 43, 14, 49, 35, 11, 6, 38,
        1,
    ];
    let input2 = vec![
        22, 18, 50, 31, 12, 13, 33, 39, 45, 21, 19, 26, 44, 10, 42, 3, 4, 15, 36, 2, 40, 47, 9, 23,
        25,
    ];
    let (score1, score2, _) = play(input1.into(), input2.into(), [0].into());

    println!("p1: {}", score1);
    println!("p2: {}", score2);
}
