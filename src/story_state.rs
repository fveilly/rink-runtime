use callstack::CallStack;
use choice::Choice;
use runtime_graph::RuntimeGraph;
use runtime::RuntimeObject;

use std::rc::Rc;

pub struct StoryState {
    callstack: CallStack,
    choices: Vec<Choice>
}

impl StoryState {
    pub fn new(runtime_graph: &RuntimeGraph) -> StoryState {
        StoryState {
            callstack: CallStack::new(runtime_graph.root_container()),
            choices: Vec::new()
        }
    }

    pub fn callstack(&mut self) -> &mut CallStack {
        &mut self.callstack
    }

    pub fn end_of_story(&self) -> bool {
        return self.callstack.runtime_object().is_none();
    }
}