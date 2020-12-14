use aoc2020::bitmask::Op;
use aoc2020::bitmask_grammar;
use std::{collections::HashMap, io::Read};

fn addr_gen(addr: u64, vs: &[u64]) -> Vec<u64> {
    let v = vs[0];
    if vs.len() == 1 {
        return vec![addr | (0b1u64 << v), addr & !(0b1u64 << v)];
    }

    let rest = addr_gen(addr, &vs[1..]);
    rest.iter()
        .map(|r| *r | (0b1 << v))
        .chain(rest.iter().map(|r| *r & !(0b1 << v)))
        .collect()
}

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();

    let ops: Vec<Op> = bitmask_grammar::ProgramParser::new()
        .parse(&code[..])
        .unwrap();

    println!("ops: {:?}", ops);

    let mut and_mask = !0u64;
    let mut or_mask = 0u64;
    let mut floating = Vec::<u64>::new();
    let mut mem: HashMap<u64, u64> = HashMap::new(); // Address Space Compression Technology (pat. pend.), fancy...
    let mut memv2: HashMap<u64, u64> = HashMap::new();

    for op in ops.iter() {
        match op {
            Op::Mask(mask) => {
                and_mask = !mask
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| if c == '0' { 0b1 << i } else { 0 })
                    .sum::<u64>();
                or_mask = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| if c == '1' { 0b1 << i } else { 0 })
                    .sum::<u64>();

                floating = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter_map(|(i, c)| if c == 'X' { Some(i as u64) } else { None })
                    .collect();
                println!("mask {} -> {:b} {:b}", mask, and_mask, or_mask);
                println!("floating: {:?}", floating);
            }
            Op::Mem(addr, value) => {
                // chip v1
                mem.insert(*addr, (value & and_mask) | or_mask);

                // chip v2
                let addr = (*addr as u64) | or_mask;
                for addr in addr_gen(addr, &floating[..]) {
                    // println!("addr: {:36b}", addr);
                    memv2.insert(addr, *value);
                }
            }
        }
    }
    println!("res: {}", mem.values().sum::<u64>());
    println!("res2: {}", memv2.values().sum::<u64>());
}
