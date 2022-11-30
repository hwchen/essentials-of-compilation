use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest() {
        assert!(grammar::TermParser::new().parse("42").is_ok());
    }
}
