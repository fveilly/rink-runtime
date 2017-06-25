use runtime::container::Container;
use runtime::divert::PushPopType;
use runtime::RuntimeObject;

use std::collections::HashMap;

struct Element {
    current_container: Option<Container>,
    current_content_index: usize,
    in_expression_evaluation: bool,
    temporary_variables: HashMap<String, RuntimeObject>,
    stack_push_type: PushPopType
}

impl Element {
    pub fn current_object(&self) -> Option<&RuntimeObject> {
        match self.current_container {
            Some(ref container) => {
                if self.current_content_index < container.content().len() {
                    container.content().get(self.current_content_index)
                }
                else {
                    None
                }
            },
            _ => None
        }
    }

    /*pub fn set_current_object(&mut self, obj: Option<RuntimeObject>) {
        match obj {
            Some(ref object) => {
                match (object) {
                    RuntimeObject::Container(container) => {
                        self.current_container = Some(container);

                    },
                    _ => {

                    }
                }
            },
            _ => {
                self.current_container = None;
                self.current_content_index = 0;
            }
        }
    }*/
}

struct CallStack {

}

struct Thread {

}