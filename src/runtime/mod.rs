pub mod choice;
pub mod choice_point;
pub mod container;
pub mod control_command;
pub mod divert;
pub mod glue;
pub mod native_function_call;
pub mod story;
pub mod tag;
pub mod value;
pub mod variable;

use std::fmt;

use runtime::choice::Choice;
use runtime::choice_point::ChoicePoint;
use runtime::container::Container;
use runtime::control_command::ControlCommand;
use runtime::divert::Divert;
use runtime::glue::Glue;
use runtime::native_function_call::NativeFunctionCall;
use runtime::story::Story;
use runtime::tag::Tag;
use runtime::value::Value;
use runtime::variable::{VariableAssignment, VariableReference, ReadCount};

use debug_metadata::DebugMetadata;
use path::Path;

pub enum RuntimeObject {
    Choice(Choice),
    ChoicePoint(ChoicePoint),
    Container(Container),
    ControlCommand(ControlCommand),
    Divert(Divert),
    Glue(Glue),
    NativeFunctionCall(NativeFunctionCall),
    Story(Story),
    Tag(Tag),
    Value(Value),
    VariableAssignment(VariableAssignment),
    VariableReference(VariableReference),
    ReadCount(ReadCount),
    Void,
    Null
}

impl fmt::Display for RuntimeObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RuntimeObject::ControlCommand(ref control_command) => write!(f, "{}", control_command.to_string()),
            _ => write!(f, "TODO"),
        }
    }
}

impl RuntimeObject {
    pub fn is_container(&self) -> bool {
        self.as_container().is_some()
    }

    pub fn as_container(&self) -> Option<&Container> {
        match *self {
            RuntimeObject::Container(ref container) => Some(container),
            _ => None,
        }
    }
}

struct RuntimeBaseObject<'a> {
    parent: Option<&'a RuntimeObject>,
    debug_metadata: Option<DebugMetadata>,
    path: Option<Path>
}

impl<'a> RuntimeBaseObject<'a> {
    pub fn new() -> RuntimeBaseObject<'a> {
        RuntimeBaseObject {
            parent: None,
            debug_metadata: None,
            path: None
        }
    }

    pub fn debug_metadata(&self) -> Option<&DebugMetadata> {
        match self.debug_metadata {
            Some(ref debug_metadata) => Some(debug_metadata),
            _ => {
                match self.parent {
                    Some(ref parent) => None, // FIXME: should return parent.debug_metadata()
                    _ => None
                }
            }
        }
    }

    pub fn set_debug_metadata(&mut self, debug_metadata: DebugMetadata) {
        self.debug_metadata = Some(debug_metadata)
    }

    /*pub fn root_content_container(&self) -> Option<&Container> {
        match self.parent {
            Some(ref parent) => {

            },
            _
        }
    }*/
}

trait RuntimeNamedObject {
    fn name(&self) -> Option<&String>;
    fn has_valid_name(&self) -> bool;
}
