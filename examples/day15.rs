use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

fn main() {
    let input = [18, 11, 9, 0, 5, 1];
    // let input = [3, 1, 2];
    let mut last_turn_for_num = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, v)| (*v, i as u32))
        .collect::<HashMap<_, _>>();

    let mut last = *input.last().unwrap();
    let start = (input.len() - 1) as u32;
    for last_turn in start..30000000u32 {
        let d = match last_turn_for_num.entry(last) {
            Occupied(mut x) => {
                let old_value = *x.get();
                x.insert(last_turn);
                last_turn - old_value // last was said in 'old_value' => distance between last_turn and old_value
            }
            Vacant(x) => {
                x.insert(last_turn);
                0 // last was not said before => 0
            }
        };
        // counting is a bit weird: the loop index is the 'last turn' and indices are 0-based
        // -> since the turns for the solution are 1-based, we need to add 2 to get the
        // 1-based index of the current turn, sigh, I hate indices...
        if last_turn == 2020 - 2 || last_turn == 30000000 - 2 {
            println!("turn {} {}", last_turn + 2, d);
        }
        last = d;
    }
}
