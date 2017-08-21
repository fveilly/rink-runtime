use callstack::CallStack;
use choice::Choice;
use runtime_graph::RuntimeGraph;

use std::rc::Rc;

pub struct StoryState {
    callstack: CallStack,
    choices: Vec<Choice>
}

impl StoryState {
    pub fn new(runtime_graph: &RuntimeGraph) -> StoryState {
        StoryState {
            callstack: CallStack::new(runtime_graph.root_container().clone()),
            choices: Vec::new()
        }
    }

    pub fn end_of_story(&self) -> bool {
        return self.callstack.current_runtime_object().is_none();
    }
}