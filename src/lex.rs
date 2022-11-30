use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("print")]
    Print,
    #[token("input_int")]
    InputInt,

    #[regex("[0-9]+")]
    Int,

    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoketest() {
        let input = "print(input_int() + -(5 + 3) )";
        let mut lex = Token::lexer(input);

        assert_eq!(lex.next(), Some(Token::Print));
        assert_eq!(lex.span(), 0..5);
        assert_eq!(lex.slice(), "print");

        assert_eq!(lex.next(), Some(Token::LParen));
        assert_eq!(lex.next(), Some(Token::InputInt));
        assert_eq!(lex.next(), Some(Token::LParen));
        assert_eq!(lex.next(), Some(Token::RParen));
        assert_eq!(lex.next(), Some(Token::Plus));
        assert_eq!(lex.next(), Some(Token::Minus));
        assert_eq!(lex.next(), Some(Token::LParen));

        assert_eq!(lex.next(), Some(Token::Int));
        assert_eq!(lex.span(), 22..23);
        assert_eq!(lex.slice(), "5");

        assert_eq!(lex.next(), Some(Token::Plus));

        assert_eq!(lex.next(), Some(Token::Int));
        assert_eq!(lex.span(), 26..27);
        assert_eq!(lex.slice(), "3");

        assert_eq!(lex.next(), Some(Token::RParen));
        assert_eq!(lex.next(), Some(Token::RParen));
    }
}
