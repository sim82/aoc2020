use std::iter::once;
fn main() {
    // part one solved in libreoffice...

    let mut input = aoc2020::map_input_vec(|l| l.parse::<usize>().unwrap());
    input.sort();
    // add 0 and last+3 at start and end
    let input = once(0)
        .chain(input.iter().cloned())
        .chain(once(input.last().unwrap() + 3))
        .collect::<Vec<_>>();

    println!("{:?}", input);
    // dynamic programming:
    // buf[i] is the number of possible combinations for the first i adapters
    let mut buf = vec![1usize; input.len()];
    for i in 1..input.len() {
        let mut sum = buf[i - 1];
        for j in [2, 3].iter() {
            if *j > i {
                break;
            }
            // runs of 2 or 3 consecutive elements sum up number of possible combinations (by skipping adapters in between)
            if input[i - j] == input[i] - j {
                sum += buf[i - j];
            }
        }
        buf[i] = sum;
    }

    println!("sum: {:?}", buf);
}
