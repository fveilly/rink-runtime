use std::fmt;
use path::Path;

pub struct VariableAssignment {
    name: String,
    is_new_declaration: bool,
    is_global: bool
}

impl VariableAssignment {
    pub fn new(name: String, is_new_declaration: bool, is_global: bool) -> VariableAssignment {
        VariableAssignment {
            name: name,
            is_new_declaration: is_new_declaration,
            is_global: is_global
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_new_declaration(&self) -> bool {
        self.is_new_declaration
    }

    pub fn is_global(&self) -> bool {
        self.is_global
    }

    pub fn set_is_global(&mut self, is_global: bool) {
        self.is_global = is_global
    }
}

impl fmt::Display for VariableAssignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarAssign to {}", self.name)
    }
}

pub struct VariableReference {
    name: String
}

impl VariableReference {
    pub fn new(name: String) ->  VariableReference {
        VariableReference {
            name: name
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for VariableReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "var({})", self.name)
    }
}

pub struct ReadCount {
    target: Path
}

impl ReadCount {
    pub fn new(target: Path) ->  ReadCount {
        ReadCount {
            target: target
        }
    }

    pub fn target(&self) -> &Path {
        &self.target
    }
}

impl fmt::Display for ReadCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "read_count({})", self.target.to_string())
    }
}