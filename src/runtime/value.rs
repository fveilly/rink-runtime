use path::Path;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    //List,
    String(String),
    DivertTarget(Path),
    VariablePointer(String, i32)
}

impl Value {
    pub fn as_int(&self) -> Option<i32> {
        match self {
            &Value::Int(value) => Some(value),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            &Value::Float(value) => Some(value),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            &Value::String(ref value) => Some(value),
            _ => None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int(value) => write!(f, "{}", value),
            &Value::Float(value) => write!(f, "{}", value),
            &Value::String(ref value) => write!(f, "{}", value),
            &Value::DivertTarget(ref value) => write!(f, "DivertTarget({})", value.to_string()),
            &Value::VariablePointer(ref name, _) => write!(f, "VarPtr({})", name)
        }
    }
}