use std::fmt;

pub struct Tag {
    text: String
}

impl Tag {
    pub fn new(text: String) -> Tag {
        Tag {
            text: text
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.text)
    }
}