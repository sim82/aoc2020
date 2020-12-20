use std::collections::BTreeMap;

fn reverse10(v: u16) -> u16 {
    (0..10).map(|i| ((v >> (9 - i)) & 1) << i).sum::<u16>()
}

fn r10id(v: u16) -> u16 {
    v.min(reverse10(v))
}

fn main() {
    let mut ns = (0..=0b1111111111)
        .map(|v| (r10id(v), v))
        .collect::<Vec<_>>();

    ns.sort_by_key(|(k, v)| *k);
    for (k, v) in ns.iter() {
        println!("{:010b} -> {:010b}", v, k);
    }
}
