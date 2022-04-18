use super::Token;
use combine::{
    choice, eof,
    parser::{
        char::{crlf, newline, space, spaces, tab},
        repeat::{repeat_until, take_until},
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

fn tokens<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    choice((
        left_paren(),
        right_paren(),
        text(),
        link(),
        table(),
        header(),
        string(),
    ))
}

pub fn tokenize<Input>() -> impl Parser<Input, Output = Vec<Token>>
where
    Input: Stream<Token = char>,
{
    repeat_until(spaces().with(tokens()), eof())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_link_expression() {
        assert_eq!(
            tokenize().parse(r#"(@ link https://example.com)"#),
            Ok((
                vec![
                    Token::LeftParen,
                    Token::Link,
                    Token::string("link"),
                    Token::string("https://example.com"),
                    Token::RightParen,
                ],
                ""
            ))
        )
    }

    #[test]
    fn it_text_expression() {
        assert_eq!(
            tokenize().parse(r#" (" hello world, good morning)"#),
            Ok((
                vec![
                    Token::LeftParen,
                    Token::Text,
                    Token::string("hello"),
                    Token::string("world,"),
                    Token::string("good"),
                    Token::string("morning"),
                    Token::RightParen,
                ],
                ""
            ))
        )
    }
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
