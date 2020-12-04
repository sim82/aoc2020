use aoc2020::passport_simple_grammar;
use aoc2020::{
    passport::{Entry, LenUnit},
    passport_grammar,
};
use std::{collections::HashSet, io::Read};

trait Validator {
    fn valid(&self) -> bool;
}

//    byr (Birth Year) - four digits; at least 1920 and at most 2002.
//    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//    hgt (Height) - a number followed by either cm or in:
//        If cm, the number must be at least 150 and at most 193.
//        If in, the number must be at least 59 and at most 76.
//    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//    pid (Passport ID) - a nine-digit number, including leading zeroes.
//    cid (Country ID) - ignored, missing or not.

impl Validator for Entry {
    fn valid(&self) -> bool {
        match self {
            Entry::BYr(y) => (1920..=2002).contains(y),
            Entry::IYr(y) => (2010..=2020).contains(y),
            Entry::EYr(y) => (2020..=2030).contains(y),
            Entry::Hgt(h, LenUnit::Cm) => (150..=193).contains(h),
            Entry::Hgt(h, LenUnit::In) => (59..=76).contains(h),
            _ => true,
        }
    }
}

fn entry_id(e: &Entry) -> usize {
    match e {
        Entry::BYr(_) => 1,
        Entry::IYr(_) => 2,
        Entry::EYr(_) => 3,
        Entry::Hgt(_, _) => 4,
        Entry::Hcl => 5,
        Entry::Ecl => 6,
        Entry::Pid => 7,
        Entry::Cid => 8,
    }
}

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();

    let entries = code.split("\n\n");
    let mut valid = 0;

    // cid
    for e in entries {
        let mandatory = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        // println!("==========\n{}", e);
        let passport = passport_simple_grammar::PassportParser::new()
            .parse(&e[..])
            .unwrap();

        // println!("passport: {:?}", passport);
        let fields: HashSet<_> = passport.iter().map(|e| e.0.clone()).collect();
        if mandatory.iter().all(|m| fields.contains(*m)) {
            valid += 1;
        }
    }
    println!("valid1: {}", valid);

    let entries = code.split("\n\n");
    let mut valid = 0;

    for e in entries {
        println!("==========\n{}", e);
        let passport = passport_grammar::PassportParser::new().parse(&e[..]);

        if passport.is_err() {
            println!("failed to parse: {:?}", passport);
            continue;
        }
        let passport = passport.unwrap();

        println!("passport: {:?}", passport);
        if !passport.iter().all(|e| e.valid()) {
            println!("invalid field");
            continue;
        }
        let mandatory = [1, 2, 3, 4, 5, 6, 7];
        let entry_ids: HashSet<_> = passport.iter().map(|e| entry_id(e)).collect();
        if !mandatory.iter().all(|m| entry_ids.contains(m)) {
            println!("incomplete");
            continue;
        }

        valid += 1;
        // let fields: HashSet<_> = passport.iter().map(|e| e.0.clone()).collect();
        // if mandatory.iter().all(|m| fields.contains(*m)) {
        //     valid += 1;
        // }
    }
    println!("valid2: {}", valid);
}
