use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::io::Read;

use error::InkError;
use path::Path;

use runtime::RuntimeObject;
use runtime::value::Value;
use runtime::glue::Glue;
use runtime::control_command::ControlCommand;
use runtime::divert::{Divert, PushPopType, TargetType};
use runtime::choice_point::ChoicePoint;
use runtime::variable::{VariableAssignment, VariableReference, ReadCount};
use runtime::tag::Tag;
use runtime::container::Container;

use serde::de::Error as SerdeError;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};

use serde_json;

struct RuntimeObjectVisitor {
}

impl RuntimeObjectVisitor {
    fn new() -> Self {
        RuntimeObjectVisitor {}
    }
}

impl<'de> Visitor<'de> for RuntimeObjectVisitor
{
    // Our Visitor is going to produce a RuntimeObject.
    type Value = RuntimeObject;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Runtime object")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v)))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Int(v as i32)))
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Float(v)))
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
    {
        Ok(RuntimeObject::Value(Value::Float(v as f32)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: SerdeError,
    {
        if v.starts_with("^") {
            return Ok(RuntimeObject::Value(Value::String(v.chars().skip(1).collect())));
        }

        match v {
            "\n" => Ok(RuntimeObject::Value(Value::String("\n".to_string()))),

            // Glue
            "<>" => Ok(RuntimeObject::Glue(Glue::Bidirectional)),
            "G<" => Ok(RuntimeObject::Glue(Glue::Left)),
            "G>" => Ok(RuntimeObject::Glue(Glue::Right)),

            // Control Commands
            "ev" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalStart)),
            "out" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalOutput)),
            "/ev" => Ok(RuntimeObject::ControlCommand(ControlCommand::EvalEnd)),
            "du" => Ok(RuntimeObject::ControlCommand(ControlCommand::Duplicate)),
            "pop" => Ok(RuntimeObject::ControlCommand(ControlCommand::PopEvaluatedValue)),
            "~ret" => Ok(RuntimeObject::ControlCommand(ControlCommand::PopFunction)),
            "->->" => Ok(RuntimeObject::ControlCommand(ControlCommand::PopTunnel)),
            "str" => Ok(RuntimeObject::ControlCommand(ControlCommand::BeginString)),
            "/str" => Ok(RuntimeObject::ControlCommand(ControlCommand::EndString)),
            "nop" => Ok(RuntimeObject::ControlCommand(ControlCommand::NoOp)),
            "choiceCnt" => Ok(RuntimeObject::ControlCommand(ControlCommand::ChoiceCount)),
            "turns" => Ok(RuntimeObject::ControlCommand(ControlCommand::TurnsSince)),
            "readc" => Ok(RuntimeObject::ControlCommand(ControlCommand::ReadCount)),
            "rnd" => Ok(RuntimeObject::ControlCommand(ControlCommand::Random)),
            "srnd" => Ok(RuntimeObject::ControlCommand(ControlCommand::SeedRandom)),
            "visit" => Ok(RuntimeObject::ControlCommand(ControlCommand::VisitIndex)),
            "seq" => Ok(RuntimeObject::ControlCommand(ControlCommand::SequenceShuffleIndex)),
            "thread" => Ok(RuntimeObject::ControlCommand(ControlCommand::StartThread)),
            "done" => Ok(RuntimeObject::ControlCommand(ControlCommand::Done)),
            "end" => Ok(RuntimeObject::ControlCommand(ControlCommand::End)),
            "listInt" => Ok(RuntimeObject::ControlCommand(ControlCommand::ListFromInt)),
            "range" => Ok(RuntimeObject::ControlCommand(ControlCommand::ListRange)),

            // Native functions
            //Some("L^") => {},

            // Void
            "void" => Ok(RuntimeObject::Void),

            _ => Err(SerdeError::custom("Invalid String"))
        }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
    {
        let mut opt_key : Option<&str> = map.next_key()?;
        if let &Some(key) = &opt_key {
            match key {
                // Divert target value to path
                "^->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            match Path::from_str(target) {
                                Some(path) => return Ok(RuntimeObject::Value(Value::DivertTarget(path))),
                                _ => return Err(SerdeError::custom("Cannot parse target path"))
                            }
                        }
                        _ => return Err(SerdeError::custom("Unexpected divert target value type"))
                    }
                },

                // VariablePointerValue
                "^var" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            let mut context_index = -1;
                            if let Some(("ci", value)) = map.next_entry()? as Option<(&str, i32)> {
                                context_index = value;
                            }
                            return Ok(RuntimeObject::Value(Value::VariablePointer(name.to_owned(), context_index)))
                        },
                        _ => return Err(SerdeError::custom("Unexpected variable pointer value type"))
                    }
                },

                // Divert
                "->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new();

                            let entry: Option<(&str, bool)> = map.next_entry()?;
                            match entry {
                                // Case {"->": "variableTarget", "var": true}
                                Some(("var", true)) => {
                                    divert.set_target(TargetType::Name(target.to_owned()));

                                    // Case {"->": "variableTarget", "var": true, "c": true}
                                    if let Some(("c", true)) = map.next_entry()? {
                                        divert.set_is_conditional(true);
                                    }
                                },
                                _ => {
                                    match Path::from_str(target) {
                                        Some(path) => divert.set_target(TargetType::Path(path)),
                                        _ => return Err(SerdeError::custom("Cannot parse divert target path"))
                                    }

                                    // Case {"->": "variableTarget", "c": true}
                                    if let Some(("c", true)) = entry {
                                        divert.set_is_conditional(true);
                                    }
                                }
                            }
                            return Ok(RuntimeObject::Divert(divert))
                        },
                        _ => return Err(SerdeError::custom("Unexpected divert type"))
                    }
                },

                // Function Call
                "f()" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_function();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path"))
                            }

                            // Case {"f()": "path.to.func", "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert))
                        },
                        _ => return Err(SerdeError::custom("Unexpected function call type"))
                    }
                },

                // Tunnel
                "->t->" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_tunnel();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path"))
                            }

                            // Case {"->t->": "path.tunnel", "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert))
                        },
                        _ => return Err(SerdeError::custom("Unexpected tunnel type"))
                    }
                },

                // External function
                "x()" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut divert = Divert::new_external_function();

                            match Path::from_str(target) {
                                Some(path) => divert.set_target(TargetType::Path(path)),
                                _ => return Err(SerdeError::custom("Cannot parse target path"))
                            }

                            // Case {"x()": "externalFuncName", "exArgs": 5}
                            if let Some(("exArgs", external_args)) = map.next_entry()? {
                                divert.set_external_args(external_args);
                            }

                            // Case {"x()": "externalFuncName", "exArgs": 5, "c": true}
                            if let Some(("c", true)) = map.next_entry()? {
                                divert.set_is_conditional(true);
                            }

                            return Ok(RuntimeObject::Divert(divert))
                        },
                        _ => return Err(SerdeError::custom("Unexpected external function type"))
                    }
                },

                // Choice
                "*" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            let mut choice = ChoicePoint::new();

                            match Path::from_str(target) {
                                Some(path) => choice.set_path_on_choice(path),
                                _ => return Err(SerdeError::custom("Cannot parse choice path"))
                            }

                            if let Some(("flg", flags)) = map.next_entry()? {
                                choice.set_flags(flags);
                            }

                            return Ok(RuntimeObject::Choice(choice))
                        },
                        _ => return Err(SerdeError::custom("Unexpected choice type"))
                    }
                },

                // Variable reference
                "VAR?" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            return Ok(RuntimeObject::VariableReference(VariableReference::new(name.to_owned())))
                        },
                        _ => return Err(SerdeError::custom("Unexpected variable reference type"))
                    }
                },

                // Read Count
                "CNT?" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(target) => {
                            match Path::from_str(target) {
                                Some(path) => return Ok(RuntimeObject::ReadCount(ReadCount::new(path))),
                                _ => return Err(SerdeError::custom("Cannot parse read count target"))
                            }
                        },
                        _ => return Err(SerdeError::custom("Unexpected read count type"))
                    }
                },

                // Variable assignment
                "VAR=" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            if let Some(("re", re)) = map.next_entry()? as Option<(&str, bool)> {
                                return Ok(RuntimeObject::VariableAssignment(VariableAssignment::new(name.to_owned(), !re, true)))
                            }

                            return Ok(RuntimeObject::VariableAssignment(VariableAssignment::new(name.to_owned(), true, true)))
                        },
                        _ => return Err(SerdeError::custom("Unexpected variable assignment type"))
                    }
                },

                // Temporary variable
                "temp=" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            return Ok(RuntimeObject::VariableAssignment(VariableAssignment::new(name.to_owned(), true, false)))
                        },
                        _ => return Err(SerdeError::custom("Unexpected temporary variable type"))
                    }
                },

                // Tag
                "#" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(tag) => {
                            return Ok(RuntimeObject::Tag(Tag::new(tag.to_owned())))
                        },
                        _ => return Err(SerdeError::custom("Unexpected temp var name type"))
                    }
                },

                // List
                "list" => { return Err(SerdeError::custom("TODO")) },

                _ => {}
            }
        }

        let mut opt_container: Option<Container> = None;

        while let Some(key) = opt_key {
            match key {
                // Container name
                "#n" => {
                    let value: Option<&str> = map.next_value()?;
                    match value {
                        Some(name) => {
                            if opt_container.is_none() {
                                opt_container = Some(Container::new());
                            }

                            if let Some(ref mut container_ref) = opt_container.as_mut() {
                                container_ref.set_name(name.to_owned());
                            }
                        },
                        _ => return Err(SerdeError::custom("Unexpected container name type"))
                    }
                },

                // Container flags
                "#f" => {
                    let value: Option<u8> = map.next_value()?;
                    match value {
                        Some(flags) => {
                            if opt_container.is_none() {
                                opt_container = Some(Container::new());
                            }

                            if let Some(ref mut container_ref) = opt_container.as_mut() {
                                container_ref.set_count_flags(flags);
                            }
                        },
                        _ => return Err(SerdeError::custom("Unexpected container flags type"))
                    }
                },

                // Sub-container
                _ => {
                    let value: Option<RuntimeObject> = map.next_value()?;
                    match value {
                        Some(obj) => {
                            if let RuntimeObject::Container(mut sub_container) = obj
                                {
                                    if opt_container.is_none() {
                                        opt_container = Some(Container::new());
                                    }

                                    sub_container.set_name(key.to_owned());

                                    if let Some(ref mut container_ref) = opt_container.as_mut() {
                                        container_ref.add_child(RuntimeObject::Container(sub_container));
                                    }
                                }
                        },
                        _ => return Err(SerdeError::custom("Unexpected sub-container type"))
                    }
                },
            }

            opt_key = map.next_key()?;
        }

        if let Some(container) = opt_container {
            return Ok(RuntimeObject::Container(container));
        }

        Err(SerdeError::custom("Runtime Object dictionary match not found"))
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
    {
        let mut runtime_objects: Vec<RuntimeObject> = Vec::new();

        let mut opt_child: Option<RuntimeObject> = seq.next_element()?;
        while let Some(child) = opt_child {
            opt_child = seq.next_element()?;

            if opt_child.is_some() {
                runtime_objects.push(child);
            }
            else {
                if let RuntimeObject::Container(mut container) = child {
                    container.prepend(runtime_objects);
                    return Ok(RuntimeObject::Container(container))
                }
            }
        }

        Ok(RuntimeObject::Container(Container::from_runtime_object_vec(runtime_objects)))
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: Error
    {
        Ok(RuntimeObject::Null)
    }
}

