// pub mod passport_grammar;
// pub mod password_grammar;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub password_grammar);
lalrpop_mod!(pub passport_grammar);
lalrpop_mod!(pub passport_simple_grammar);
lalrpop_mod!(pub bag_grammar);
lalrpop_mod!(pub asm_grammar);

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

pub mod math {
    use std::ops;
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
