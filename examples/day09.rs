use std::io::BufRead;

fn main() {
    const N: usize = 25;

    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    let mut not_found = 0;

    for start in N..(input.len() - N) {
        let pre = &input[(start - N)..start];
        let mut found = false;
        for i in 0..N {
            for j in i..N {
                if input[start] == pre[i] + pre[j] {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            println!("not found: {}", input[start]);
            not_found = input[start];
            break;
        }
    }

    for i in 0..input.len() {
        for j in i..input.len() {
            let range = &input[i..j];
            if range.iter().sum::<i64>() == not_found {
                println!("range: {} {} {:?}", i, j, &input[i..j]);
                println!(
                    "res: {}",
                    range.iter().min().unwrap() + range.iter().max().unwrap()
                );
            }
        }
    }
}
