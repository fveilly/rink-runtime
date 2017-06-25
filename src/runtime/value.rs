#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    List,
    String(String),
    DivertTarget,
    VariablePointer
}
