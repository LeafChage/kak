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

    /// !
    Image,

    /// * | + | -
    List,

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

// impl std::fmt::Debug for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match &self {
//                 &Token::LeftParen => "(",
//                 &Token::RightParen => ")",
//                 &Token::Text => "\"",
//                 &Token::Link => "@",
//                 &Token::Table => "|",
//                 &Token::Header => "#",
//                 &Token::String(str) => str,
//                 &Token::Unknown => "?",
//             }
//         )
//     }
// }
