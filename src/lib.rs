// pub mod passport_grammar;
// pub mod password_grammar;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub password_grammar);
lalrpop_mod!(pub passport_grammar);
lalrpop_mod!(pub passport_simple_grammar);
lalrpop_mod!(pub bag_grammar);
lalrpop_mod!(pub asm_grammar);
lalrpop_mod!(pub bitmask_grammar);
lalrpop_mod!(pub badmath_grammar);
lalrpop_mod!(pub message_grammar);
lalrpop_mod!(pub test_grammar);
lalrpop_mod!(pub ingredients_grammar);
pub mod passport {
    #[derive(Debug)]
    pub enum LenUnit {
        Cm,
        In,
    }
    #[derive(Debug)]
    pub enum Entry {
        BYr(i64),
        IYr(i64),
        EYr(i64),
        Hgt(i64, LenUnit),
        Hcl,
        Ecl,
        Pid,
        Cid,
    }
}
pub mod bag {
    #[derive(Debug, Clone)]
    pub struct Content {
        pub color: String,
        pub num: usize,
    }
    #[derive(Debug, Clone)]
    pub struct Policy {
        pub color: String,
        pub content: Option<Vec<Content>>,
    }
}

pub mod vm;

pub mod bitmask {
    #[derive(Debug, Clone)]
    pub enum Op {
        Mask(String),
        Mem(u64, u64),
    }
}

pub mod badmath {
    #[derive(Debug, Clone)]
    pub enum Term {
        Num(i64),
        Op(Box<Term>, char, Box<Term>),
    }
    pub fn binop(t1: Term, op: char, t2: Term) -> Term {
        Term::Op(Box::new(t1), op, Box::new(t2))
    }
}

pub mod message {
    #[derive(Debug, Clone)]
    pub enum Element {
        Rule(i64, Node),
        Message(String),
    }
    #[derive(Debug, Clone)]
    pub enum Node {
        Seq(Vec<i64>),
        Or(Box<Node>, Box<Node>),
        String(String),
    }
}

use std::{io::BufRead, iter::FromIterator};
pub fn map_input_vec<F, B>(f: F) -> Vec<B>
where
    F: FnMut(String) -> B,
{
    map_input(f)
}

pub fn map_input<F, B, C>(mut f: F) -> C
where
    F: FnMut(String) -> B,
    C: FromIterator<B>,
{
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| f(line.unwrap()))
        .collect()
}

pub fn semicolonized_input() -> String {
    // add semicolon after each line -> helps to make some languages LR(1) parsable
    // inspired by go...
    let mut code = String::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        code.push_str(&format!("{};\n", line));
    }
    // println!("rewritten code: {}", code);
    code
}

pub fn skip_empty_input() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            if !l.is_empty() {
                Some(l)
            } else {
                None
            }
        })
        .collect()
}

pub mod math {
    use std::ops;
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Vec2(pub i32, pub i32);

    impl Vec2 {
        pub fn x(&self) -> i32 {
            self.0
        }
        pub fn y(&self) -> i32 {
            self.1
        }
        pub fn manhattan(&self) -> i32 {
            self.x().abs() + self.y().abs()
        }
        pub fn rotate_right90(&self) -> Vec2 {
            // matrix:
            // x      0       -1
            // y      1        0
            Vec2(self.1, -self.0)
        }
        pub fn rotate_left90(&self) -> Vec2 {
            // matrix:
            // x       0       1
            // y      -1       0
            Vec2(-self.1, self.0)
        }
    }

