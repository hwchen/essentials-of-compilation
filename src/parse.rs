use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest() {
        dbg!(grammar::LangIntParser::new().parse("42"));
        assert!(grammar::LangIntParser::new().parse("42").is_ok());
    }
}
