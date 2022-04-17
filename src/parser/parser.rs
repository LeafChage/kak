use super::Token;

use combine::{
    many,
    parser::{char::space, repeat::take_until},
    token, Parser, Stream,
};

fn left_paren<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('(').map(|_| Token::LeftParen)
}

fn right_paren<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token(')').map(|_| Token::RightParen)
}

fn text<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('"').map(|_| Token::Text)
}

fn link<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('@').map(|_| Token::Link)
}

fn table<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('|').map(|_| Token::Table)
}

fn header<Input>() -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    token('#').map(|_| Token::Header)
}

fn spaces<Input>() -> impl Parser<Input, Output = Vec<char>>
where
    Input: Stream<Token = char>,
{
    many::<Vec<_>, _, _>(space())
}

#[test]
fn it_link_parser_moving() {
    println!(
        "{:?}",
        left_paren()
            .skip(spaces())
            .and(link())
            .skip(spaces())
            .and(take_until::<String, _, _>(space()))
            .skip(spaces())
            .and(take_until::<String, _, _>(right_paren()))
            .and(right_paren())
            .parse(r#"(@ link https://example.com)"#)
            .map(|(((((lp, l), label), location), rp), output)| (
                (lp, l, label, location, rp),
                output
            ))
    );
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
