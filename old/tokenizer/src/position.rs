
#[derive(Debug, Default, Clone, PartialOrd, PartialEq)]
pub struct Position {
    pub file: String,
    pub line: u32,
    pub column: u32
}