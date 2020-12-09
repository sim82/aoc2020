use std::io::BufRead;

fn pairs(first: usize, last: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
    Box::new((first..last).flat_map(move |i| (i..last).map(move |j| (i, j))))
}

fn main() {
    const N: usize = 25;

    let input: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect();

    let mut not_found = 0;

    for start in N..(input.len() - N) {
        let pre = &input[(start - N)..start];
        if pairs(0, N)
            .find(|(i, j)| pre[*i] + pre[*j] == input[start])
            .is_none()
        {
            println!("not found: {}", input[start]);
            not_found = input[start];
            break;
        }
    }

    for i in 0..input.len() {
        for j in i..input.len() {
            let range = &input[i..j];
            if range.iter().sum::<usize>() == not_found {
                println!("range: {} {} {:?}", i, j, &input[i..j]);
                println!(
                    "res: {}",
                    range.iter().min().unwrap() + range.iter().max().unwrap()
                );
            }
        }
    }

    // or more complicated
    let l = input.len();
    let res = pairs(0, l)
        .filter_map(|(i, j)| {
            let range = &input[i..j];
            if range.iter().sum::<usize>() == not_found {
                Some(range.iter().min().unwrap() + range.iter().max().unwrap())
            } else {
                None
            }
        })
        .next()
        .unwrap();

    println!("res: {}", res);
}
