use super::token::Token;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Inline {
    Text(String),
    Link(String, String),
}

impl Inline {
    pub fn text(v: impl Into<String>) -> Inline {
        Inline::text(v.into())
    }

    pub fn link(label: impl Into<String>, link: impl Into<String>) -> Inline {
        Inline::link(label.into(), link.into())
    }
}

// pub enum Block {
//     Header((Inline, Block))
// }
