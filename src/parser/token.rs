#[derive(Clone, Eq, PartialEq, Debug, Copy)]
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
}
