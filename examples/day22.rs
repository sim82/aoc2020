use std::collections::{HashSet, VecDeque};
type Card = u32; // slight perfromance gain (due to HashSet)
fn play(
    mut player1: VecDeque<Card>,
    mut player2: VecDeque<Card>,
    s: Vec<usize>,
) -> (usize, usize, usize) {
    // println!("game {:?}: {:?} {:?}", s, player1, player2);

    //let prev: HashSet<(VecDeque<usize>, VecDeque<usize>)> = std::iter::once(t).collect();

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

        if player1.len() >= c1 as usize && player2.len() >= c2 as usize {
            // sub game

            let mut s = s.clone();
            s.push(round);
            let (score1, score2, end) = play(
                player1.iter().take(c1 as usize).cloned().collect(),
                player2.iter().take(c2 as usize).cloned().collect(),
                s,
            );

            if score1 > score2 {
                player1.push_back(c1);
                player1.push_back(c2);
            } else {
                player2.push_back(c2);
                player2.push_back(c1);
            }
            round = end;
            // println!(
            //     "after rec: {:?} {:?} {} {}",
            //     player1, player2, score1, score2
            // );
        } else {
            if c1 > c2 {
                player1.push_back(c1);
                player1.push_back(c2);
            } else {
                player2.push_back(c2);
                player2.push_back(c1);
            }
        }
    }

    let score1: usize = player1
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c as usize)
        .sum();
    let score2: usize = player2
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * *c as usize)
        .sum();

    (score1, score2, round)
}

fn main() {
    // let input1 = [9, 2, 6, 3, 1];
    // let input2 = [5, 8, 4, 7, 10];

    let input1 = [
        5, 20, 28, 30, 48, 7, 41, 24, 29, 8, 37, 32, 16, 17, 34, 27, 46, 43, 14, 49, 35, 11, 6, 38,
        1,
    ];
    let input2 = [
        22, 18, 50, 31, 12, 13, 33, 39, 45, 21, 19, 26, 44, 10, 42, 3, 4, 15, 36, 2, 40, 47, 9, 23,
        25,
    ];

    // let mut player1: VecDeque<usize> = player1.iter().cloned().collect();
    // let mut player2: VecDeque<usize> = player2.iter().cloned().collect();

    let (score1, score2, _) = play(
        input1.iter().cloned().collect(),
        input2.iter().cloned().collect(),
        [0].into(),
    );

    println!("p1: {}", score1);
    println!("p2: {}", score2);
}
