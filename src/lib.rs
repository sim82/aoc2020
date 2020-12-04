// pub mod passport_grammar;
// pub mod password_grammar;
#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub password_grammar);
lalrpop_mod!(pub passport_grammar);
lalrpop_mod!(pub passport_simple_grammar);

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
