use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Glue {
    Bidirectional,
    Left,
    Right
}

impl fmt::Display for Glue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Glue::Bidirectional => write!(f, "<>"),
            Glue::Left => write!(f, "G<"),
            Glue::Right => write!(f, "G>")
        }
    }
}