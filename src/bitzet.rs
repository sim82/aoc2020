use crate::math::Vec2;
use bitset_core::BitSet;

pub struct Bitzet {
    quadrants: [[u32; 1024]; 4],
}

pub struct ZOrderIterator<'a> {
    z: usize,
    s: &'a Bitzet,
}

impl<'a> Iterator for ZOrderIterator<'a> {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        let num_bits = self.s.quadrants[0].len() * 32;
        while self.z < num_bits && !self.s.quadrants[0].bit_test(self.z) {
            self.z += 1;
        }
        if self.z >= num_bits {
            return None;
        }
        let ret = zinv(self.z as u32);
        self.z += 1;
        Some(ret)
    }
}

impl Bitzet {
    pub fn new() -> Bitzet {
        Bitzet {
            quadrants: [[0; 1024]; 4],
        }
    }
    pub fn insert(&mut self, v: Vec2) {
        self.quadrants[quadrant_index(&v)].bit_set(zorder_abs(&v));
    }
    pub fn get(&self, v: &Vec2) -> bool {
        self.quadrants[quadrant_index(&v)].bit_test(zorder_abs(v))
    }
    pub fn difference(&self, other: &Self) -> Self {
        let mut q0 = self.quadrants[0].clone();
        q0.bit_andnot(&other.quadrants[0]);
        let mut q1 = self.quadrants[1].clone();
        q1.bit_andnot(&other.quadrants[1]);
        let mut q2 = self.quadrants[2].clone();
        q2.bit_andnot(&other.quadrants[2]);
        let mut q3 = self.quadrants[3].clone();
        q3.bit_andnot(&other.quadrants[3]);

        Bitzet {
            quadrants: [q0, q1, q2, q3],
        }
    }
    pub fn iter<'a>(&'a self) -> ZOrderIterator<'a> {
        ZOrderIterator { s: self, z: 0 }
    }
}
fn quadrant_index(v: &Vec2) -> usize {
    let xneg = if v.x() < 0 { 1 } else { 0 };
    let yneg = if v.y() < 0 { 1 } else { 0 };
    yneg * 2 + xneg
}
fn zorder_abs(v: &Vec2) -> usize {
    zorder2(v.x().abs() as u32, v.y().abs() as u32)
}
fn zorder2(mut x: u32, mut y: u32) -> usize {
    let mut rout: usize = 0;

    let mut n = 0;
    while x > 0 || y > 0 {
        rout <<= 1;
        rout |= (y & 0b1) as usize;
        rout <<= 1;
        rout |= (x & 0b1) as usize;
        x >>= 1;
        y >>= 1;
        n += 1;
    }
    let mut out = 0;
    for _ in 0..n {
        out <<= 2;
        out |= (rout & 0b11) as usize;
        rout >>= 2;
    }

    // println!("zorder: {} {} {:b}", x, y, out);
    out
}

#[test]
fn zorder2_test() {
    assert_eq!(zorder2(0, 0), 0);
    assert_eq!(zorder2(3, 5), 0b100111);
    assert_eq!(zorder2(6, 2), 0b011100);
    assert_eq!(zorder2(7, 7), 0b111111);
}

#[test]
fn basic() {
    let mut bs = Bitzet::new();
    bs.insert(Vec2(1, 1));
    bs.insert(Vec2(2, 1));
    bs.insert(Vec2(3, 1));

    let mut bs2 = Bitzet::new();
    bs2.insert(Vec2(2, 1));
    let bs3 = bs.difference(&bs2);
    let s = bs3.iter().collect::<Vec<_>>();
    println!("s: {:?}", s);
}

fn zinv(mut z: u32) -> Vec2 {
    let mut x = 0;
    let mut y = 0;
    let mut mask = 1;
    while z != 0 {
        x |= z & mask;
        z >>= 1;
        y |= z & mask;
        mask <<= 1;
    }
    Vec2(x as i32, y as i32)
}

#[test]
fn test_zinv() {
    assert_eq!(zinv(0b0), Vec2(0, 0));
    assert_eq!(zinv(0b100110), Vec2(0b10, 0b101));
    assert_eq!(zinv(0b111101), Vec2(0b111, 0b110));

    let v = zinv(0b111100);
    println!("{:b} {:b}", v.x(), v.y());
}

#[test]
fn test1() {
    let mut x = 0u32;
    for i in 0..10 {
        println!("x: {:b}", x);

        x = (x + 0xaaaaaaab) & 0x55555555;
    }
}
