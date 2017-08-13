use std::fmt;
use std::hash::{Hash, Hasher};
use std::slice::{Iter, IterMut};

#[derive(Clone, PartialEq, Hash, Debug)]
pub enum Fragment {
    Index(usize),
    Name(String)
}

impl fmt::Display for Fragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Fragment::Index(ref index) => write!(f, "{}", index),
            Fragment::Name(ref name) => write!(f, "{}", name),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    fragments: Vec<Fragment>,
    is_relative: bool
}

impl Path {
    fn from_fragments(fragments: Vec<Fragment>, is_relative: bool) -> Path {
        Path {
            fragments: fragments,
            is_relative: is_relative
        }
    }

    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn first(&self) -> Option<&Fragment>  {
        self.fragments.first()
    }

    pub fn last(&self) -> Option<&Fragment>  {
        self.fragments.last()
    }

    pub fn iter(&self) -> Iter<Fragment> {
        self.fragments.iter()
    }

    pub fn len(&self) -> usize {
        self.fragments.len()
    }

    pub fn from_str(path: &str) -> Option<Path> {
        if path.is_empty() {
            return None;
        }

        let is_relative = path.starts_with('.');

        // If the path is relative remove the first dot
        let new_path = if is_relative {
            let mut iter = path.chars();
            iter.next();
            iter.as_str()
        } else {
            path
        };

        let fragments: Vec<Fragment> = new_path.split('.').map(|ref token| {
            match token.parse::<usize>() {
                Ok(index) => Fragment::Index(index),
                Err(_) => Fragment::Name(token.to_string()),
            }
        }).collect();

        Some(Path::from_fragments(fragments, is_relative))
    }

}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_relative {
            try!(write!(f, "."));
        }

        write!(f, "{}", self.fragments.iter().map(|ref fragment| fragment.to_string()).collect::<Vec<_>>().join("."))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        if self.is_relative != other.is_relative {
            return false;
        }

        if self.fragments.len() != other.fragments.len() {
            return false;
        }

        return self.fragments == other.fragments;
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}