    impl ops::Add for Vec2 {
        type Output = Vec2;
        fn add(self, rhs: Self) -> Self::Output {
            Vec2(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl ops::AddAssign for Vec2 {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            self.1 += rhs.1;
        }
    }

    impl ops::Mul<i32> for Vec2 {
        type Output = Vec2;

        fn mul(self, rhs: i32) -> Self::Output {
            Vec2(self.0 * rhs, self.1 * rhs)
        }
    }

    impl ops::MulAssign<i32> for Vec2 {
        fn mul_assign(&mut self, rhs: i32) {
            self.0 *= rhs;
            self.1 *= rhs;
        }
    }

    impl From<(i32, i32)> for Vec2 {
        fn from(v: (i32, i32)) -> Self {
            Vec2(v.0, v.1)
        }
    }
    impl From<char> for Vec2 {
        fn from(c: char) -> Self {
            match c {
                'N' => Vec2(0, 1),
                'S' => Vec2(0, -1),
                'E' => Vec2(1, 0),
                'W' => Vec2(-1, 0),
                _ => panic!("unhandled direction: {}", c),
            }
        }
    }
    #[cfg(test)]
    mod test_vec2 {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(Vec2(0, 0) + Vec2(2, 3), Vec2(2, 3));
            assert_eq!(Vec2(4, 5) + Vec2(2, 3), Vec2(6, 8));
            assert_eq!(Vec2(2, 3) + Vec2(-2, -3), Vec2(0, 0));
        }
        #[test]
        fn test_addassign() {
            let mut v = Vec2(0, 0);
            v += Vec2(2, 3);
            assert_eq!(v, Vec2(2, 3));
            v += Vec2(-2, -3);
            assert_eq!(v, Vec2(2, 3) + Vec2(-2, -3));
        }
        #[test]
        fn test_mul() {
            assert_eq!(Vec2(2, 3) * 8, Vec2(2 * 8, 3 * 8));
            assert_eq!(Vec2(-2, -3) * 8, Vec2(-2 * 8, -3 * 8));
            assert_eq!(Vec2(4, 5) * 0, Vec2(0, 0));
        }
        #[test]
        fn test_mulassign() {
            let mut v = Vec2(2, 3);
            v *= 27;
            assert_eq!(v, Vec2(2, 3) * 27);
            v *= -3;
            assert_eq!(v, Vec2(2, 3) * 27 * -3);
            v *= 0;
            assert_eq!(v, Vec2(0, 0));
        }
        #[test]
        fn test_from() {
            let v: Vec2 = (7, -8).into();
            assert_eq!(v, Vec2(7, -8));

            let n: Vec2 = 'N'.into();
            let s: Vec2 = 'S'.into();
            let e: Vec2 = 'E'.into();
            let w: Vec2 = 'W'.into();
            assert_eq!(n, s * -1);
            assert_eq!(e, w * -1);
            assert_ne!(n, e);
            assert_ne!(n, Vec2(0, 0));
        }
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Vec3(pub i32, pub i32, pub i32);
    impl Vec3 {
        pub fn x(&self) -> i32 {
            self.0
        }
        pub fn y(&self) -> i32 {
            self.1
        }
        pub fn z(&self) -> i32 {
            self.2
        }
        pub fn manhattan(&self) -> i32 {
            self.x().abs() + self.y().abs() + self.z().abs()
        }
    }

    impl ops::Add for Vec3 {
        type Output = Vec3;
        fn add(self, rhs: Self) -> Self::Output {
            Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
        }
    }

    impl ops::AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            self.1 += rhs.1;
            self.2 += rhs.2;
        }
    }

