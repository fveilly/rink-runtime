use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
#[derive(PartialEq)]
pub enum Component {
    Index(IndexComponent),
    Named(NamedComponent)
}

impl Component {
    pub fn is_index_component(&self) -> bool {
        self.as_index_component().is_some()
    }

    pub fn as_index_component(&self) -> Option<&IndexComponent> {
        match *self {
            Component::Index(ref component) => Some(component),
            _ => None,
        }
    }

    pub fn is_named_component(&self) -> bool {
        self.as_named_component().is_some()
    }

    pub fn as_named_component(&self) -> Option<&NamedComponent> {
        match *self {
            Component::Named(ref component) => Some(component),
            _ => None,
        }
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Component::Index(ref index_component) => write!(f, "{}", index_component.index()),
            Component::Named(ref named_component) => write!(f, "{}", named_component.name()),
        }
    }
}

#[derive(Clone)]
pub struct NamedComponent {
    name: String
}

impl PartialEq for NamedComponent {
    fn eq(&self, other: &NamedComponent) -> bool {
        self.name == other.name
    }
}

impl Hash for NamedComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for NamedComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl NamedComponent {
    pub fn is_parent(&self) -> bool  {
        self.name == "^"
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct IndexComponent {
    index: usize
}

impl PartialEq for IndexComponent {
    fn eq(&self, other: &IndexComponent) -> bool {
        self.index == other.index
    }
}

impl Hash for IndexComponent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl fmt::Display for IndexComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index)
    }
}

impl IndexComponent {
    pub fn index(&self) -> usize {
        self.index
    }
}

pub struct Path {
    components: Vec<Component>,
    is_relative: bool
}

impl Path {
    fn from_components(components: Vec<Component>, is_relative: bool) -> Path {
        Path {
            components: components,
            is_relative: is_relative
        }
    }

    pub fn is_relative(&self) -> bool {
        self.is_relative
    }

    pub fn head(&self) -> Option<&Component>  {
        self.components.first()
    }

    pub fn last(&self) -> Option<&Component>  {
        self.components.last()
    }

    pub fn tail(&self) -> Option<Path> {
        let length = self.components.len();

        if length > 1 {
            match self.components.get(1..length-1) {
                Some(components) => {
                    let mut new_vector : Vec<Component> = Vec::with_capacity(components.len());
                    new_vector.clone_from_slice(components);
                    Some(Path::from_components(new_vector, self.is_relative))
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn length(&self) -> usize {
        self.components.len()
    }

    pub fn contains_named_component(&self) -> bool {
        let mut iter = self.components.iter();
        while let Some(component) = iter.next() {
            if let &Component::Named(_) = component {
                return true;
            }
        }
        false
    }

    pub fn parse(path: &str) -> Option<Path> {
        if path.is_empty() {
            return None;
        }

        let is_relative = path.starts_with('.');

        // FIXME: Find a more elegant way to do this
        let final_path = if is_relative {
            let mut iter = path.chars();
            iter.next();
            iter.as_str()
        } else {
            path
        };

        let components: Vec<Component> = final_path.split('.').map(|ref token| {
            match token.parse::<usize>() {
                Ok(index) => Component::Index(IndexComponent{index: index}),
                Err(_) => Component::Named(NamedComponent{name: token.to_string()}),
            }
        }).collect();

        Some(Path::from_components(components, is_relative))
    }

}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_relative {
            try!(write!(f, "."));
        }

        let components_str = self.components.iter().map(|ref component| component.to_string()).collect::<Vec<_>>().join(".");
        write!(f, "{}", components_str)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Path) -> bool {
        if self.is_relative != other.is_relative {
            return false;
        }

        if self.components.len() != other.components.len() {
            return false;
        }

        return self.components == other.components;
    }
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}