use super::token;
use crate::token::Token;

use combine::{
    choice, eof,
    parser::{char::spaces, repeat::repeat_until},
    Parser, Stream,
};

fn tokens<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    choice((
        token::left_paren(),
        token::right_paren(),
        token::text(),
        token::link(),
        token::table(),
        token::header(),
        token::string(),
    ))
}

fn tokenize<Input>() -> impl Parser<Input, Output = Vec<Token>>
where
    Input: Stream<Token = char>,
{
    repeat_until(spaces().with(tokens()), eof())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_link() {
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
    fn it_text() {
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
}