    impl ops::Mul<i32> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: i32) -> Self::Output {
            Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
        }
    }

    impl ops::MulAssign<i32> for Vec3 {
        fn mul_assign(&mut self, rhs: i32) {
            self.0 *= rhs;
            self.1 *= rhs;
            self.2 *= rhs;
        }
    }

    impl From<(i32, i32, i32)> for Vec3 {
        fn from(v: (i32, i32, i32)) -> Self {
            Vec3(v.0, v.1, v.2)
        }
    }
    #[cfg(test)]
    mod test_vec3 {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(Vec3(0, 0, 0) + Vec3(2, 3, 1), Vec3(2, 3, 1));
            assert_eq!(Vec3(4, 5, 6) + Vec3(2, 3, 4), Vec3(6, 8, 10));
            assert_eq!(Vec3(2, 3, 4) + Vec3(-2, -3, -4), Vec3(0, 0, 0));
        }
        #[test]
        fn test_addassign() {
            let mut v = Vec3(0, 0, 0);
            v += Vec3(2, 3, 4);
            assert_eq!(v, Vec3(2, 3, 4));
            v += Vec3(-2, -3, -4);
            assert_eq!(v, Vec3(2, 3, 4) + Vec3(-2, -3, -4));
        }
        #[test]
        fn test_mul() {
            assert_eq!(Vec3(2, 3, 4) * 8, Vec3(2 * 8, 3 * 8, 4 * 8));
            assert_eq!(Vec3(-2, -3, -4) * 8, Vec3(-2 * 8, -3 * 8, -4 * 8));
            assert_eq!(Vec3(4, 5, 6) * 0, Vec3(0, 0, 0));
        }
        #[test]
        fn test_mulassign() {
            let mut v = Vec3(2, 3, 4);
            v *= 27;
            assert_eq!(v, Vec3(2, 3, 4) * 27);
            v *= -3;
            assert_eq!(v, Vec3(2, 3, 4) * 27 * -3);
            v *= 0;
            assert_eq!(v, Vec3(0, 0, 0));
        }
        #[test]
        fn test_from() {
            let v: Vec3 = (7, -8, 11).into();
            assert_eq!(v, Vec3(7, -8, 11));
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct Vec4(pub i32, pub i32, pub i32, pub i32);
    impl Vec4 {
        pub fn x(&self) -> i32 {
            self.0
        }
        pub fn y(&self) -> i32 {
            self.1
        }
        pub fn z(&self) -> i32 {
            self.2
        }
        pub fn w(&self) -> i32 {
            self.3
        }
        pub fn manhattan(&self) -> i32 {
            self.x().abs() + self.y().abs() + self.z().abs() + self.w().abs()
        }
    }

    impl ops::Add for Vec4 {
        type Output = Vec4;
        fn add(self, rhs: Self) -> Self::Output {
            Vec4(
                self.0 + rhs.0,
                self.1 + rhs.1,
                self.2 + rhs.2,
                self.3 + rhs.3,
            )
        }
    }

    impl ops::AddAssign for Vec4 {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            self.1 += rhs.1;
            self.2 += rhs.2;
            self.3 += rhs.3;
        }
    }

    impl ops::Mul<i32> for Vec4 {
        type Output = Vec4;

        fn mul(self, rhs: i32) -> Self::Output {
            Vec4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
        }
    }

    impl ops::MulAssign<i32> for Vec4 {
        fn mul_assign(&mut self, rhs: i32) {
            self.0 *= rhs;
            self.1 *= rhs;
            self.2 *= rhs;
            self.3 *= rhs;
        }
    }

    impl From<(i32, i32, i32, i32)> for Vec4 {
        fn from(v: (i32, i32, i32, i32)) -> Self {
            Vec4(v.0, v.1, v.2, v.3)
        }
    }
    #[cfg(test)]
    mod test_vec4 {
        use super::*;
        #[test]
        fn test_add() {
            assert_eq!(Vec4(0, 0, 0, 0) + Vec4(2, 3, 1, 4), Vec4(2, 3, 1, 4));
            assert_eq!(Vec4(4, 5, 6, 7) + Vec4(2, 3, 4, 5), Vec4(6, 8, 10, 12));
            assert_eq!(Vec4(2, 3, 4, 5) + Vec4(-2, -3, -4, -5), Vec4(0, 0, 0, 0));
        }
        #[test]
        fn test_addassign() {
            let mut v = Vec4(0, 0, 0, 0);
            v += Vec4(2, 3, 4, 5);
            assert_eq!(v, Vec4(2, 3, 4, 5));
            v += Vec4(-2, -3, -4, -5);
            assert_eq!(v, Vec4(2, 3, 4, 5) + Vec4(-2, -3, -4, -5));
        }
        #[test]
        fn test_mul() {
            assert_eq!(Vec4(2, 3, 4, 5) * 8, Vec4(2 * 8, 3 * 8, 4 * 8, 5 * 8));
            assert_eq!(
                Vec4(-2, -3, -4, -5) * 8,
                Vec4(-2 * 8, -3 * 8, -4 * 8, -5 * 8)
            );
            assert_eq!(Vec4(4, 5, 6, 7) * 0, Vec4(0, 0, 0, 0));
        }
        #[test]
        fn test_mulassign() {
            let mut v = Vec4(2, 3, 4, 5);
            v *= 27;
            assert_eq!(v, Vec4(2, 3, 4, 5) * 27);
            v *= -3;
            assert_eq!(v, Vec4(2, 3, 4, 5) * 27 * -3);
            v *= 0;
            assert_eq!(v, Vec4(0, 0, 0, 0));
        }
        #[test]
        fn test_from() {
            let v: Vec4 = (7, -8, 11, -17).into();
            assert_eq!(v, Vec4(7, -8, 11, -17));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod bitzet;
