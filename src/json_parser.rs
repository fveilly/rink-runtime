use serde_json;

use std::fmt;
use std::collections::BTreeMap;
use std::fs::File;
use std::error::Error;

use runtime::RuntimeObject;
use runtime::value::Value;
use runtime::glue::Glue;
use runtime::control_command::ControlCommand;
use runtime::divert::{Divert, PushPopType, TargetType};
use runtime::choice_point::ChoicePoint;
use path::Path;

use serde::de::Error as SerdeError;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess, SeqAccess};

#[derive(Deserialize)]
#[serde(untagged)]
enum InkDictionaryContent {
    Container(Vec<RuntimeObject>),
    String(String),
    Integer(u32),
    Bool(bool)
}

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
        let entry: Option<(&str, InkDictionaryContent)> = try!(map.next_entry());
        match entry {
            // Divert target value to path
            Some(("^->", value)) => { Err(SerdeError::custom("TODO")) },

            // VariablePointerValue
            Some(("^var", value)) => { Err(SerdeError::custom("TODO")) },

            // Divert
            Some(("->", value)) => {
                match value {
                    InkDictionaryContent::String(target) => {
                        let mut divert = Divert::new();

                        let entry: Option<(&str, bool)> = try!(map.next_entry());
                        match entry {
                            // Case {"->": "variableTarget", "var": true}
                            Some(("var", true)) => {
                                divert.set_target(TargetType::Name(target));

                                // Case {"->": "variableTarget", "var": true, "c": true}
                                if let Some(("c", true)) = try!(map.next_entry()) {
                                    divert.set_is_conditional(true);
                                }
                            },
                            _ => {
                                match Path::parse(&target) {
                                    Some(path) => divert.set_target(TargetType::Path(path)),
                                    _ => return Err(SerdeError::custom("Cannot parse target path"))
                                }

                                // Case {"->": "variableTarget", "c": true}
                                if let Some(("c", true)) = entry {
                                    divert.set_is_conditional(true);
                                }
                            }
                        }
                        Ok(RuntimeObject::Divert(divert))
                    },
                    _  => Err(SerdeError::custom("Unexpected divert target type"))
                }
            },
            // Function Call
            Some(("f()", value)) => {
                match value {
                    InkDictionaryContent::String(target) => {
                        let mut divert = Divert::new_function();

                        match Path::parse(&target) {
                            Some(path) => divert.set_target(TargetType::Path(path)),
                            _ => return Err(SerdeError::custom("Cannot parse target path"))
                        }

                        // Case {"f()": "path.to.func", "c": true}
                        if let Some(("c", true)) = try!(map.next_entry()) {
                            divert.set_is_conditional(true);
                        }

                        Ok(RuntimeObject::Divert(divert))
                    },
                    _  => Err(SerdeError::custom("Unexpected divert target type"))
                }
            },
            // Tunnel
            Some(("->t->", value)) => {
                match value {
                    InkDictionaryContent::String(target) => {
                        let mut divert = Divert::new_tunnel();

                        match Path::parse(&target) {
                            Some(path) => divert.set_target(TargetType::Path(path)),
                            _ => return Err(SerdeError::custom("Cannot parse target path"))
                        }

                        // Case {"->t->": "path.tunnel", "c": true}
                        if let Some(("c", true)) = try!(map.next_entry()) {
                            divert.set_is_conditional(true);
                        }

                        Ok(RuntimeObject::Divert(divert))
                    },
                    _  => Err(SerdeError::custom("Unexpected divert target type"))
                }
            },
            // External function
            Some(("x()", value)) => {
                match value {
                    InkDictionaryContent::String(target) => {
                        let mut divert = Divert::new_external_function();

                        match Path::parse(&target) {
                            Some(path) => divert.set_target(TargetType::Path(path)),
                            _ => return Err(SerdeError::custom("Cannot parse target path"))
                        }

                        // Case {"x()": "externalFuncName", "exArgs": 5}
                        if let Some(("exArgs", external_args)) = try!(map.next_entry()) {
                            divert.set_external_args(external_args);
                        }

                        // Case {"x()": "externalFuncName", "exArgs": 5, "c": true}
                        if let Some(("c", true)) = try!(map.next_entry()) {
                            divert.set_is_conditional(true);
                        }

                        Ok(RuntimeObject::Divert(divert))
                    },
                    _  => Err(SerdeError::custom("Unexpected divert target type"))
                }
            }
            // Choice
            Some(("*", value)) => {
                match value {
                    InkDictionaryContent::String(target) => {
                        let mut choice = ChoicePoint::new();

                        match Path::parse(&target) {
                            Some(path) => choice.set_path_on_choice(path),
                            _ => return Err(SerdeError::custom("Cannot parse choice path"))
                        }

                        if let Some(("flg", flags)) = try!(map.next_entry()) {
                            choice.set_flags(flags);
                        }

                        Ok(RuntimeObject::ChoicePoint(choice))
                    },
                    _  => Err(SerdeError::custom("Unexpected choice path type"))
                }
            },

            // Variable reference
            Some(("VAR?", value)) => { Err(SerdeError::custom("TODO")) },
            Some(("CNT?", value)) => { Err(SerdeError::custom("TODO")) },

            // Variable assignment
            Some(("VAR=", value)) => { Err(SerdeError::custom("TODO")) },
            Some(("temp=", value)) => { Err(SerdeError::custom("TODO")) },

            // List
            Some(("list", value)) => { Err(SerdeError::custom("TODO")) },

            _ => { Err(SerdeError::custom("TODO")) }
        }
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
    root: Vec<RuntimeObject>,
    #[serde(rename = "listDefs")]
    list_defs: Option<BTreeMap<String,String>> // FIXME: listDefs is not a map<string,string>
}


pub fn typed_example() -> Result<(), Box<Error>> {
    let mut file = File::open("/home/midgard/dev/rink-runtime/tests/simple4.ink.json")?;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let json: InkJSon = serde_json::from_reader(file)?;

    // Do things just like with any other Rust data structure.
    println!("Ink version {}", json.ink_version);
    Ok(())
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
            RuntimeObject::ChoicePoint(choice) => {
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
}
