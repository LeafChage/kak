use crate::token::Token;
use combine::{
    choice, eof,
    parser::{
        char::{crlf, newline, space, tab},
        repeat::take_until,
    },
    token, Parser, Stream,
};

pub fn left_paren<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('(').map(|_| Token::LeftParen)
}

pub fn right_paren<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token(')').map(|_| Token::RightParen)
}

pub fn text<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('"').map(|_| Token::Text)
}

pub fn link<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('@').map(|_| Token::Link)
}

pub fn table<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('|').map(|_| Token::Table)
}

pub fn header<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('#').map(|_| Token::Header)
}

pub fn string<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    take_until::<String, _, _>(choice((
        space().map(|_| Token::Unknown),
        eof().map(|_| Token::Unknown),
        crlf().map(|_| Token::Unknown),
        newline().map(|_| Token::Unknown),
        tab().map(|_| Token::Unknown),
        left_paren(),
        right_paren(),
    )))
    .map(|v| Token::string(v))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_left_paren() {
        assert_eq!(left_paren().parse("("), Ok((Token::LeftParen, "")))
    }
    #[test]
    fn it_right_paren() {
        assert_eq!(right_paren().parse(")"), Ok((Token::RightParen, "")))
    }
    #[test]
    fn it_text() {
        assert_eq!(text().parse("\""), Ok((Token::Text, "")))
    }
    #[test]
    fn it_link() {
        assert_eq!(link().parse("@"), Ok((Token::Link, "")))
    }
    #[test]
    fn it_table() {
        assert_eq!(table().parse("|"), Ok((Token::Table, "")))
    }
    #[test]
    fn it_block() {
        assert_eq!(header().parse("#"), Ok((Token::Header, "")))
    }

    #[test]
    fn it_string() {
        assert_eq!(
            string().parse("helloworld"),
            Ok((Token::string("helloworld"), ""))
        );
        assert_eq!(
            string().parse("hello world"),
            Ok((Token::string("hello"), " world"))
        );
        assert_eq!(string().parse("hello("), Ok((Token::string("hello"), "(")));
        assert_eq!(string().parse("hello)"), Ok((Token::string("hello"), ")")));
    }
}
