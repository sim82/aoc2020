#![feature(linked_list_cursors)]
use std::collections::LinkedList;

fn print_list(list: &[usize]) {
    let mut cur = list[0];

    loop {
        print!("{}", cur);
        cur = list[cur];
        if cur == list[0] {
            println!("");
            break;
        }
    }
}
fn main() {
    let input = "685974213";
    // let input = "389125467";

    let mut list = vec![0usize; 1000001];
    let mut last = 0;
    let mut max = 0;
    let extend = 1000000;
    for i in input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .chain(10..=extend)
    {
        list[last] = i;
        last = i;
        max = max.max(i);
    }

    list[last] = list[0];

    // print_list(&list);
    let n = 10000000;
    for _ in 0..n {
        let cur = list[0]; // head
        let rs = list[cur]; // first of three elements to be removed
        let rm = list[rs];
        let re = list[rm]; // last of three elements to be removed
        let sn = list[re]; // first of remaining list

        // println!("rs: {} re: {} sn: {}", rs, re, sn);

        // find new insertion position
        let mut ins = cur - 1;
        if ins == 0 {
            ins = max;
        }
        while [rs, rm, re].contains(&ins) {
            ins -= 1;
            if ins == 0 {
                ins = max;
            }
        }
        // re-twiddle list
        list[0] = sn; // new head of list
        list[cur] = sn; // old head goes to end => points to new head (keep list circular)

        // splice in 'removed' after ins (to form ins -> rs -> rm -> re -> ins+1)
        list[re] = list[ins];
        list[ins] = rs;
        // print_list(&list);
    }
    println!(
        "1: {} {} {}",
        list[1],
        list[list[1]],
        list[1] * list[list[1]]
    );
}
