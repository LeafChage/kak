use combine::{token, Parser, Stream};

fn left_paren<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token('(')
}

fn right_paren<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token(')')
}

fn text<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token('"')
}

fn link<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token('@')
}

fn table<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token('|')
}

fn block<Input>() -> impl Parser<Input, Output = char>
where
    Input: Stream<Token = char>,
{
    token('#')
}

#[test]
fn it_left_paren() {
    assert_eq!(left_paren().parse("("), Ok(('(', "")))
}
#[test]
fn it_right_paren() {
    assert_eq!(right_paren().parse(")"), Ok((')', "")))
}
#[test]
fn it_text() {
    assert_eq!(text().parse("\""), Ok(('"', "")))
}
#[test]
fn it_link() {
    assert_eq!(link().parse("@"), Ok(('@', "")))
}
#[test]
fn it_table() {
    assert_eq!(table().parse("|"), Ok(('|', "")))
}
#[test]
fn it_block() {
    assert_eq!(block().parse("#"), Ok(('#', "")))
}
