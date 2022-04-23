use super::syntax::Inline;
use super::tokenize;
use super::Token;
use combine::{
    choice, eof,
    parser::{
        char::{crlf, newline, space, spaces, tab},
        repeat::{repeat_until, take_until},
    },
    token, Parser, Stream,
};

fn skip_space<Input>(p: impl Parser<Input, Output = Token>) -> impl Parser<Input, Output = Token>
where
    Input: Stream<Token = char>,
{
    spaces().with(p).map(|v| v)
}

/// parse from "(" to ")"
fn paren_between<Input, O>(p: impl Parser<Input, Output = O>) -> impl Parser<Input, Output = O>
where
    Input: Stream<Token = char>,
{
    skip_space(tokenize::left_paren())
        .with(p)
        .skip(skip_space(tokenize::right_paren()))
}

pub fn link<Input>() -> impl Parser<Input, Output = Inline>
where
    Input: Stream<Token = char>,
{
    paren_between(
        skip_space(tokenize::link())
            .and(skip_space(tokenize::string()))
            .and(skip_space(tokenize::string())),
    )
    .map(|((_, label), link)| match (label, link) {
        (Token::String(label), Token::String(link)) => Inline::link(label, link),
        (_, _) => panic!("??"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::parser::char::string;
    #[test]
    fn ts_paren_between() {
        assert_eq!(
            paren_between(string("link")).parse("(link)"),
            Ok(("link", ""))
        )
    }

    #[test]
    fn ts_link() {
        assert_eq!(
            link().parse("(@ link https://example.com)"),
            Ok((Inline::link("link", "https://example.com"), ""))
        )
    }
}
