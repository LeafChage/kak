#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    /// (
    LeftParen,

    /// )
    RightParen,

    /// "
    Text,

    /// @
    Link,

    /// |
    Table,

    /// #
    Header,

    /// string
    String(String),

    /// unknown
    Unknown,
}

impl Token {
    pub fn string(v: impl Into<String>) -> Token {
        Token::String(v.into())
    }
}