impl<'de> Deserialize<'de> for RuntimeObject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        // Instantiate our Visitor and ask the Deserializer to drive
        // it over the input data, resulting in an instance of RuntimeObject.
        deserializer.deserialize_map(RuntimeObjectVisitor::new())
    }
}

#[derive(Deserialize)]
struct InkJSon {
    #[serde(rename = "inkVersion")]
    ink_version: u32,
    root: RuntimeObject,
    #[serde(rename = "listDefs")]
    list_defs: Option<BTreeMap<String,String>> // FIXME: listDefs is not a map<string,string>
}

impl InkJSon {
    pub fn from_str(s: &str) -> Result<InkJSon, InkError>
    {
        serde_json::from_str(s).map_err(|e| InkError::from(e))
    }

    pub fn from_slice(v: &[u8]) -> Result<InkJSon, InkError>
    {
        serde_json::from_slice(v).map_err(|e| InkError::from(e))
    }

    pub fn from_reader<R>(rdr: R) -> Result<InkJSon, InkError>
        where
            R: Read
    {
        serde_json::from_reader(rdr).map_err(|e| InkError::from(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_int_test() {
        let json = "[42]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::Int(int_value) => assert_eq!(int_value, 42),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn value_float_test() {
        let json = "[3.14159265359]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::Float(float_value) => assert_eq!(float_value, 3.14159265359),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn value_string_test() {
        let json = "[\"^I looked at Monsieur Fogg\"]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::String(ref string_value) => assert_eq!(string_value, "I looked at Monsieur Fogg"),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn value_divert_target_test() {
        let json = "{\"^->\":\"0.g-0.2.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Value(ref value) => match value {
                &Value::DivertTarget(ref path) => assert_eq!(path.to_string(), "0.g-0.2.$r1"),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn value_variable_pointer_test() {
        let json = "{\"^var\": \"varname\", \"ci\": 0}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Value(value) => match value {
                Value::VariablePointer(name, context_index) => {
                    assert_eq!(name, "varname");
                    assert_eq!(context_index, 0);
                },
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn newline_test() {
        let json = "[\"\\n\"]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        match runtime_objects.get(0).unwrap() {
            &RuntimeObject::Value(ref value) => match value {
                &Value::String(ref string_value) => assert_eq!(string_value, "\n"),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn glue_test() {
        let json = "[\"<>\", \"G<\", \"G>\"]";
        let glues: Vec<Glue> = vec![Glue::Bidirectional, Glue::Left, Glue::Right];

        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        assert_eq!(glues.len(), runtime_objects.len());

        for (i, runtime_object) in runtime_objects.iter().enumerate() {
            let glue = glues.get(i).unwrap();

            match runtime_object {
                &RuntimeObject::Glue(ref value) => assert_eq!(value, glue),
                _ => assert!(false)
            }
        }
    }

    #[test]
    fn control_command_test() {
        let json = "[\"ev\", \"out\", \"/ev\", \"du\", \"pop\", \"~ret\", \"->->\", \"str\", \"/str\", \"nop\", \"choiceCnt\", \"turns\", \"readc\", \"rnd\", \"srnd\", \"visit\", \"seq\", \"thread\", \"done\", \"end\", \"listInt\", \"range\"]";
        let control_commands: Vec<ControlCommand> = vec![ControlCommand::EvalStart, ControlCommand::EvalOutput, ControlCommand::EvalEnd, ControlCommand::Duplicate,
            ControlCommand::PopEvaluatedValue, ControlCommand::PopFunction, ControlCommand::PopTunnel, ControlCommand::BeginString, ControlCommand::EndString,
            ControlCommand::NoOp, ControlCommand::ChoiceCount, ControlCommand::TurnsSince, ControlCommand::ReadCount, ControlCommand::Random, ControlCommand::SeedRandom,
            ControlCommand::VisitIndex, ControlCommand::SequenceShuffleIndex, ControlCommand::StartThread, ControlCommand::Done, ControlCommand::End,
            ControlCommand::ListFromInt, ControlCommand::ListRange];

        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        assert_eq!(control_commands.len(), runtime_objects.len());

        for (i, runtime_object) in runtime_objects.iter().enumerate() {
            let control_command = control_commands.get(i).unwrap();

            match runtime_object {
                &RuntimeObject::ControlCommand(ref value) => assert_eq!(value, control_command),
                _ => assert!(false)
            }
        }
    }

    #[test]
    fn void_test() {
        let json = "[\"void\"]";
        let runtime_objects: Vec<RuntimeObject> = serde_json::from_str(json).unwrap();
        // TODO: impl PartialEq for RuntimeObject
        //assert_eq!(runtime_objects.get(0).unwrap(), RuntimeObject::Void);
    }

    #[test]
    fn divert_test() {
        let json = "{\"->\": \".^.s\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target().unwrap() {
                    &TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), ".^.s");
                    },
                    _ => assert!(false)
                }

                assert_eq!(divert.stack_push_type(), &PushPopType::None);
                assert_eq!(divert.pushes_to_stack(), false);
                assert_eq!(divert.is_conditional(), false);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn divert_conditional_test() {
        let json = "{\"->\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn divert_with_var_test() {
        let json = "{\"->\":\"$r\",\"var\":true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target().unwrap() {
                    &TargetType::Name(ref target_name) => {
                        assert_eq!(target_name, "$r");
                    },
                    _ => assert!(false)
                }

                assert_eq!(divert.stack_push_type(), &PushPopType::None);
                assert_eq!(divert.pushes_to_stack(), false);
                assert_eq!(divert.is_conditional(), false);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn function_call_test() {
        let json = "{\"f()\": \"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target().unwrap() {
                    &TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), "0.g-0.2.c.12.0.c.11.g-0.2.c.$r2");
                    },
                    _ => assert!(false)
                }

                assert_eq!(divert.stack_push_type(), &PushPopType::Function);
                assert_eq!(divert.pushes_to_stack(), true);
                assert_eq!(divert.is_conditional(), false);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn function_call_conditional_test() {
        let json = "{\"f()\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn tunnel_test() {
        let json = "{\"->t->\": \"0.g-0.2.c.12.0.c.11.g-0.2.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target().unwrap() {
                    &TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), "0.g-0.2.c.12.0.c.11.g-0.2.$r1");
                    },
                    _ => assert!(false)
                }

                assert_eq!(divert.stack_push_type(), &PushPopType::Tunnel);
                assert_eq!(divert.pushes_to_stack(), true);
                assert_eq!(divert.is_conditional(), false);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn tunnel_conditional_test() {
        let json = "{\"->t->\": \".^.s\", \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn external_function_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                match divert.target().unwrap() {
                    &TargetType::Path(ref path) => {
                        assert_eq!(path.to_string(), "0.g-0.3.$r1");
                    },
                    _ => assert!(false)
                }

                assert_eq!(divert.stack_push_type(), &PushPopType::Function);
                assert_eq!(divert.pushes_to_stack(), false);
                assert_eq!(divert.is_conditional(), false);
                assert_eq!(divert.is_external(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn external_function_with_args_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\", \"exArgs\": 5}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.external_args().unwrap(), 5);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn external_function_with_conditional_test() {
        let json = "{\"x()\": \"0.g-0.3.$r1\", \"exArgs\": 5, \"c\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Divert(divert) => {
                assert_eq!(divert.is_conditional(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn choice_test() {
        let json = "{\"*\":\".^.c\",\"flg\":18}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Choice(choice) => {
                assert_eq!(choice.path_on_choice().unwrap().to_string(), ".^.c");
                assert_eq!(choice.has_condition(), false);
                assert_eq!(choice.has_start_content(), true);
                assert_eq!(choice.has_choice_only_content(), false);
                assert_eq!(choice.is_invisible_default(), false);
                assert_eq!(choice.once_only(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn variable_reference_test() {
        let json = "{\"VAR?\": \"danger\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableReference(variable) => {
                assert_eq!(variable.name(), "danger");
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn read_count_test() {
        let json = "{\"CNT?\": \"the_hall.light_switch\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::ReadCount(variable) => {
                assert_eq!(variable.target().to_string(), "the_hall.light_switch");
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn variable_assignment_test() {
        let json = "{\"VAR=\": \"money\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name(), "money");
                assert_eq!(variable.is_new_declaration(), true);
                assert_eq!(variable.is_global(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn variable_assignment_redeclared_test() {
        let json = "{\"VAR=\": \"money\", \"re\": true}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name(), "money");
                assert_eq!(variable.is_new_declaration(), false);
                assert_eq!(variable.is_global(), true);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn temporary_variable_assignment_test() {
        let json = "{\"temp=\": \"x\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::VariableAssignment(variable) => {
                assert_eq!(variable.name(), "x");
                assert_eq!(variable.is_new_declaration(), true);
                assert_eq!(variable.is_global(), false);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn tag_test() {
        let json = "{\"#\": \"This is a tag\"}";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Tag(tag) => {
                assert_eq!(tag.text(), "This is a tag");
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn container_test() {
        let json = "[\"^'Ah\",{\"->\":\"$r\",\"var\":true}, null]";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Container(container) => {
                let content = container.content();
                assert_eq!(content.len(), 2);

                match content.get(0).unwrap() {
                    &RuntimeObject::Value(ref value) => {
                        match value {
                            &Value::String(ref str) => assert_eq!(str, "'Ah"),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }

                match content.get(1).unwrap() {
                    &RuntimeObject::Divert(ref divert) => {
                        match divert.target().unwrap() {
                            &TargetType::Name(ref target_name) => {
                                assert_eq!(target_name, "$r");
                            },
                            _ => assert!(false)
                        }

                        assert_eq!(divert.stack_push_type(), &PushPopType::None);
                        assert_eq!(divert.pushes_to_stack(), false);
                        assert_eq!(divert.is_conditional(), false);
                    },
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn nested_container_test() {
        let json = "[\"^test\",{\"subContainer\":[5,6,null],\"#f\":3,\"#n\":\"container\"}]";
        let runtime_object: RuntimeObject = serde_json::from_str(json).unwrap();
        match runtime_object {
            RuntimeObject::Container(container) => {
                let content = container.content();
                assert_eq!(content.len(), 2);
                assert_eq!(container.name().unwrap(), "container");

                match content.get(0).unwrap() {
                    &RuntimeObject::Value(ref value) => {
                        match value {
                            &Value::String(ref str) => assert_eq!(str, "test"),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }

                match content.get(1).unwrap() {
                    &RuntimeObject::Container(ref sub_container) => {
                        let sub_content = sub_container.content();
                        assert_eq!(content.len(), 2);
                        assert_eq!(sub_container.name().unwrap(), "subContainer");

                        match sub_content.get(0).unwrap() {
                            &RuntimeObject::Value(ref value) => match value {
                                &Value::Int(int_value) => assert_eq!(int_value, 5),
                                _ => assert!(false)
                            },
                            _ => assert!(false)
                        }

                        match sub_content.get(1).unwrap() {
                            &RuntimeObject::Value(ref value) => match value {
                                &Value::Int(int_value) => assert_eq!(int_value, 6),
                                _ => assert!(false)
                            },
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn ink_test() {
        let json = r###"{"inkVersion":17,"root":[[["^I looked at Monsieur Fogg","\n",["ev",{"^->":"0.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... and I could contain myself no longer.",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'What is the purpose of our journey, Monsieur?'","\n","^'A wager,' he replied.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A wager!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ I returned.","\n","\n","^He nodded.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But surely that is foolishness!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A most serious matter then!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^He nodded again.","\n",["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But can we win?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'That is what we will endeavour to find out,' he answered.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A modest wager, I trust?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'Twenty thousand pounds,' he replied, quite flatly.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.","/str","/ev",{"*":".^.c","flg":22},{"s":["^I asked nothing further of him then",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^, and after a final, polite cough, he offered nothing more to me. ","<>","\n","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.'","/str","/ev",{"*":".^.c","flg":22},{"s":["^'Ah",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^,' I replied, uncertain what I thought.","\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^After that, ","<>","\n",{"->":"0.g-1"},null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... but I said nothing",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ and ","<>","\n","\n",{"->":"0.g-1"},{"#f":5}]}],{"#n":"g-0"}],{"g-1":["^we passed the day in silence.","\n",["end",{"#n":"g-2"}],null]}],"done",{"#f":3}],"listDefs":{}}"###;
        let inkObject: InkJSon = InkJSon::from_str(json).unwrap();
        assert_eq!(inkObject.ink_version, 17)
    }

    // FIXME: For now serde MapAccess::next_value() for &str fail when deserializing from a reader
    // FIXME: https://github.com/serde-rs/serde/issues/1009
    /*#[test]
    fn ink_test_from_reader() {
        use std::io::BufReader;
        use std::fs::File;

        let json = r###"{"inkVersion":17,"root":[[["^I looked at Monsieur Fogg","\n",["ev",{"^->":"0.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... and I could contain myself no longer.",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'What is the purpose of our journey, Monsieur?'","\n","^'A wager,' he replied.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A wager!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ I returned.","\n","\n","^He nodded.","\n",[["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But surely that is foolishness!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.0.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A most serious matter then!'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^He nodded again.","\n",["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'But can we win?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.2.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'That is what we will endeavour to find out,' he answered.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^'A modest wager, I trust?'",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"\n","\n","^'Twenty thousand pounds,' he replied, quite flatly.","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.","/str","/ev",{"*":".^.c","flg":22},{"s":["^I asked nothing further of him then",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.0.c.11.g-0.4.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^, and after a final, polite cough, he offered nothing more to me. ","<>","\n","\n",{"->":"0.g-0.2.c.12.g-0"},{"#f":5}]}],null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.2.c.12.1.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","str","^.'","/str","/ev",{"*":".^.c","flg":22},{"s":["^'Ah",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.2.c.12.1.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^,' I replied, uncertain what I thought.","\n","\n",{"->":".^.^.^.g-0"},{"#f":5}]}],{"g-0":["^After that, ","<>","\n",{"->":"0.g-1"},null]}],{"#f":5}]}],["ev",{"^->":"0.g-0.3.$r1"},{"temp=":"$r"},"str",{"->":".^.s"},[{"#n":"$r1"}],"/str","/ev",{"*":".^.c","flg":18},{"s":["^... but I said nothing",{"->":"$r","var":true},null],"c":["ev",{"^->":"0.g-0.3.c.$r2"},"/ev",{"temp=":"$r"},{"->":".^.^.s"},[{"#n":"$r2"}],"^ and ","<>","\n","\n",{"->":"0.g-1"},{"#f":5}]}],{"#n":"g-0"}],{"g-1":["^we passed the day in silence.","\n",["end",{"#n":"g-2"}],null]}],"done",{"#f":3}],"listDefs":{}}"###;
        //let reader = BufReader::new(json.as_bytes());
        let reader = File::open("/home/midgard/dev/rink-runtime/tests/simple4.ink.json").unwrap();
        let inkObject = InkJSon::from_reader(reader).unwrap();
        assert_eq!(inkObject.ink_version, 17)
    }*/
}
