#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Inline {
    Text(String),
    Link((String, String)),
}

// pub enum Block {
//     Header((Inline, Block))
// }
