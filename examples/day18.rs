use aoc2020::badmath::Term;
use aoc2020::badmath_grammar;
use std::io::Read;

fn eval(t: &Term) -> i64 {
    match t {
        Term::Num(n) => *n,
        Term::Op(t1, '+', t2) => eval(t1) + eval(t2),
        Term::Op(t1, '*', t2) => eval(t1) * eval(t2),
        _ => panic!("unhandled"),
    }
}
fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();

    let terms: Vec<Term> = badmath_grammar::TermsParser::new()
        .parse(&code[..])
        .unwrap();

    println!("{:?}", terms);
    for term in terms.iter() {
        println!("term: {:?} -> {}", term, eval(term));
    }

    println!("sum: {}", terms.iter().map(|t| eval(t)).sum::<i64>());
}
