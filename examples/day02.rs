use aoc2020::password_grammar;
use std::io::Read;

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();

    // using lalrpop grammar to parse input, because - why not?
    let passwords = password_grammar::LinesParser::new()
        .parse(&code[..])
        .unwrap();

    let mut num_valid = 0;
    let mut num_valid2 = 0;
    for ((low, high, char), password) in passwords.iter() {
        let char = char.chars().nth(0).unwrap();
        println!("{} - {} {} {}", low, high, char, password);

        let num: i64 = password
            .chars()
            .map(|c| if c == char { 1 } else { 0 })
            .sum();

        if num >= *low && num <= *high {
            num_valid += 1;
        }
        let p1 = password.chars().nth((low - 1) as usize).unwrap() == char;
        let p2 = password.chars().nth((high - 1) as usize).unwrap() == char;
        if (p1 || p2) && p1 != p2 {
            num_valid2 += 1;
        }
    }
    println!("valid: {} {}", num_valid, num_valid2);
}
