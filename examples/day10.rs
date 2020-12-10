use std::io::BufRead;
fn main() {
    // part one solved in libreoffice...

    let input = {
        let mut input = vec![0usize; 1];
        input.extend(
            std::io::stdin()
                .lock()
                .lines()
                .map(|l| l.unwrap().parse::<usize>().unwrap()),
        );
        input.sort();
        input.push(input.last().unwrap() + 3);
        input
    };

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
